use std::collections::{HashMap, HashSet};

use tower_lsp::lsp_types::Url;
use tree_sitter::Node;

use crate::analysis::definitions::{
    collect_global_preprocessor_define_sites, collect_preprocessor_define_sites,
};
use crate::analysis::includes::collect_include_sites_from_tree;
use crate::analysis::includes::resolve_include_site_path;
use crate::analysis::scopes::containing_scope;
use crate::backend::Backend;
use crate::utils::ts::direct_child_by_kind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionParameter {
    pub name: String,
    pub label: String,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FunctionDocumentation {
    pub description: Option<String>,
    pub returns: Option<String>,
    pub param_docs: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<String>,
    pub documentation: FunctionDocumentation,
    is_forward: bool,
}

pub fn find_function_signature(root: Node, src: &[u8], symbol: &str) -> Option<FunctionSignature> {
    let mut matches = Vec::new();
    collect_function_signatures(root, src, symbol, &mut matches);
    matches.into_iter().max_by_key(signature_score)
}

fn collect_function_signatures(
    node: Node,
    src: &[u8],
    symbol: &str,
    out: &mut Vec<FunctionSignature>,
) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) && let Some(name_node) = node.child_by_field_name("name")
        && let Ok(name) = name_node.utf8_text(src)
        && name.eq_ignore_ascii_case(symbol)
    {
        let mut documentation = function_doc_comment(node, src).unwrap_or_default();
        let parameters = collect_function_params(node, src, &mut documentation);
        let return_type = node
            .child_by_field_name("type")
            .and_then(|n| n.utf8_text(src).ok())
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty());

        out.push(FunctionSignature {
            name: name.to_string(),
            parameters,
            return_type,
            documentation,
            is_forward: node.kind() == "function_forward_definition",
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_signatures(ch, src, symbol, out);
        }
    }
}

fn collect_function_params(
    function_node: Node,
    src: &[u8],
    docs: &mut FunctionDocumentation,
) -> Vec<FunctionParameter> {
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let mut header_params = Vec::new();
        collect_params_by_kind(parameters_node, src, "parameter", docs, &mut header_params);
        if !header_params.is_empty() {
            return header_params;
        }
    }

    let mut out = Vec::new();
    collect_params_recursive(function_node, src, docs, &mut out, true);
    out
}

fn collect_params_recursive(
    node: Node,
    src: &[u8],
    docs: &mut FunctionDocumentation,
    out: &mut Vec<FunctionParameter>,
    is_root: bool,
) {
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

    if matches!(node.kind(), "parameter" | "parameter_definition")
        && let Some(rendered) = render_param(node, src, docs)
    {
        out.push(rendered);
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_params_recursive(ch, src, docs, out, false);
        }
    }
}

fn collect_params_by_kind(
    node: Node,
    src: &[u8],
    target_kind: &str,
    docs: &mut FunctionDocumentation,
    out: &mut Vec<FunctionParameter>,
) {
    if node.kind() == target_kind
        && let Some(rendered) = render_param(node, src, docs)
    {
        out.push(rendered);
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_params_by_kind(ch, src, target_kind, docs, out);
        }
    }
}

fn render_param(
    node: Node,
    src: &[u8],
    docs: &mut FunctionDocumentation,
) -> Option<FunctionParameter> {
    let name = node
        .child_by_field_name("name")
        .and_then(|n| n.utf8_text(src).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "param".to_string());

    let ty = node
        .child_by_field_name("type")
        .and_then(|n| n.utf8_text(src).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            node.child_by_field_name("table")
                .and_then(|n| n.utf8_text(src).ok())
                .map(|s| format!("TABLE {}", s.trim()))
        })
        .or_else(|| {
            node.child_by_field_name("dataset")
                .and_then(|n| n.utf8_text(src).ok())
                .map(|s| format!("DATASET {}", s.trim()))
        })
        .unwrap_or_else(|| "ANY".to_string());

    let mode = node
        .utf8_text(src)
        .ok()
        .map(|raw| raw.trim().to_ascii_uppercase())
        .and_then(|raw| {
            if raw.starts_with("INPUT-OUTPUT ") {
                Some("INPUT-OUTPUT")
            } else if raw.starts_with("INPUT ") {
                Some("INPUT")
            } else if raw.starts_with("OUTPUT ") {
                Some("OUTPUT")
            } else {
                None
            }
        });

    let label = match mode {
        Some(mode) => format!("{mode} {name}: {ty}"),
        None => format!("{name}: {ty}"),
    };

    let documentation = docs.take_param_doc(&name);
    Some(FunctionParameter {
        name,
        label,
        documentation,
    })
}

fn signature_score(sig: &FunctionSignature) -> (usize, usize, usize, usize) {
    (
        sig.parameters.len(),
        usize::from(sig.return_type.is_some()),
        usize::from(sig.documentation.description.is_some() || sig.documentation.returns.is_some()),
        usize::from(!sig.is_forward),
    )
}

pub fn normalize_function_name(name: &str) -> String {
    name.split(|c: char| c == '.' || c == ':' || c.is_whitespace())
        .next_back()
        .unwrap_or(name)
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
        .to_ascii_uppercase()
}

