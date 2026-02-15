use std::collections::HashMap;

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::functions::normalize_function_name;
use crate::utils::ts::{count_nodes_by_kind, direct_child_by_kind, node_to_range};

#[derive(Clone)]
pub struct FunctionCallSite {
    pub display_name: String,
    pub name_upper: String,
    pub arg_count: usize,
    pub range: Range,
}

pub fn collect_function_arities(node: Node<'_>, src: &[u8], out: &mut HashMap<String, Vec<usize>>) {
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

pub fn collect_function_calls(node: Node<'_>, src: &[u8], out: &mut Vec<FunctionCallSite>) {
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
                .map(count_argument_nodes)
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

pub fn append_function_arity_mismatch_diags(
    signatures: &HashMap<String, Vec<usize>>,
    calls: &[FunctionCallSite],
    out: &mut Vec<Diagnostic>,
) {
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

fn function_param_count(function_node: Node<'_>, src: &[u8]) -> usize {
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let count = count_nodes_by_kind(parameters_node, "parameter");
        if count > 0 {
            return count;
        }
    }

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

#[cfg(test)]
mod tests {
    use super::{collect_function_arities, collect_function_calls};
    use crate::analysis::parse_abl;
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

        let tree = parse_abl(src);

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

        let tree = parse_abl(src);

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
