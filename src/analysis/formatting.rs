use tree_sitter::{Node, Parser};

#[derive(Debug, Clone, Copy)]
pub struct IndentOptions {
    pub indent_size: usize,
    pub use_tabs: bool,
}

impl Default for IndentOptions {
    fn default() -> Self {
        Self {
            indent_size: 2,
            use_tabs: false,
        }
    }
}

pub fn autoindent_text(text: &str, options: IndentOptions) -> String {
    let mut out = String::with_capacity(text.len());
    let mut line_indents = vec![0usize; line_count(text)];

    if let Some(tree) = parse_abl_tree(text) {
        collect_line_indents(tree.root_node(), &mut line_indents);
    }

    for (idx, raw_line) in text.split_inclusive('\n').enumerate() {
        let (line_without_nl, newline) = split_line_ending(raw_line);
        let trimmed = line_without_nl.trim_start_matches([' ', '\t']);
        if trimmed.is_empty() {
            out.push_str(newline);
            continue;
        }

        let indent = line_indents.get(idx).copied().unwrap_or_default();
        push_indent(&mut out, indent, options);
        out.push_str(trimmed);
        out.push_str(newline);
    }

    out
}

pub fn preserves_ast_shape(original: &str, formatted: &str, parser: &mut Parser) -> bool {
    let Some(before) = parser.parse(original, None) else {
        return false;
    };
    let Some(after) = parser.parse(formatted, None) else {
        return false;
    };

    let before_root = before.root_node();
    let after_root = after.root_node();
    if before_root.has_error() || after_root.has_error() {
        return false;
    }

    before_root.to_sexp() == after_root.to_sexp()
}

fn split_line_ending(raw_line: &str) -> (&str, &str) {
    if let Some(stripped) = raw_line.strip_suffix("\r\n") {
        return (stripped, "\r\n");
    }
    if let Some(stripped) = raw_line.strip_suffix('\n') {
        return (stripped, "\n");
    }
    (raw_line, "")
}

fn line_count(text: &str) -> usize {
    if text.is_empty() {
        return 1;
    }
    text.bytes().filter(|b| *b == b'\n').count() + 1
}

fn parse_abl_tree(text: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_abl::LANGUAGE.into())
        .ok()?;
    parser.parse(text, None)
}

fn collect_line_indents(node: Node<'_>, line_indents: &mut [usize]) {
    apply_body_indent(node, line_indents);
    apply_continuation_indent(node, line_indents);

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.is_named() {
            collect_line_indents(child, line_indents);
        }
    }
}

fn apply_body_indent(node: Node<'_>, line_indents: &mut [usize]) {
    if node.kind() == "include_file_reference" {
        let start = node.start_position().row + 1;
        let end = node.end_position().row.saturating_sub(1);
        add_indent_range(line_indents, start, end);
    }

    if let Some(body) = first_named_child_of_kind(node, "body") {
        let start_row = body.start_position().row;
        let start_col = body.start_position().column;
        let mut end_row = body.end_position().row;
        let end_col = body.end_position().column;

        let start = if start_col > 0 {
            start_row.saturating_add(1)
        } else {
            start_row
        };
        if end_col == 0 && end_row > 0 {
            end_row -= 1;
        }
        add_indent_range(line_indents, start, end_row);
    }
}

fn apply_continuation_indent(node: Node<'_>, line_indents: &mut [usize]) {
    let Some((start, end)) = continuation_range(node) else {
        return;
    };
    add_indent_range(line_indents, start, end);
}

fn continuation_range(node: Node<'_>) -> Option<(usize, usize)> {
    let start_row = node.start_position().row;
    let mut end_row = node.end_position().row;
    let end_col = node.end_position().column;
    if end_col == 0 && end_row > 0 {
        end_row -= 1;
    }

    match node.kind() {
        "for_statement" | "repeat_statement" | "do_statement" | "if_statement" => {
            let body_or_then = if node.kind() == "if_statement" {
                if_then_anchor(node)
            } else {
                first_named_child_of_kind(node, "body")
            };
            let Some(anchor) = body_or_then else {
                return None;
            };
            let anchor_row = anchor.start_position().row;
            let anchor_col = anchor.start_position().column;
            let upper = if anchor_col == 0 {
                anchor_row.saturating_sub(1)
            } else {
                anchor_row
            };
            let from = start_row.saturating_add(1);
            (from <= upper).then_some((from, upper))
        }
        "find_statement"
        | "assign_statement"
        | "assignment_statement"
        | "prompt_for_statement"
        | "transaction_statement"
        | "can_find_expression"
        | "function_call_statement" => {
            let from = start_row.saturating_add(1);
            (from <= end_row).then_some((from, end_row))
        }
        _ => None,
    }
}

