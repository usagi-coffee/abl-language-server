use std::collections::HashMap;

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};
use tree_sitter::Node;

use crate::analysis::functions::normalize_function_name;
use crate::analysis::types::{BasicType, builtin_type_from_name};
use crate::utils::ts::{direct_child_by_kind, node_to_range};

struct TypedBinding {
    name_upper: String,
    ty: BasicType,
    start_byte: usize,
}

#[derive(Clone)]
struct FunctionTypeSignature {
    param_types: Vec<Option<BasicType>>,
}

pub fn collect_assignment_type_diags(root: Node<'_>, src: &[u8], out: &mut Vec<Diagnostic>) {
    let mut bindings = Vec::<TypedBinding>::new();
    collect_typed_bindings(root, src, &mut bindings);

    if bindings.is_empty() {
        return;
    }

    let mut function_returns = HashMap::<String, BasicType>::new();
    collect_function_return_types(root, src, &mut function_returns);

    collect_assignment_type_diags_in_node(root, src, &bindings, &function_returns, out);
}

pub fn collect_function_call_arg_type_diags(root: Node<'_>, src: &[u8], out: &mut Vec<Diagnostic>) {
    let mut bindings = Vec::<TypedBinding>::new();
    collect_typed_bindings(root, src, &mut bindings);

    let mut function_returns = HashMap::<String, BasicType>::new();
    collect_function_return_types(root, src, &mut function_returns);

    let mut signatures = HashMap::<String, Vec<FunctionTypeSignature>>::new();
    collect_function_type_signatures(root, src, &mut signatures);

    collect_function_call_arg_type_diags_in_node(
        root,
        src,
        &bindings,
        &function_returns,
        &signatures,
        out,
    );
}

fn collect_typed_bindings(node: Node<'_>, src: &[u8], out: &mut Vec<TypedBinding>) {
    if matches!(node.kind(), "variable_definition" | "parameter_definition")
        && let (Some(name_node), Some(type_node)) = (
            node.child_by_field_name("name"),
            node.child_by_field_name("type"),
        )
        && let (Ok(name), Ok(raw_ty)) = (name_node.utf8_text(src), type_node.utf8_text(src))
        && let Some(ty) = builtin_type_from_name(raw_ty)
    {
        out.push(TypedBinding {
            name_upper: name.trim().to_ascii_uppercase(),
            ty,
            start_byte: name_node.start_byte(),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_typed_bindings(ch, src, out);
        }
    }
}

fn collect_function_return_types(node: Node<'_>, src: &[u8], out: &mut HashMap<String, BasicType>) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) && let (Some(name_node), Some(type_node)) = (
        node.child_by_field_name("name"),
        node.child_by_field_name("type"),
    ) && let (Ok(name), Ok(raw_ty)) = (name_node.utf8_text(src), type_node.utf8_text(src))
        && let Some(ty) = builtin_type_from_name(raw_ty)
    {
        out.insert(normalize_function_name(name), ty);
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_return_types(ch, src, out);
        }
    }
}

fn collect_function_type_signatures(
    node: Node<'_>,
    src: &[u8],
    out: &mut HashMap<String, Vec<FunctionTypeSignature>>,
) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) && let Some(name_node) = node.child_by_field_name("name")
        && let Ok(name) = name_node.utf8_text(src)
    {
        let param_types = function_param_types(node, src);
        out.entry(normalize_function_name(name))
            .or_default()
            .push(FunctionTypeSignature { param_types });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_type_signatures(ch, src, out);
        }
    }
}

