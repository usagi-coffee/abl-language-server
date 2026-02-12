use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::functions::normalize_function_name;
use crate::analysis::includes::collect_include_sites;
use crate::backend::Backend;
use crate::utils::ts::{count_nodes_by_kind, direct_child_by_kind, node_to_range};

pub async fn on_change(
    backend: &Backend,
    uri: Url,
    version: i32,
    text: String,
    include_semantic_diags: bool,
) {
    if !should_accept_version(backend, &uri, version) {
        return;
    }

    backend.doc_versions.insert(uri.clone(), version);
    backend.docs.insert(uri.clone(), text.to_owned());

    if !is_latest_version(backend, &uri, version) {
        return;
    }

    let diagnostics_enabled = backend.config.lock().await.diagnostics.enabled;

    let parsed_tree = {
        let parser_mutex = backend
            .abl_parsers
            .entry(uri.clone())
            .or_insert_with(|| std::sync::Mutex::new(backend.new_abl_parser()));
        let mut parser = parser_mutex.lock().expect("ABL parser mutex poisoned");
        if !is_latest_version(backend, &uri, version) {
            return;
        }
        parser.parse(text.clone(), None)
    };
    let tree = match parsed_tree {
        Some(t) => t,
        None => {
            if !is_latest_version(backend, &uri, version) {
                return;
            }
            backend
                .client
                .publish_diagnostics(uri.clone(), vec![], Some(version))
                .await;
            return;
        }
    };

    if !is_latest_version(backend, &uri, version) {
        return;
    }

    if !diagnostics_enabled {
        backend
            .client
            .publish_diagnostics(uri.clone(), vec![], Some(version))
            .await;
        if !is_latest_version(backend, &uri, version) {
            return;
        }
        backend.trees.insert(uri, tree);
        return;
    }

    let mut diags: Vec<Diagnostic> = Vec::new();
    collect_ts_error_diags(tree.root_node(), &mut diags);
    if include_semantic_diags
        && !collect_function_call_arity_diags(
            backend,
            &uri,
            version,
            &text,
            tree.root_node(),
            &mut diags,
        )
        .await
    {
        return;
    }
    if !is_latest_version(backend, &uri, version) {
        return;
    }
    backend
        .client
        .publish_diagnostics(uri.clone(), diags, Some(version))
        .await;

    if !is_latest_version(backend, &uri, version) {
        return;
    }
    backend.trees.insert(uri, tree);
}

async fn collect_function_call_arity_diags(
    backend: &Backend,
    uri: &Url,
    version: i32,
    text: &str,
    root: Node<'_>,
    out: &mut Vec<Diagnostic>,
) -> bool {
    if !is_latest_version(backend, uri, version) {
        return false;
    }

    let mut signatures = HashMap::<String, Vec<usize>>::new();
    collect_function_arities(root, text.as_bytes(), &mut signatures);

    // Include signatures from directly included files.
    if let Ok(current_path) = uri.to_file_path() {
        let include_sites = collect_include_sites(text);
        let mut seen = HashSet::<PathBuf>::new();
        let mut include_parser = backend.new_abl_parser();
        for include in include_sites {
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            let Some(path) = backend
                .resolve_include_path_for(&current_path, &include.path)
                .await
            else {
                continue;
            };
            if !seen.insert(path.clone()) {
                continue;
            }

            let Ok(include_text) = tokio::fs::read_to_string(&path).await else {
                continue;
            };
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            let include_tree = include_parser.parse(&include_text, None);
            let Some(include_tree) = include_tree else {
                continue;
            };
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            collect_function_arities(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut signatures,
            );
        }
    }

    if !is_latest_version(backend, uri, version) {
        return false;
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

    true
}

fn should_accept_version(backend: &Backend, uri: &Url, version: i32) -> bool {
    match backend.doc_versions.get(uri) {
        Some(current) => *current <= version,
        None => true,
    }
}

fn is_latest_version(backend: &Backend, uri: &Url, version: i32) -> bool {
    matches!(backend.doc_versions.get(uri), Some(current) if *current == version)
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
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let count = count_nodes_by_kind(parameters_node, "parameter");
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
                range: node_to_range(target_node),
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
    for i in 0..arguments_node.child_count() {
        if let Some(ch) = arguments_node.child(i as u32)
            && ch.kind() == "argument"
        {
            count += 1;
        }
    }
    count
}

struct FunctionCallSite {
    display_name: String,
    name_upper: String,
    arg_count: usize,
    range: Range,
}

fn collect_ts_error_diags(node: Node, out: &mut Vec<Diagnostic>) {
    if node.is_error() || node.is_missing() {
        out.push(Diagnostic {
            range: node_to_range(node),
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

#[cfg(test)]
mod tests {
    use super::{collect_function_arities, collect_function_calls};
    use std::collections::HashMap;

    #[test]
    fn extracts_function_arities_and_call_arg_counts() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL (INPUT p1 AS CHARACTER, OUTPUT p2 AS INTEGER):
  RETURN TRUE.
END FUNCTION.

DEFINE VARIABLE x AS LOGICAL NO-UNDO.
x = foo("a", 1).
x = foo().
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut signatures = HashMap::<String, Vec<usize>>::new();
        collect_function_arities(tree.root_node(), src.as_bytes(), &mut signatures);
        assert_eq!(signatures.get("FOO").cloned(), Some(vec![2]));

        let mut calls = Vec::new();
        collect_function_calls(tree.root_node(), src.as_bytes(), &mut calls);
        let foo_calls = calls
            .into_iter()
            .filter(|c| c.name_upper == "FOO")
            .map(|c| c.arg_count)
            .collect::<Vec<_>>();
        assert_eq!(foo_calls, vec![2, 0]);
    }

    #[test]
    fn counts_nested_function_call_as_single_argument() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL (INPUT p1 AS INTEGER):
  RETURN TRUE.
END FUNCTION.

DEFINE VARIABLE y AS LOGICAL NO-UNDO.
DEFINE VARIABLE pzd_linia AS CHARACTER NO-UNDO.
y = foo(INTEGER(pzd_linia)).
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut calls = Vec::new();
        collect_function_calls(tree.root_node(), src.as_bytes(), &mut calls);
        let foo_calls = calls
            .into_iter()
            .filter(|c| c.name_upper == "FOO")
            .map(|c| c.arg_count)
            .collect::<Vec<_>>();
        assert_eq!(foo_calls, vec![1]);
    }
}
