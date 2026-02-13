use tower_lsp::lsp_types::Range;

pub fn is_in_range(start_line: u32, start_col: u32, length: u32, range: Option<&Range>) -> bool {
    let Some(range) = range else {
        return true;
    };
    let token_end_col = start_col.saturating_add(length);

    if start_line < range.start.line || start_line > range.end.line {
        return false;
    }
    if start_line == range.start.line && token_end_col <= range.start.character {
        return false;
    }
    if start_line == range.end.line && start_col >= range.end.character {
        return false;
    }

    true
}

pub fn line_start_offsets(text: &str) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

pub fn point_column_byte_to_utf16(
    text: &str,
    line_starts: &[usize],
    line: u32,
    column_byte: usize,
) -> Option<u32> {
    let line_start = *line_starts.get(line as usize)?;
    let abs = line_start.saturating_add(column_byte);
    if abs > text.len() || !text.is_char_boundary(abs) {
        return None;
    }
    Some(text[line_start..abs].encode_utf16().count() as u32)
}

#[cfg(test)]
mod tests {
    use super::{line_start_offsets, point_column_byte_to_utf16};

    #[test]
    fn converts_byte_column_to_utf16_with_non_ascii_prefix() {
        let text = "oNestedObject:Add(\"Ilość\", lpopak_mstr.lpopak_ilosc).";
        let starts = line_start_offsets(text);

        let token = "lpopak_mstr";
        let byte_col = text.find(token).expect("token byte column");
        let utf16_col = point_column_byte_to_utf16(text, &starts, 0, byte_col)
            .expect("utf16 column conversion");

        let expected_utf16 = text[..byte_col].encode_utf16().count() as u32;
        assert_eq!(utf16_col, expected_utf16);
    }
}
