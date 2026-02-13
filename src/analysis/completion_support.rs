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
