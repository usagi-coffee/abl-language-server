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

#[cfg(test)]
mod tests {
    use super::containing_scope;
    use crate::analysis::parse_abl;

    #[test]
    fn returns_function_scope_for_offset_inside_function() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL ():
  DEFINE VARIABLE x AS INTEGER NO-UNDO.
  x = 1.
  RETURN TRUE.
END FUNCTION.
"#;
        let tree = parse_abl(src);

        let offset = src.find("x = 1").expect("inside function offset");
        let scope = containing_scope(tree.root_node(), offset).expect("scope");
        assert!(scope.start <= offset);
        assert!(scope.end >= offset);
        assert!(scope.end < tree.root_node().end_byte());
    }

    #[test]
    fn falls_back_to_root_scope_when_not_inside_named_scope_node() {
        let src = r#"
DEFINE VARIABLE y AS INTEGER NO-UNDO.
y = 2.
"#;
        let tree = parse_abl(src);

        let offset = src.find("y = 2").expect("root statement offset");
        let scope = containing_scope(tree.root_node(), offset).expect("scope");

        assert_eq!(scope.start, tree.root_node().start_byte());
        assert_eq!(scope.end, tree.root_node().end_byte());
    }
}
