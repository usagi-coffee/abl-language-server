use log::debug;
use tree_sitter::Node;

/// Walks the syntax tree and extracts declared variable names.
///
/// Adapts to the ABL grammar:
/// - Looks for "variable_definition" nodes
/// - Extracts the "name" field containing the identifier
pub fn collect_variable_decls(node: Node, src: &[u8], out: &mut Vec<String>) {
    debug!("{}", node.kind());
    if matches!(node.kind(), "variable_definition") {
        if let Some(name) = node.child_by_field_name("name") {
            if name.kind() == "identifier"
                && let Ok(s) = name.utf8_text(src)
            {
                out.push(s.to_string());
            }
        } else {
            // Pattern B: declaration contains an identifier somewhere inside (fallback)
            find_first_identifier(node, src, out);
        }
    }

    // DFS
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_variable_decls(ch, src, out);
        }
    }
}

fn find_first_identifier(node: Node, src: &[u8], out: &mut Vec<String>) {
    if node.kind() == "identifier" {
        if let Ok(s) = node.utf8_text(src) {
            out.push(s.to_string());
        }
        return;
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            find_first_identifier(ch, src, out);
        }
    }
}
