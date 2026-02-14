use std::collections::HashSet;

use dashmap::DashSet;
use tower_lsp::lsp_types::{CompletionItemKind, Diagnostic, DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::builtins::{is_builtin_function_name, is_builtin_variable_name};
use crate::analysis::definitions::collect_definition_symbols;
use crate::analysis::diagnostics::functions::FunctionCallSite;
use crate::analysis::functions::normalize_function_name;
use crate::analysis::local_tables::collect_local_table_definitions;
use crate::backend::Backend;
use crate::utils::ts::{collect_nodes_by_kind, node_to_range};

#[derive(Clone)]
pub struct IdentifierRef {
    pub name_upper: String,
    pub display_name: String,
    pub range: Range,
}

pub fn collect_known_symbols(
    root: Node<'_>,
    src: &[u8],
    known_variables: &mut HashSet<String>,
    known_functions: &mut HashSet<String>,
) {
    let mut symbols = Vec::new();
    collect_definition_symbols(root, src, &mut symbols);
    for sym in symbols {
        let upper = sym.label.trim().to_ascii_uppercase();
        if upper.is_empty() {
            continue;
        }
        match sym.kind {
            CompletionItemKind::FUNCTION
            | CompletionItemKind::METHOD
            | CompletionItemKind::CONSTRUCTOR => {
                known_functions.insert(normalize_function_name(&upper));
            }
            _ => {
                known_variables.insert(upper);
            }
        }
    }
}

pub fn collect_identifier_refs_for_unknown_symbol_diag(
    node: Node<'_>,
    src: &[u8],
    out: &mut Vec<IdentifierRef>,
) {
    match node.kind() {
        "assignment_statement" => {
            if let Some(left) = node.child_by_field_name("left")
                && left.kind() == "identifier"
                && let Ok(name_raw) = left.utf8_text(src)
            {
                let display_name = name_raw.trim().to_string();
                if !display_name.is_empty() {
                    out.push(IdentifierRef {
                        name_upper: display_name.to_ascii_uppercase(),
                        display_name,
                        range: node_to_range(left),
                    });
                }
            }
            if let Some(right) = node.child_by_field_name("right") {
                collect_identifier_refs_from_expression(right, src, out);
            }
        }
        "return_statement" => {
            if let Some(value) = node
                .child_by_field_name("value")
                .or_else(|| node.named_child(0))
            {
                collect_identifier_refs_from_expression(value, src, out);
            }
        }
        "expression_statement" => {
            if let Some(expr) = node.named_child(0) {
                collect_identifier_refs_from_expression(expr, src, out);
            }
        }
        _ => {}
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_identifier_refs_for_unknown_symbol_diag(ch, src, out);
        }
    }
}