fn first_named_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.is_named() && child.kind() == kind)
}

fn if_then_anchor<'a>(node: Node<'a>) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    let mut saw_condition = false;
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            continue;
        }
        if !saw_condition {
            saw_condition = true;
            continue;
        }
        let kind = child.kind();
        if kind.ends_with("_statement") || kind.ends_with("_definition") || kind == "body" {
            return Some(child);
        }
    }
    None
}

fn add_indent_range(line_indents: &mut [usize], start: usize, end: usize) {
    if start > end || line_indents.is_empty() {
        return;
    }
    let from = start.min(line_indents.len() - 1);
    let to = end.min(line_indents.len() - 1);
    if from > to {
        return;
    }
    for indent in line_indents.iter_mut().take(to + 1).skip(from) {
        *indent += 1;
    }
}

fn push_indent(out: &mut String, level: usize, options: IndentOptions) {
    if options.use_tabs {
        for _ in 0..level {
            out.push('\t');
        }
        return;
    }

    let width = options.indent_size.max(1);
    for _ in 0..(level * width) {
        out.push(' ');
    }
}

#[cfg(test)]
mod tests {
    use super::{IndentOptions, autoindent_text, collect_line_indents, preserves_ast_shape};
    use tree_sitter::Parser;

    fn parse_abl(src: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        parser.parse(src, None).expect("parse source")
    }

    #[test]
    fn indents_simple_do_block() {
        let input = "IF TRUE THEN DO:\nMESSAGE \"X\".\nEND.\n";
        let got = autoindent_text(input, IndentOptions::default());
        let expected = "IF TRUE THEN DO:\n  MESSAGE \"X\".\nEND.\n";
        assert_eq!(got, expected);
    }

    #[test]
    fn keeps_for_each_header_continuation_indented() {
        let input = "FOR EACH cust WHERE\nname = \"A\" AND\ncity = \"B\"\nNO-LOCK\nON ERROR UNDO, THROW:\nMESSAGE cust.name.\nEND.\n";
        let got = autoindent_text(input, IndentOptions::default());
        let expected = "FOR EACH cust WHERE\n  name = \"A\" AND\n  city = \"B\"\n  NO-LOCK\n  ON ERROR UNDO, THROW:\n  MESSAGE cust.name.\nEND.\n";
        assert_eq!(got, expected);
    }

    #[test]
    fn indents_include_arguments() {
        let input = "{{&ZM_CIM}cim_icunis.i\n&CIM=CIM\n&OUT=OUT\n}\n";
        let got = autoindent_text(input, IndentOptions::default());
        let expected = "{{&ZM_CIM}cim_icunis.i\n  &CIM=CIM\n  &OUT=OUT\n}\n";
        assert_eq!(got, expected);
    }

    #[test]
    fn indents_multiline_if_condition() {
        let input = "IF a = 1 AND\nb = 2 THEN DO:\nMESSAGE \"ok\".\nEND.\n";
        let got = autoindent_text(input, IndentOptions::default());
        let expected = "IF a = 1 AND\n  b = 2 THEN DO:\n  MESSAGE \"ok\".\nEND.\n";
        assert_eq!(got, expected);
    }

    #[test]
    fn indents_assign_continuation_lines() {
        let input = "ASSIGN\nx = 1\ny = 2.\n";
        let got = autoindent_text(input, IndentOptions::default());
        let expected = "ASSIGN\n  x = 1\n  y = 2.\n";
        assert_eq!(got, expected);
    }

    #[test]
    fn preserves_ast_for_indentation_only_change() {
        let source = "IF TRUE THEN DO:\nMESSAGE \"X\".\nEND.\n";
        let formatted = autoindent_text(source, IndentOptions::default());
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        assert!(preserves_ast_shape(source, &formatted, &mut parser));
    }

    #[test]
    fn output_is_idempotent() {
        let input = "IF TRUE THEN DO:\nMESSAGE \"X\".\nEND.\n";
        let once = autoindent_text(input, IndentOptions::default());
        let twice = autoindent_text(&once, IndentOptions::default());
        assert_eq!(once, twice);
        // sanity: parse remains valid
        let _ = parse_abl(&once);
    }

    #[test]
    fn derives_indent_from_do_body_node() {
        let source = "IF TRUE THEN DO:\nMESSAGE \"X\".\nEND.\n";
        let tree = parse_abl(source);
        let mut indents = vec![0usize; 4];
        collect_line_indents(tree.root_node(), &mut indents);
        assert_eq!(indents, vec![0, 1, 0, 0]);
    }
}
