use crate::backend::DbFieldInfo;
use tower_lsp::lsp_types::Documentation;
use tree_sitter::Node;

pub fn qualifier_before_dot(text: &str, offset: usize, prefix: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let prefix_len = prefix.len();
    if offset < prefix_len + 1 {
        return None;
    }
    let dot_pos = offset - prefix_len - 1;
    if bytes.get(dot_pos).copied() != Some(b'.') {
        return None;
    }

    let mut start = dot_pos;
    while start > 0 {
        let c = bytes[start - 1];
        let is_ident = c.is_ascii_alphanumeric() || c == b'_' || c == b'-';
        if !is_ident {
            break;
        }
        start -= 1;
    }

    if start == dot_pos {
        return None;
    }
    Some(text[start..dot_pos].to_string())
}

pub fn text_has_dot_before_cursor(text: &str, offset: usize) -> bool {
    if offset == 0 {
        return false;
    }
    text.as_bytes().get(offset - 1).copied() == Some(b'.')
}

pub fn lookup_case_insensitive_fields(
    map: &dashmap::DashMap<String, Vec<DbFieldInfo>>,
    key: &str,
) -> Option<Vec<DbFieldInfo>> {
    map.get(key)
        .map(|fields| fields.value().clone())
        .or_else(|| {
            map.iter().find_map(|entry| {
                if entry.key().eq_ignore_ascii_case(key) {
                    Some(entry.value().clone())
                } else {
                    None
                }
            })
        })
}

pub fn lookup_case_insensitive_indexes_by_table(
    map: &dashmap::DashMap<String, Vec<String>>,
    key: &str,
) -> Option<Vec<String>> {
    map.get(key)
        .map(|indexes| indexes.value().clone())
        .or_else(|| {
            map.iter().find_map(|entry| {
                if entry.key().eq_ignore_ascii_case(key) {
                    Some(entry.value().clone())
                } else {
                    None
                }
            })
        })
}

pub fn use_index_table_symbol_at_offset(
    root: Node<'_>,
    text: &str,
    offset: usize,
) -> Option<String> {
    let src = text.as_bytes();
    let probe = offset.saturating_sub(1).min(src.len().saturating_sub(1));
    let mut node = root.named_descendant_for_byte_range(probe, probe)?;

    loop {
        let in_index = node
            .child_by_field_name("index")
            .map(|index| offset >= index.start_byte() && offset <= index.end_byte())
            .unwrap_or(false);
        if in_index {
            let table_node = node
                .child_by_field_name("table")
                .or_else(|| node.child_by_field_name("record"));
            if let Some(table_node) = table_node
                && let Ok(raw) = table_node.utf8_text(src)
            {
                let symbol = raw
                    .trim()
                    .split('.')
                    .next_back()
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                if !symbol.is_empty() {
                    return Some(symbol);
                }
            }
            return None;
        }

        let Some(parent) = node.parent() else {
            break;
        };
        node = parent;
    }

    None
}

pub fn field_detail(field: &DbFieldInfo, table_key: &str) -> String {
    match field.field_type.as_deref() {
        Some(ty) => format!("{ty} ({table_key})"),
        None => format!("FIELD ({table_key})"),
    }
}

pub fn field_documentation(field: &DbFieldInfo) -> Option<Documentation> {
    let mut lines = Vec::new();
    if let Some(label) = &field.label
        && !label.is_empty()
    {
        lines.push(format!("Label: {label}"));
    }
    if let Some(format) = &field.format
        && !format.is_empty()
    {
        lines.push(format!("Format: {format}"));
    }
    if let Some(description) = &field.description
        && !description.is_empty()
    {
        lines.push(format!("Description: {description}"));
    }

    if lines.is_empty() {
        None
    } else {
        Some(Documentation::String(lines.join("\n")))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        field_detail, field_documentation, qualifier_before_dot, text_has_dot_before_cursor,
        use_index_table_symbol_at_offset,
    };
    use crate::backend::DbFieldInfo;
    use tower_lsp::lsp_types::Documentation;

    #[test]
    fn finds_qualifier_before_dot_with_dash() {
        let text = "f-lpd_det.";
        let offset = text.len();
        let prefix = "";
        assert_eq!(
            qualifier_before_dot(text, offset, prefix).as_deref(),
            Some("f-lpd_det")
        );
        assert!(text_has_dot_before_cursor(text, offset));
    }

    #[test]
    fn renders_field_detail_and_docs() {
        let field = DbFieldInfo {
            name: "z9zw_id".to_string(),
            field_type: Some("CHARACTER".to_string()),
            format: Some("x(24)".to_string()),
            label: Some("ID".to_string()),
            description: Some("Identifier".to_string()),
        };

        assert_eq!(field_detail(&field, "z9zw_mstr"), "CHARACTER (z9zw_mstr)");
        let docs = field_documentation(&field).expect("documentation");
        match docs {
            Documentation::String(s) => {
                assert!(s.contains("Label: ID"));
                assert!(s.contains("Format: x(24)"));
                assert!(s.contains("Description: Identifier"));
            }
            _ => panic!("unexpected documentation kind"),
        }
    }

    #[test]
    fn detects_use_index_table_symbol() {
        let src = r#"
FOR EACH Customer USE-INDEX CustNum NO-LOCK:
END.
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let offset = src.find("CustNum").expect("index usage");
        let table = use_index_table_symbol_at_offset(tree.root_node(), src, offset + 2)
            .expect("table symbol");
        assert_eq!(table, "Customer");
    }
}