fn collect_identifier_refs_from_expression(
    expr: Node<'_>,
    src: &[u8],
    out: &mut Vec<IdentifierRef>,
) {
    match expr.kind() {
        "preprocessor_reference" | "preprocessor_directive" | "macro_concatenated_name" => {
            return;
        }
        "include_file_reference" => {
            return;
        }
        "identifier" => {
            if let Ok(name_raw) = expr.utf8_text(src) {
                let display_name = name_raw.trim().to_string();
                if !display_name.is_empty() {
                    out.push(IdentifierRef {
                        name_upper: display_name.to_ascii_uppercase(),
                        display_name,
                        range: node_to_range(expr),
                    });
                }
            }
            return;
        }
        "qualified_name" | "widget_qualified_name" | "scoped_name" | "object_access" => return,
        "function_call" => {
            if let Some(args) = expr
                .children(&mut expr.walk())
                .find(|n| n.kind() == "arguments")
            {
                for arg in argument_exprs(args) {
                    collect_identifier_refs_from_expression(arg, src, out);
                }
            }
            return;
        }
        "new_expression" => {
            if let Some(args) = expr
                .children(&mut expr.walk())
                .find(|n| n.kind() == "arguments")
            {
                for arg in argument_exprs(args) {
                    collect_identifier_refs_from_expression(arg, src, out);
                }
            }
            return;
        }
        _ => {}
    }

    for i in 0..expr.child_count() {
        if let Some(ch) = expr.child(i as u32) {
            collect_identifier_refs_from_expression(ch, src, out);
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

pub fn collect_local_table_field_symbols(
    backend: &Backend,
    root: Node<'_>,
    src: &[u8],
    known_variables: &mut HashSet<String>,
) {
    let mut defs = Vec::new();
    collect_local_table_definitions(root, src, &mut defs);
    for def in defs {
        for field in def.fields {
            let upper = field.name.trim().to_ascii_uppercase();
            if !upper.is_empty() {
                known_variables.insert(upper);
            }
        }
        if let Some(like_table_upper) = def.like_table_upper
            && let Some(fields) = backend.db_fields_by_table.get(&like_table_upper)
        {
            for field in fields.value().iter() {
                let upper = field.name.trim().to_ascii_uppercase();
                if !upper.is_empty() {
                    known_variables.insert(upper);
                }
            }
        }
    }
}

pub fn collect_active_buffer_like_names(
    root: Node<'_>,
    src: &[u8],
    backend: &Backend,
) -> HashSet<String> {
    let mut out = HashSet::<String>::new();

    let mut buffer_mappings = Vec::new();
    collect_buffer_mappings(root, src, &mut buffer_mappings);
    for mapping in buffer_mappings {
        let alias_upper = mapping.alias.trim().to_ascii_uppercase();
        if !alias_upper.is_empty() {
            out.insert(alias_upper);
        }
        let table_upper = mapping.table.trim().to_ascii_uppercase();
        if !table_upper.is_empty() {
            out.insert(table_upper);
        }
    }

    let mut local_table_defs = Vec::new();
    collect_local_table_definitions(root, src, &mut local_table_defs);
    for def in local_table_defs {
        if !def.name_upper.is_empty() {
            out.insert(def.name_upper);
        }
    }

    let mut identifiers = Vec::<Node>::new();
    collect_nodes_by_kind(root, "identifier", &mut identifiers);
    for ident in identifiers {
        let Ok(name_raw) = ident.utf8_text(src) else {
            continue;
        };
        let name_upper = name_raw.trim().to_ascii_uppercase();
        if name_upper.is_empty() {
            continue;
        }
        if backend.db_tables.contains(&name_upper) {
            out.insert(name_upper);
        }
    }

    out
}

pub fn collect_active_db_table_field_symbols(
    backend: &Backend,
    active_table_like_names: &HashSet<String>,
) -> HashSet<String> {
    let mut out = HashSet::<String>::new();
    for table_like in active_table_like_names {
        let Some(fields) = backend.db_fields_by_table.get(table_like) else {
            continue;
        };
        for field in fields.value().iter() {
            let upper = field.name.trim().to_ascii_uppercase();
            if !upper.is_empty() {
                out.insert(upper);
            }
        }
    }
    out
}

pub fn looks_like_table_field_reference(
    name_upper: &str,
    active_buffers: &HashSet<String>,
) -> bool {
    if name_upper.is_empty() || active_buffers.is_empty() {
        return false;
    }
    for buffer in active_buffers {
        if looks_like_prefixed_field(name_upper, buffer)
            || table_field_prefix_from_table_like_name(buffer)
                .is_some_and(|prefix| looks_like_prefixed_field(name_upper, &prefix))
        {
            return true;
        }
    }
    false
}

fn table_field_prefix_from_table_like_name(name_upper: &str) -> Option<String> {
    let trimmed = name_upper.trim();
    if trimmed.is_empty() {
        return None;
    }
    for sep in ['_', '-'] {
        if let Some(idx) = trimmed.find(sep)
            && idx > 0
        {
            let mut prefix = trimmed[..idx].to_string();
            prefix.push('_');
            return Some(prefix);
        }
    }
    None
}

fn looks_like_prefixed_field(name_upper: &str, prefix_upper: &str) -> bool {
    if !name_upper.starts_with(prefix_upper) || name_upper.len() <= prefix_upper.len() {
        return false;
    }
    let suffix = &name_upper[prefix_upper.len()..];
    let Some(first) = suffix.chars().next() else {
        return false;
    };
    first.is_ascii_alphabetic() || first == '_'
}

pub fn normalize_identifier_refs(refs: &mut Vec<IdentifierRef>) {
    refs.sort_by(|a, b| {
        a.range
            .start
            .line
            .cmp(&b.range.start.line)
            .then(a.range.start.character.cmp(&b.range.start.character))
            .then(a.name_upper.cmp(&b.name_upper))
    });
    refs.dedup_by(|a, b| a.name_upper == b.name_upper && a.range == b.range);
}

pub struct UnknownSymbolDiagInputs<'a> {
    pub refs: &'a [IdentifierRef],
    pub calls: &'a [FunctionCallSite],
    pub known_variables: &'a HashSet<String>,
    pub known_functions: &'a HashSet<String>,
    pub unknown_variables_ignored: &'a HashSet<String>,
    pub unknown_functions_ignored: &'a HashSet<String>,
    pub db_tables: &'a DashSet<String>,
    pub active_table_fields: &'a HashSet<String>,
    pub active_buffer_like_names: &'a HashSet<String>,
    pub unknown_variables_enabled: bool,
    pub unknown_functions_enabled: bool,
}

pub fn append_unknown_symbol_diags(inputs: UnknownSymbolDiagInputs<'_>, out: &mut Vec<Diagnostic>) {
    if inputs.unknown_variables_enabled {
        for r in inputs.refs {
            if inputs.known_variables.contains(&r.name_upper)
                || inputs.unknown_variables_ignored.contains(&r.name_upper)
                || inputs.db_tables.contains(&r.name_upper)
                || inputs.active_table_fields.contains(&r.name_upper)
                || is_builtin_variable_name(&r.name_upper)
                || is_builtin_function_name(&r.name_upper)
                || looks_like_table_field_reference(&r.name_upper, inputs.active_buffer_like_names)
            {
                continue;
            }
            out.push(Diagnostic {
                range: r.range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("abl-semantic".into()),
                message: format!("Unknown variable '{}'", r.display_name),
                ..Default::default()
            });
        }
    }

    if inputs.unknown_functions_enabled {
        for call in inputs.calls {
            if inputs.known_functions.contains(&call.name_upper)
                || inputs.unknown_functions_ignored.contains(&call.name_upper)
                || is_builtin_function_name(&call.name_upper)
                || call.display_name.contains('.')
                || call.display_name.contains(':')
            {
                continue;
            }
            out.push(Diagnostic {
                range: call.range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("abl-semantic".into()),
                message: format!("Unknown function '{}'", call.display_name),
                ..Default::default()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::collect_identifier_refs_for_unknown_symbol_diag;

    #[test]
    fn ignores_preprocessor_references_for_unknown_variable_refs() {
        let src = "OUTPUT TO VALUE({&OUT}) {&OUTPUT-ARGS}.";

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut refs = Vec::new();
        collect_identifier_refs_for_unknown_symbol_diag(
            tree.root_node(),
            src.as_bytes(),
            &mut refs,
        );

        assert!(refs.is_empty());
    }

    #[test]
    fn ignores_new_expression_type_identifier_for_unknown_variable_refs() {
        let src = r#"
DEFINE VARIABLE a AS HANDLE NO-UNDO.
DEFINE VARIABLE x AS INTEGER NO-UNDO.
a = NEW JsonArray(x).
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut refs = Vec::new();
        collect_identifier_refs_for_unknown_symbol_diag(
            tree.root_node(),
            src.as_bytes(),
            &mut refs,
        );

        assert!(refs.iter().all(|r| r.name_upper != "JSONARRAY"));
        assert!(refs.iter().any(|r| r.name_upper == "X"));
    }

    #[test]
    fn ignores_include_file_reference_for_unknown_variable_refs() {
        let src = r#"
{{&US_BBI}gprun.i ""zmzlec2got.p"" "(input zlec,zlec)"}.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut refs = Vec::new();
        collect_identifier_refs_for_unknown_symbol_diag(
            tree.root_node(),
            src.as_bytes(),
            &mut refs,
        );

        assert!(refs.is_empty());
    }
}
