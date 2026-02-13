use dashmap::DashMap;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};
use tree_sitter::Node;

use crate::analysis::functions::FunctionSignature;
use crate::backend::DbFieldInfo;
use crate::utils::ts::{direct_child_by_kind, node_trimmed_text};

#[derive(Clone)]
pub struct DbFieldMatch {
    pub table: String,
    pub field: DbFieldInfo,
}

pub fn symbol_at_offset(root: Node<'_>, text: &str, offset: usize) -> Option<String> {
    let node = root.named_descendant_for_byte_range(offset, offset)?;
    if node.kind() == "identifier" {
        return node_trimmed_text(node, text.as_bytes());
    }

    direct_child_by_kind(node, "identifier").and_then(|n| node_trimmed_text(n, text.as_bytes()))
}

pub fn markdown_hover(markdown: String) -> Hover {
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: markdown,
        }),
        range: None,
    }
}

pub fn function_signature_hover(sig: &FunctionSignature) -> Hover {
    let header = match sig.return_type {
        Some(ref ret) => format!(
            "`FUNCTION {}({}) RETURNS {}`",
            sig.name,
            sig.params.join(", "),
            ret
        ),
        None => format!("`FUNCTION {}({})`", sig.name, sig.params.join(", ")),
    };
    markdown_hover(header)
}

pub fn find_db_field_matches(
    db_fields_by_table: &DashMap<String, Vec<DbFieldInfo>>,
    field_upper: &str,
) -> Vec<DbFieldMatch> {
    let mut out = Vec::new();
    for entry in db_fields_by_table.iter() {
        let table = entry.key();
        let fields = entry.value();
        for field in fields {
            if field.name.eq_ignore_ascii_case(field_upper) {
                out.push(DbFieldMatch {
                    table: table.clone(),
                    field: field.clone(),
                });
            }
        }
    }
    out
}
