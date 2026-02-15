use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionList, CompletionResponse, InsertTextFormat,
};
use tree_sitter::Node;

use crate::analysis::completion::{field_detail, field_documentation};
use crate::analysis::scopes::{ByteScope, containing_scope};
use crate::backend::DbFieldInfo;

pub fn completion_response(items: Vec<CompletionItem>, is_incomplete: bool) -> CompletionResponse {
    if is_incomplete {
        CompletionResponse::List(CompletionList {
            is_incomplete: true,
            items,
        })
    } else {
        CompletionResponse::Array(items)
    }
}

pub fn build_field_completion_items(
    fields: &[DbFieldInfo],
    table_key: &str,
    field_prefix: &str,
) -> Vec<CompletionItem> {
    let pref_up = field_prefix.to_ascii_uppercase();
    let mut items = fields
        .iter()
        .filter(|f| f.name.to_ascii_uppercase().starts_with(&pref_up))
        .map(|f| CompletionItem {
            label: f.name.clone(),
            kind: Some(CompletionItemKind::FIELD),
            detail: Some(field_detail(f, table_key)),
            documentation: field_documentation(f),
            insert_text: Some(f.name.clone()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    items.sort_by(|a, b| {
        a.label
            .to_ascii_uppercase()
            .cmp(&b.label.to_ascii_uppercase())
            .then(a.label.cmp(&b.label))
    });
    items.dedup_by(|a, b| a.label.eq_ignore_ascii_case(&b.label));
    items
}

pub fn is_parameter_symbol_at_byte(root: Node<'_>, start_byte: usize) -> bool {
    let Some(mut node) = root.named_descendant_for_byte_range(start_byte, start_byte) else {
        return false;
    };
    loop {
        if matches!(node.kind(), "parameter" | "parameter_definition") {
            return true;
        }
        let Some(parent) = node.parent() else {
            return false;
        };
        node = parent;
    }
}

pub fn symbol_is_in_current_scope(
    root: Node<'_>,
    symbol_start_byte: usize,
    current_scope: Option<ByteScope>,
) -> bool {
    let Some(current_scope) = current_scope else {
        return false;
    };
    let Some(symbol_scope) = containing_scope(root, symbol_start_byte) else {
        return false;
    };
    symbol_scope.start == current_scope.start && symbol_scope.end == current_scope.end
}

#[cfg(test)]
mod tests {
    use super::{
        build_field_completion_items, completion_response, is_parameter_symbol_at_byte,
        symbol_is_in_current_scope,
    };
    use crate::analysis::scopes::containing_scope;
    use crate::backend::DbFieldInfo;
    use tower_lsp::lsp_types::CompletionResponse;

    #[test]
    fn builds_completion_response_variants() {
        let array = completion_response(Vec::new(), false);
        assert!(matches!(array, CompletionResponse::Array(_)));

        let list = completion_response(Vec::new(), true);
        assert!(matches!(list, CompletionResponse::List(_)));
    }

    #[test]
    fn builds_sorted_deduplicated_field_items() {
        let fields = vec![
            DbFieldInfo {
                name: "name".to_string(),
                field_type: Some("CHARACTER".to_string()),
                format: None,
                label: None,
                description: None,
            },
            DbFieldInfo {
                name: "Name".to_string(),
                field_type: Some("CHARACTER".to_string()),
                format: None,
                label: None,
                description: None,
            },
            DbFieldInfo {
                name: "number".to_string(),
                field_type: Some("INTEGER".to_string()),
                format: None,
                label: None,
                description: None,
            },
        ];

        let items = build_field_completion_items(&fields, "customer", "na");
        let labels = items.into_iter().map(|i| i.label).collect::<Vec<_>>();
        assert_eq!(labels, vec!["Name".to_string()]);
    }

    #[test]
    fn detects_parameter_symbols_and_scope_membership() {
        let src = r#"
DEFINE VARIABLE outsideVar AS INTEGER NO-UNDO.

FUNCTION foo RETURNS LOGICAL (INPUT p1 AS INTEGER):
  DEFINE VARIABLE insideVar AS INTEGER NO-UNDO.
  insideVar = p1.
  RETURN TRUE.
END FUNCTION.
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let param_offset = src.find("p1 AS").expect("parameter start");
        assert!(is_parameter_symbol_at_byte(tree.root_node(), param_offset));

        let inside_offset = src.find("insideVar =").expect("inside offset");
        let outside_offset = src.find("outsideVar").expect("outside offset");
        let current_scope =
            containing_scope(tree.root_node(), inside_offset).expect("current function scope");

        assert!(symbol_is_in_current_scope(
            tree.root_node(),
            inside_offset,
            Some(current_scope)
        ));
        assert!(!symbol_is_in_current_scope(
            tree.root_node(),
            outside_offset,
            Some(current_scope)
        ));
    }
}