fn collect_assignment_type_diags_in_node(
    node: Node<'_>,
    src: &[u8],
    bindings: &[TypedBinding],
    function_returns: &HashMap<String, BasicType>,
    out: &mut Vec<Diagnostic>,
) {
    if node.kind() == "assignment_statement"
        && let (Some(left), Some(right)) = (
            node.child_by_field_name("left"),
            node.child_by_field_name("right"),
        )
        && left.kind() == "identifier"
        && let Ok(name_raw) = left.utf8_text(src)
    {
        let left_name_upper = name_raw.trim().to_ascii_uppercase();
        if let Some(left_ty) = resolve_binding_type(bindings, &left_name_upper, left.start_byte())
            && let Some(right_ty) = infer_expr_type(right, src, bindings, function_returns)
            && left_ty != right_ty
        {
            out.push(Diagnostic {
                range: node_to_range(right),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("abl-semantic".into()),
                message: format!(
                    "Type mismatch: cannot assign {} to {} variable '{}'",
                    right_ty.label(),
                    left_ty.label(),
                    left_name_upper
                ),
                ..Default::default()
            });
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_assignment_type_diags_in_node(ch, src, bindings, function_returns, out);
        }
    }
}

fn resolve_binding_type(
    bindings: &[TypedBinding],
    name_upper: &str,
    at_byte: usize,
) -> Option<BasicType> {
    bindings
        .iter()
        .filter(|b| b.name_upper == name_upper && b.start_byte <= at_byte)
        .max_by_key(|b| b.start_byte)
        .map(|b| b.ty)
}

fn infer_expr_type(
    expr: Node<'_>,
    src: &[u8],
    bindings: &[TypedBinding],
    function_returns: &HashMap<String, BasicType>,
) -> Option<BasicType> {
    match expr.kind() {
        "string_literal" => Some(BasicType::Character),
        "number_literal" => Some(BasicType::Numeric),
        "boolean_literal" => Some(BasicType::Logical),
        "identifier" => expr
            .utf8_text(src)
            .ok()
            .map(|s| s.trim().to_ascii_uppercase())
            .and_then(|name| resolve_binding_type(bindings, &name, expr.start_byte())),
        "parenthesized_expression" => expr
            .named_child(0)
            .and_then(|inner| infer_expr_type(inner, src, bindings, function_returns)),
        "function_call" => {
            let function_name = expr
                .child_by_field_name("function")
                .and_then(|n| n.utf8_text(src).ok())
                .map(normalize_function_name)?;
            function_returns.get(&function_name).copied()
        }
        _ => None,
    }
}

