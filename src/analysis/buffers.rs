use tree_sitter::Node;

pub struct BufferMapping {
    pub alias: String,
    pub table: String,
    pub start_byte: usize,
}

pub fn collect_buffer_mappings(node: Node, src: &[u8], out: &mut Vec<BufferMapping>) {
    if node.kind() == "buffer_definition"
        && let (Some(name_node), Some(table_node)) = (
            node.child_by_field_name("name"),
            node.child_by_field_name("table"),
        )
        && let (Ok(alias), Ok(table_raw)) = (name_node.utf8_text(src), table_node.utf8_text(src))
    {
        let alias = alias.trim();
        let table = normalize_table_name(table_raw);
        if !alias.is_empty() && !table.is_empty() {
            out.push(BufferMapping {
                alias: alias.to_string(),
                table,
                start_byte: node.start_byte(),
            });
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_buffer_mappings(ch, src, out);
        }
    }
}

fn normalize_table_name(raw: &str) -> String {
    raw.trim()
        .split('.')
        .next_back()
        .unwrap_or("")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::collect_buffer_mappings;
    use crate::analysis::parse_abl;

    #[test]
    fn collects_buffer_alias_and_table_name() {
        let src = r#"
DEFINE BUFFER f-lpd_det FOR lpd_det.
DEFINE BUFFER b-pt FOR sports.pt_mstr.
"#;

        let tree = parse_abl(src);

        let mut out = Vec::new();
        collect_buffer_mappings(tree.root_node(), src.as_bytes(), &mut out);

        assert!(
            out.iter()
                .any(|m| m.alias == "f-lpd_det" && m.table == "lpd_det")
        );
        assert!(
            out.iter()
                .any(|m| m.alias == "b-pt" && m.table == "pt_mstr")
        );
    }
}
