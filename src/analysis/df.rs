use std::collections::HashSet;
use tree_sitter::Node;

/// Collects table names from parsed DF source (`ADD TABLE "name"` statements).
pub fn collect_df_table_names(node: Node, src: &[u8], out: &mut HashSet<String>) {
    if node.kind() == "add_table_statement"
        && let Some(table_node) = node.child_by_field_name("table")
        && let Ok(raw) = table_node.utf8_text(src)
        && let Some(name) = unquote(raw)
    {
        out.insert(name.to_ascii_uppercase());
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_table_names(ch, src, out);
        }
    }
}

fn unquote(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0];
        let last = trimmed.as_bytes()[trimmed.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return Some(&trimmed[1..trimmed.len() - 1]);
        }
    }
    None
}
