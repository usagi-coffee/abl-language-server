use tower_lsp::lsp_types::Position;

/// Converts an LSP Position to a UTF-8 byte offset in the text.
/// Assumes Position.character is a UTF-8 byte column within that line.
pub fn lsp_pos_to_utf8_byte_offset(text: &str, pos: Position) -> Option<usize> {
    let line = pos.line as usize;
    let col = pos.character as usize;

    let mut cur_line = 0usize;
    let mut line_start = 0usize;

    for (i, b) in text.bytes().enumerate() {
        if cur_line == line {
            line_start = i;
            break;
        }
        if b == b'\n' {
            cur_line += 1;
        }
    }

    if line == 0 {
        line_start = 0;
    } else if cur_line != line {
        return None;
    }

    // find line end
    let line_end = text[line_start..]
        .find('\n')
        .map(|d| line_start + d)
        .unwrap_or(text.len());

    let target = line_start.saturating_add(col);
    if target > line_end {
        Some(line_end)
    } else {
        Some(target)
    }
}

/// Walks backward from offset and captures [A-Za-z0-9_]* as prefix.
pub fn ascii_ident_prefix(text: &str, mut offset: usize) -> String {
    let bytes = text.as_bytes();
    if offset > bytes.len() {
        offset = bytes.len();
    }
    let mut start = offset;
    while start > 0 {
        let c = bytes[start - 1];
        let is_ident = c.is_ascii_alphanumeric() || c == b'_';
        if !is_ident {
            break;
        }
        start -= 1;
    }
    text[start..offset].to_string()
}

/// Returns the full ASCII identifier at the given offset or immediately before it.
pub fn ascii_ident_at_or_before(text: &str, mut offset: usize) -> Option<String> {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return None;
    }

    if offset > bytes.len() {
        offset = bytes.len();
    }

    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';

    let cursor = if offset < bytes.len() && is_ident(bytes[offset]) {
        offset
    } else if offset > 0 && is_ident(bytes[offset - 1]) {
        offset - 1
    } else {
        return None;
    };

    let mut start = cursor;
    while start > 0 && is_ident(bytes[start - 1]) {
        start -= 1;
    }

    let mut end = cursor + 1;
    while end < bytes.len() && is_ident(bytes[end]) {
        end += 1;
    }

    Some(text[start..end].to_string())
}
