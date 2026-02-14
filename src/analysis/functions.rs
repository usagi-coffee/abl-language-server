use std::collections::HashSet;

use tower_lsp::lsp_types::Url;
use tree_sitter::Node;

use crate::analysis::definitions::{
    collect_global_preprocessor_define_sites, collect_preprocessor_define_sites,
};
use crate::analysis::includes::collect_include_sites;
use crate::analysis::includes::resolve_include_site_path;
use crate::analysis::scopes::containing_scope;
use crate::backend::Backend;
use crate::utils::ts::direct_child_by_kind;

pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: Option<String>,
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
        let params = collect_function_params(node, src);
        let return_type = node
            .child_by_field_name("type")
            .and_then(|n| n.utf8_text(src).ok())
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty());

        out.push(FunctionSignature {
            name: name.to_string(),
            params,
            return_type,
            is_forward: node.kind() == "function_forward_definition",
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_signatures(ch, src, symbol, out);
        }
    }
}

fn collect_function_params(function_node: Node, src: &[u8]) -> Vec<String> {
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let mut header_params = Vec::new();
        collect_params_by_kind(parameters_node, src, "parameter", &mut header_params);
        if !header_params.is_empty() {
            return header_params;
        }
    }

    let mut out = Vec::new();
    collect_params_recursive(function_node, src, &mut out, true);
    out
}

fn collect_params_recursive(node: Node, src: &[u8], out: &mut Vec<String>, is_root: bool) {
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
        && let Some(rendered) = render_param(node, src)
    {
        out.push(rendered);
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_params_recursive(ch, src, out, false);
        }
    }
}

fn collect_params_by_kind(node: Node, src: &[u8], target_kind: &str, out: &mut Vec<String>) {
    if node.kind() == target_kind
        && let Some(rendered) = render_param(node, src)
    {
        out.push(rendered);
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_params_by_kind(ch, src, target_kind, out);
        }
    }
}

fn render_param(node: Node, src: &[u8]) -> Option<String> {
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

    Some(match mode {
        Some(mode) => format!("{mode} {name}: {ty}"),
        None => format!("{name}: {ty}"),
    })
}

fn signature_score(sig: &FunctionSignature) -> (usize, usize, usize) {
    (
        sig.params.len(),
        usize::from(sig.return_type.is_some()),
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

    let include_sites = collect_include_sites(text);
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

#[cfg(test)]
mod tests {
    use super::find_function_signature;

    #[test]
    fn picks_richest_function_signature_and_renders_params() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL FORWARD.

FUNCTION foo RETURNS LOGICAL (INPUT p1 AS CHARACTER, OUTPUT p2 AS INTEGER):
  RETURN TRUE.
END FUNCTION.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let sig = find_function_signature(tree.root_node(), src.as_bytes(), "foo")
            .expect("function signature");
        assert_eq!(sig.name, "foo");
        assert_eq!(sig.return_type.as_deref(), Some("LOGICAL"));
        assert_eq!(sig.params.len(), 2);
        assert!(sig.params[0].contains("INPUT"));
        assert!(sig.params[0].contains("p1"));
        assert!(sig.params[1].contains("OUTPUT"));
        assert!(sig.params[1].contains("p2"));
    }
}