fn collect_function_call_arg_type_diags_in_node(
    node: Node<'_>,
    src: &[u8],
    bindings: &[TypedBinding],
    function_returns: &HashMap<String, BasicType>,
    signatures: &HashMap<String, Vec<FunctionTypeSignature>>,
    out: &mut Vec<Diagnostic>,
) {
    if node.kind() == "function_call" {
        let function_name = node
            .child_by_field_name("function")
            .and_then(|n| n.utf8_text(src).ok())
            .map(normalize_function_name);
        let args = node
            .children(&mut node.walk())
            .find(|n| n.kind() == "arguments")
            .map(argument_exprs)
            .unwrap_or_default();

        if let Some(function_name) = function_name
            && let Some(all_signatures) = signatures.get(&function_name)
        {
            let matching_arity = all_signatures
                .iter()
                .filter(|sig| sig.param_types.len() == args.len())
                .collect::<Vec<_>>();

            if !matching_arity.is_empty() {
                for (idx, arg_expr) in args.into_iter().enumerate() {
                    let expected = unify_expected_param_type(&matching_arity, idx);
                    let actual = infer_expr_type(arg_expr, src, bindings, function_returns);
                    if let (Some(expected), Some(actual)) = (expected, actual)
                        && expected != actual
                    {
                        out.push(Diagnostic {
                            range: node_to_range(arg_expr),
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("abl-semantic".into()),
                            message: format!(
                                "Function '{}' argument {} expects {}, got {}",
                                function_name,
                                idx + 1,
                                expected.label(),
                                actual.label()
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_call_arg_type_diags_in_node(
                ch,
                src,
                bindings,
                function_returns,
                signatures,
                out,
            );
        }
    }
}

fn unify_expected_param_type(
    signatures: &[&FunctionTypeSignature],
    index: usize,
) -> Option<BasicType> {
    let mut expected = None;
    for sig in signatures {
        let ty = sig.param_types.get(index).copied().flatten()?;
        match expected {
            None => expected = Some(ty),
            Some(prev) if prev == ty => {}
            Some(_) => return None,
        }
    }
    expected
}

fn function_param_types(function_node: Node<'_>, src: &[u8]) -> Vec<Option<BasicType>> {
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let mut header_param_types = Vec::new();
        collect_param_types_by_kind(parameters_node, src, "parameter", &mut header_param_types);
        if !header_param_types.is_empty() {
            return header_param_types;
        }
    }

    let mut out = Vec::new();
    collect_param_types_recursive(function_node, src, &mut out, true);
    out
}

fn collect_param_types_by_kind(
    node: Node<'_>,
    src: &[u8],
    target_kind: &str,
    out: &mut Vec<Option<BasicType>>,
) {
    if node.kind() == target_kind {
        out.push(
            node.child_by_field_name("type")
                .and_then(|n| n.utf8_text(src).ok())
                .and_then(builtin_type_from_name),
        );
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_param_types_by_kind(ch, src, target_kind, out);
        }
    }
}

fn collect_param_types_recursive(
    node: Node<'_>,
    src: &[u8],
    out: &mut Vec<Option<BasicType>>,
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

    if node.kind() == "parameter_definition" {
        out.push(
            node.child_by_field_name("type")
                .and_then(|n| n.utf8_text(src).ok())
                .and_then(builtin_type_from_name),
        );
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_param_types_recursive(ch, src, out, false);
        }
    }
}

fn argument_exprs(arguments_node: Node<'_>) -> Vec<Node<'_>> {
    let mut out = Vec::new();
    for i in 0..arguments_node.child_count() {
        let Some(ch) = arguments_node.child(i as u32) else {
            continue;
        };
        if ch.kind() != "argument" {
            continue;
        }
        if let Some(arg_expr) = ch.child_by_field_name("name").or_else(|| ch.named_child(0)) {
            out.push(arg_expr);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{collect_assignment_type_diags, collect_function_call_arg_type_diags};

    #[test]
    fn reports_assignment_type_mismatches_for_variables_and_function_returns() {
        let src = r#"
FUNCTION ret_int RETURNS INTEGER ():
  RETURN 1.
END FUNCTION.

DEFINE VARIABLE c AS CHARACTER NO-UNDO.
DEFINE VARIABLE i AS INTEGER NO-UNDO.
DEFINE VARIABLE okc AS CHARACTER NO-UNDO.

c = i.
i = c.
c = ret_int().
okc = "abc".
i = 42.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut diags = Vec::new();
        collect_assignment_type_diags(tree.root_node(), src.as_bytes(), &mut diags);

        assert_eq!(diags.len(), 3);
        let messages = diags.into_iter().map(|d| d.message).collect::<Vec<_>>();
        assert!(
            messages
                .iter()
                .any(|m| m.contains("cannot assign NUMERIC to CHARACTER variable 'C'"))
        );
        assert!(
            messages
                .iter()
                .any(|m| m.contains("cannot assign CHARACTER to NUMERIC variable 'I'"))
        );
        assert!(
            messages
                .iter()
                .any(|m| m.contains("cannot assign NUMERIC to CHARACTER variable 'C'"))
        );
    }

    #[test]
    fn reports_function_argument_type_mismatches() {
        let src = r#"
FUNCTION local_mul RETURNS INTEGER (INPUT a AS INTEGER, INPUT b AS INTEGER):
  RETURN a * b.
END FUNCTION.

local_mul("5", 1).
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut diags = Vec::new();
        collect_function_call_arg_type_diags(tree.root_node(), src.as_bytes(), &mut diags);

        assert_eq!(diags.len(), 1);
        assert!(
            diags[0]
                .message
                .contains("Function 'LOCAL_MUL' argument 1 expects NUMERIC, got CHARACTER")
        );
    }
}
