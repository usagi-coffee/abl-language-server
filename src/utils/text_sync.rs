use tower_lsp::lsp_types::TextDocumentContentChangeEvent;

use crate::utils::position::lsp_pos_to_utf8_byte_offset;

pub fn apply_content_changes(
    mut text: String,
    changes: &[TextDocumentContentChangeEvent],
) -> Option<String> {
    if changes.is_empty() {
        return Some(text);
    }

    for change in changes {
        match change.range {
            None => {
                text = change.text.clone();
            }
            Some(range) => {
                let start = lsp_pos_to_utf8_byte_offset(&text, range.start)?;
                let end = lsp_pos_to_utf8_byte_offset(&text, range.end)?;
                if start > end || end > text.len() {
                    return None;
                }
                text.replace_range(start..end, &change.text);
            }
        }
    }

    Some(text)
}

#[cfg(test)]
mod tests {
    use super::apply_content_changes;
    use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    #[test]
    fn applies_full_text_change() {
        let out = apply_content_changes(
            "abc".to_string(),
            &[TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: "xyz".to_string(),
            }],
        )
        .expect("updated text");
        assert_eq!(out, "xyz");
    }

    #[test]
    fn applies_incremental_change() {
        let out = apply_content_changes(
            "test_a".to_string(),
            &[TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(0, 5), Position::new(0, 6))),
                range_length: None,
                text: "b".to_string(),
            }],
        )
        .expect("updated text");
        assert_eq!(out, "test_b");
    }
}