pub async fn find_function_signature_from_includes(
    backend: &Backend,
    uri: &Url,
    text: &str,
    root: Node<'_>,
    offset: usize,
    symbol: &str,
) -> Option<FunctionSignature> {
    let scope = containing_scope(root, offset)?;
    let current_path = uri.to_file_path().ok()?;

    let include_sites = collect_include_sites_from_tree(root, text.as_bytes());
    let mut available_define_sites = Vec::new();
    collect_preprocessor_define_sites(root, text.as_bytes(), &mut available_define_sites);
    let mut seen_files = HashSet::new();

    for include in include_sites {
        if include.start_offset < scope.start || include.start_offset > scope.end {
            continue;
        }
        let include_path_value = resolve_include_site_path(&include, &available_define_sites);
        let Some(include_path) = backend
            .resolve_include_path_for(&current_path, &include_path_value)
            .await
        else {
            continue;
        };
        if !seen_files.insert(include_path.clone()) {
            continue;
        }
        let Some((include_text, include_tree)) =
            backend.get_cached_include_parse(&include_path).await
        else {
            continue;
        };
        if let Some(sig) =
            find_function_signature(include_tree.root_node(), include_text.as_bytes(), symbol)
        {
            return Some(sig);
        }
        let mut include_global_defines = Vec::new();
        collect_global_preprocessor_define_sites(
            include_tree.root_node(),
            include_text.as_bytes(),
            &mut include_global_defines,
        );
        for mut define in include_global_defines {
            define.start_byte = include.start_offset;
            available_define_sites.push(define);
        }
    }

    None
}

pub async fn find_function_signature_from_ambient(
    backend: &Backend,
    symbol: &str,
) -> Option<FunctionSignature> {
    for ambient_path in backend.ambient_paths().await {
        let Some((ambient_text, ambient_tree)) =
            backend.get_cached_include_parse(&ambient_path).await
        else {
            continue;
        };
        if let Some(sig) =
            find_function_signature(ambient_tree.root_node(), ambient_text.as_bytes(), symbol)
        {
            return Some(sig);
        }
    }
    None
}

pub async fn collect_function_arities_from_ambient(
    backend: &Backend,
    out: &mut HashMap<String, Vec<usize>>,
) {
    for ambient_path in backend.ambient_paths().await {
        let Some((ambient_text, ambient_tree)) =
            backend.get_cached_include_parse(&ambient_path).await
        else {
            continue;
        };
        crate::analysis::diagnostics::functions::collect_function_arities(
            ambient_tree.root_node(),
            ambient_text.as_bytes(),
            out,
        );
    }
}

pub async fn collect_known_functions_from_ambient(
    backend: &Backend,
    known_functions: &mut HashSet<String>,
    known_function_signatures: &mut HashMap<String, Vec<usize>>,
) {
    let mut scratch_variables = HashSet::new();
    for ambient_path in backend.ambient_paths().await {
        let Some((ambient_text, ambient_tree)) =
            backend.get_cached_include_parse(&ambient_path).await
        else {
            continue;
        };
        crate::analysis::diagnostics::symbols::collect_known_symbols(
            ambient_tree.root_node(),
            ambient_text.as_bytes(),
            &mut scratch_variables,
            known_functions,
        );
        crate::analysis::diagnostics::functions::collect_function_arities(
            ambient_tree.root_node(),
            ambient_text.as_bytes(),
            known_function_signatures,
        );
    }
}

fn function_doc_comment(node: Node<'_>, src: &[u8]) -> Option<FunctionDocumentation> {
    let comment = node.prev_named_sibling()?;
    if comment.kind() != "comment" {
        return None;
    }
    let raw = comment.utf8_text(src).ok()?;
    if !raw.trim_start().starts_with("/**") {
        return None;
    }
    let between = src.get(comment.end_byte()..node.start_byte())?;
    if !between.iter().all(u8::is_ascii_whitespace) {
        return None;
    }
    Some(parse_doc_comment(raw))
}

