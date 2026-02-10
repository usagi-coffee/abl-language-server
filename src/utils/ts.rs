use tower_lsp::lsp_types::{Position, Range};
use tree_sitter::Node;

pub fn node_trimmed_text(node: Node<'_>, src: &[u8]) -> Option<String> {
    node.utf8_text(src)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn direct_child_by_kind<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32)
            && ch.kind() == kind
        {
            return Some(ch);
        }
    }
    None
}

pub fn first_descendant_by_kind<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    if node.kind() == kind {
        return Some(node);
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32)
            && let Some(found) = first_descendant_by_kind(ch, kind)
        {
            return Some(found);
        }
    }
    None
}

pub fn collect_nodes_by_kind<'tree>(node: Node<'tree>, kind: &str, out: &mut Vec<Node<'tree>>) {
    if node.kind() == kind {
        out.push(node);
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_nodes_by_kind(ch, kind, out);
        }
    }
}

pub fn count_nodes_by_kind(node: Node<'_>, kind: &str) -> usize {
    let mut count = 0usize;
    if node.kind() == kind {
        count += 1;
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            count += count_nodes_by_kind(ch, kind);
        }
    }
    count
}

pub fn point_to_position(point: tree_sitter::Point) -> Position {
    Position::new(point.row as u32, point.column as u32)
}

pub fn node_to_range(node: Node<'_>) -> Range {
    Range::new(
        point_to_position(node.start_position()),
        point_to_position(node.end_position()),
    )
}
