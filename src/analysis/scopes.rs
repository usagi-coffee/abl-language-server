use tree_sitter::Node;

#[derive(Clone, Copy)]
pub struct ByteScope {
    pub start: usize,
    pub end: usize,
}

pub fn containing_scope(root: Node<'_>, offset: usize) -> Option<ByteScope> {
    let mut node = root.named_descendant_for_byte_range(offset, offset)?;
    loop {
        if is_scope_node(node.kind()) {
            return Some(ByteScope {
                start: node.start_byte(),
                end: node.end_byte(),
            });
        }
        let Some(parent) = node.parent() else {
            break;
        };
        node = parent;
    }

    Some(ByteScope {
        start: root.start_byte(),
        end: root.end_byte(),
    })
}

fn is_scope_node(kind: &str) -> bool {
    matches!(
        kind,
        "function_definition"
            | "function_forward_definition"
            | "procedure_definition"
            | "method_definition"
            | "constructor_definition"
            | "destructor_definition"
    )
}