fn parse_doc_comment(raw: &str) -> FunctionDocumentation {
    let mut docs = FunctionDocumentation::default();
    let body = raw.trim().trim_start_matches("/**").trim_end_matches("*/");
    let lines = body.lines().map(normalize_doc_line).collect::<Vec<_>>();

    let mut description_lines = Vec::new();
    let mut param_docs = HashMap::<String, Vec<String>>::new();
    let mut returns_lines = Vec::new();
    let mut active_param: Option<String> = None;
    let mut in_returns = false;

    for line in lines {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("@param ") {
            let mut parts = rest.splitn(2, char::is_whitespace);
            let Some(name) = parts.next().map(str::trim).filter(|name| !name.is_empty()) else {
                active_param = None;
                in_returns = false;
                continue;
            };
            let doc = parts.next().map(str::trim).unwrap_or_default();
            let entry = param_docs.entry(name.to_ascii_uppercase()).or_default();
            if !doc.is_empty() {
                entry.push(doc.to_string());
            }
            active_param = Some(name.to_ascii_uppercase());
            in_returns = false;
            continue;
        }

        if let Some(rest) = trimmed
            .strip_prefix("@returns")
            .or_else(|| trimmed.strip_prefix("@return"))
        {
            let doc = rest.trim();
            if !doc.is_empty() {
                returns_lines.push(doc.to_string());
            }
            active_param = None;
            in_returns = true;
            continue;
        }

        if trimmed.starts_with('@') {
            active_param = None;
            in_returns = false;
            continue;
        }

        if let Some(name) = active_param.as_ref() {
            param_docs
                .entry(name.clone())
                .or_default()
                .push(trimmed.to_string());
            continue;
        }

        if in_returns {
            if trimmed.is_empty() {
                in_returns = false;
                description_lines.push(line);
                continue;
            }
            returns_lines.push(trimmed.to_string());
            continue;
        }

        description_lines.push(line);
    }

    docs.description = compact_doc_lines(description_lines);
    docs.returns = compact_doc_lines(returns_lines);
    docs.param_docs = param_docs
        .into_iter()
        .filter_map(|(name, lines)| compact_doc_lines(lines).map(|doc| (name, doc)))
        .collect();
    docs
}

fn normalize_doc_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let trimmed = trimmed
        .strip_prefix('*')
        .map(str::trim_start)
        .unwrap_or(trimmed);
    trimmed.trim_end().to_string()
}

fn compact_doc_lines(lines: Vec<String>) -> Option<String> {
    let mut start = 0usize;
    let mut end = lines.len();
    while start < end && lines[start].trim().is_empty() {
        start += 1;
    }
    while end > start && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    if start >= end {
        return None;
    }
    Some(lines[start..end].join("\n"))
}

impl FunctionDocumentation {
    fn take_param_doc(&mut self, name: &str) -> Option<String> {
        self.param_docs.remove(&name.to_ascii_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{find_function_signature, parse_doc_comment};
    use crate::analysis::parse_abl;

    #[test]
    fn picks_richest_function_signature_and_renders_params() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL FORWARD.

FUNCTION foo RETURNS LOGICAL (INPUT p1 AS CHARACTER, OUTPUT p2 AS INTEGER):
  RETURN TRUE.
END FUNCTION.
"#;

        let tree = parse_abl(src);

        let sig = find_function_signature(tree.root_node(), src.as_bytes(), "foo")
            .expect("function signature");
        assert_eq!(sig.name, "foo");
        assert_eq!(sig.return_type.as_deref(), Some("LOGICAL"));
        assert_eq!(sig.parameters.len(), 2);
        assert!(sig.parameters[0].label.contains("INPUT"));
        assert!(sig.parameters[0].label.contains("p1"));
        assert!(sig.parameters[1].label.contains("OUTPUT"));
        assert!(sig.parameters[1].label.contains("p2"));
    }

    #[test]
    fn parses_jsdoc_like_function_docs() {
        let raw = r#"/**
 * Returns the first position of `target` in `source`.
 *
 * Notes
 * - Case sensitivity follows the operands.
 *
 * @param source-string The string to search in.
 * @param target-string The substring to search for.
 * @returns Returns the found position.
 *
 * ```abl
 * DEFINE VARIABLE pos AS INTEGER NO-UNDO.
 * pos = INDEX("banana", "na").
 * ```
 */"#;

        let docs = parse_doc_comment(raw);
        assert_eq!(
            docs.description.as_deref(),
            Some(
                "Returns the first position of `target` in `source`.\n\nNotes\n- Case sensitivity follows the operands.\n\n\n```abl\nDEFINE VARIABLE pos AS INTEGER NO-UNDO.\npos = INDEX(\"banana\", \"na\").\n```"
            )
        );
        assert_eq!(
            docs.param_docs.get("SOURCE-STRING").map(String::as_str),
            Some("The string to search in.")
        );
        assert_eq!(
            docs.param_docs.get("TARGET-STRING").map(String::as_str),
            Some("The substring to search for.")
        );
        assert_eq!(docs.returns.as_deref(), Some("Returns the found position."));
        assert_eq!(
            docs.description.as_deref(),
            Some(
                "Returns the first position of `target` in `source`.\n\nNotes\n- Case sensitivity follows the operands.\n\n\n```abl\nDEFINE VARIABLE pos AS INTEGER NO-UNDO.\npos = INDEX(\"banana\", \"na\").\n```"
            )
        );
    }

    #[test]
    fn attaches_doc_comments_to_function_signature() {
        let src = r#"
/**
 * Returns a value.
 *
 * @param p_value Value to return.
 * @returns Same value.
 */
FUNCTION foo RETURNS INTEGER (INPUT p_value AS INTEGER) FORWARD.
"#;

        let tree = parse_abl(src);
        let sig = find_function_signature(tree.root_node(), src.as_bytes(), "foo")
            .expect("function signature");

        assert_eq!(
            sig.documentation.description.as_deref(),
            Some("Returns a value.")
        );
        assert_eq!(sig.documentation.returns.as_deref(), Some("Same value."));
        assert_eq!(
            sig.parameters[0].documentation.as_deref(),
            Some("Value to return.")
        );
    }
}
