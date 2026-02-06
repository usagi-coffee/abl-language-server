use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::includes::collect_include_sites;
use crate::backend::Backend;

pub async fn on_change(backend: &Backend, uri: Url, text: String) {
    backend.docs.insert(uri.clone(), text.to_owned());

    let tree = {
        let mut parser = backend.parser.lock().await;
        match parser.parse(text.clone(), None) {
            Some(t) => t,
            None => {
                backend
                    .client
                    .publish_diagnostics(uri.clone(), vec![], None)
                    .await;
                return;
            }
        }
    };

    let mut diags: Vec<Diagnostic> = Vec::new();
    collect_ts_error_diags(tree.root_node(), &mut diags);
    collect_function_call_arity_diags(backend, &uri, &text, tree.root_node(), &mut diags).await;
    backend
        .client
        .publish_diagnostics(uri.clone(), diags, None)
        .await;

    backend.trees.insert(uri, tree);
}

async fn collect_function_call_arity_diags(
    backend: &Backend,
    uri: &Url,
    text: &str,
    root: Node<'_>,
    out: &mut Vec<Diagnostic>,
) {
    let mut signatures = HashMap::<String, Vec<usize>>::new();
    collect_function_arities(root, text.as_bytes(), &mut signatures);

    // Include signatures from directly included files.
    if let Ok(current_path) = uri.to_file_path() {
        let workspace_root = backend.workspace_root.lock().await.clone();
        let include_sites = collect_include_sites(text);
        let mut seen = HashSet::<PathBuf>::new();
        for include in include_sites {
            let Some(path) =
                resolve_include_path(&current_path, workspace_root.as_deref(), &include.path)
            else {
                continue;
            };
            if !seen.insert(path.clone()) {
                continue;
            }

            let Ok(include_text) = tokio::fs::read_to_string(&path).await else {
                continue;
            };
            let include_tree = {
                let mut parser = backend.parser.lock().await;
                parser.parse(&include_text, None)
            };
            let Some(include_tree) = include_tree else {
                continue;
            };
            collect_function_arities(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut signatures,
            );
        }
    }

    for arities in signatures.values_mut() {
        arities.sort_unstable();
        arities.dedup();
    }

    let mut calls = Vec::<FunctionCallSite>::new();
    collect_function_calls(root, text.as_bytes(), &mut calls);
    for call in calls {
        let Some(expected_set) = signatures.get(&call.name_upper) else {
            continue;
        };
        if expected_set.contains(&call.arg_count) {
            continue;
        }

        let expected = expected_set
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" or ");
        out.push(Diagnostic {
            range: call.range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("abl-semantic".into()),
            message: format!(
                "Function '{}' expects {} argument(s), got {}",
                call.display_name, expected, call.arg_count
            ),
            ..Default::default()
        });
    }
}

fn collect_function_arities(node: Node<'_>, src: &[u8], out: &mut HashMap<String, Vec<usize>>) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) {
        let name = node
            .child_by_field_name("name")
            .and_then(|n| n.utf8_text(src).ok())
            .map(normalize_function_name);
        if let Some(name_upper) = name {
            let arity = function_param_count(node, src);
            out.entry(name_upper).or_default().push(arity);
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_arities(ch, src, out);
        }
    }
}

fn function_param_count(function_node: Node<'_>, src: &[u8]) -> usize {
    if let Some(parameters_node) = find_child_by_kind(function_node, "parameters") {
        let mut count = 0usize;
        count_nodes_by_kind(parameters_node, "parameter", &mut count);
        if count > 0 {
            return count;
        }
    }

    // Fallback for alternative grammar forms.
    let mut count = 0usize;
    count_parameter_definitions(function_node, &mut count, true);
    let _ = src;
    count
}

fn count_parameter_definitions(node: Node<'_>, out: &mut usize, is_root: bool) {
    if !is_root
        && matches!(
            node.kind(),
            "function_definition"
                | "function_forward_definition"
                | "procedure_definition"
                | "procedure_forward_definition"
                | "method_definition"
                | "constructor_definition"
                | "destructor_definition"
        )
    {
        return;
    }
    if node.kind() == "parameter_definition" {
        *out += 1;
        return;
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            count_parameter_definitions(ch, out, false);
        }
    }
}

fn collect_function_calls(node: Node<'_>, src: &[u8], out: &mut Vec<FunctionCallSite>) {
    if node.kind() == "function_call" {
        let function_node = node.child_by_field_name("function");
        let display_name = function_node
            .and_then(|n| n.utf8_text(src).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        if let Some(display_name) = display_name {
            let name_upper = normalize_function_name(&display_name);
            let arg_count = node
                .children(&mut node.walk())
                .find(|n| n.kind() == "arguments")
                .map(|args| count_argument_nodes(args))
                .unwrap_or(0);

            let target_node = function_node.unwrap_or(node);
            out.push(FunctionCallSite {
                display_name,
                name_upper,
                arg_count,
                range: Range::new(
                    Position::new(
                        target_node.start_position().row as u32,
                        target_node.start_position().column as u32,
                    ),
                    Position::new(
                        target_node.end_position().row as u32,
                        target_node.end_position().column as u32,
                    ),
                ),
            });
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_calls(ch, src, out);
        }
    }
}

fn count_argument_nodes(arguments_node: Node<'_>) -> usize {
    let mut count = 0usize;
    count_nodes_by_kind(arguments_node, "argument", &mut count);
    count
}

fn count_nodes_by_kind(node: Node<'_>, kind: &str, out: &mut usize) {
    if node.kind() == kind {
        *out += 1;
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            count_nodes_by_kind(ch, kind, out);
        }
    }
}

fn find_child_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32)
            && ch.kind() == kind
        {
            return Some(ch);
        }
    }
    None
}

fn normalize_function_name(name: &str) -> String {
    name.split(|c: char| c == '.' || c == ':' || c.is_whitespace())
        .next_back()
        .unwrap_or(name)
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
        .to_ascii_uppercase()
}

fn resolve_include_path(
    current_file: &Path,
    workspace_root: Option<&Path>,
    include: &str,
) -> Option<PathBuf> {
    let candidate = PathBuf::from(include);
    if candidate.is_absolute() && candidate.exists() {
        return Some(candidate);
    }

    if let Some(current_dir) = current_file.parent() {
        let from_current = current_dir.join(include);
        if from_current.exists() {
            return Some(from_current);
        }
    }

    if let Some(root) = workspace_root {
        let from_root = root.join(include);
        if from_root.exists() {
            return Some(from_root);
        }
    }

    None
}

struct FunctionCallSite {
    display_name: String,
    name_upper: String,
    arg_count: usize,
    range: Range,
}

fn collect_ts_error_diags(node: Node, out: &mut Vec<Diagnostic>) {
    if node.is_error() || node.is_missing() {
        let sp = node.start_position();
        let ep = node.end_position();

        out.push(Diagnostic {
            range: Range::new(
                Position::new(sp.row as u32, sp.column as u32),
                Position::new(ep.row as u32, ep.column as u32),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("tree-sitter".into()),
            message: if node.is_missing() {
                "Missing token".into()
            } else {
                "Syntax error".into()
            },
            ..Default::default()
        });
    }

    // DFS
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_ts_error_diags(ch, out);
        }
    }
}
