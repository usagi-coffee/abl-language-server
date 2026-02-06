pub struct IncludeSite {
    pub path: String,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Best-effort scan for ABL include directives in raw source text.
pub fn collect_include_sites(text: &str) -> Vec<IncludeSite> {
    let mut out = Vec::new();
    let mut line_start = 0usize;

    for line in text.split_inclusive('\n') {
        if let Some((relative_start, relative_end, path)) = parse_include_from_line(line) {
            out.push(IncludeSite {
                path,
                start_offset: line_start + relative_start,
                end_offset: line_start + relative_end,
            });
        }
        line_start += line.len();
    }

    out
}

fn parse_include_from_line(line: &str) -> Option<(usize, usize, String)> {
    let trimmed = line.trim_start();
    let trim_delta = line.len().saturating_sub(trimmed.len());

    let open = trimmed.find('{')?;
    let close = trimmed.rfind('}')?;
    if close <= open {
        return None;
    }

    let body = &trimmed[open + 1..close];
    let path = extract_include_path(body)?;
    Some((trim_delta + open, trim_delta + close + 1, path))
}

fn extract_include_path(body: &str) -> Option<String> {
    let lower = body.to_ascii_lowercase();
    let idx = lower.find(".i")?;
    let end = idx + 2;

    let bytes = body.as_bytes();
    let mut start = idx;
    while start > 0 && is_path_char(bytes[start - 1]) {
        start -= 1;
    }

    let mut stop = end;
    while stop < body.len() && is_path_char(bytes[stop]) {
        stop += 1;
    }

    let candidate = body[start..stop].trim();
    if candidate.is_empty() {
        return None;
    }

    Some(candidate.to_string())
}

fn is_path_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.' | b'/' | b'\\')
}

#[cfg(test)]
mod tests {
    use super::collect_include_sites;

    #[test]
    fn extracts_include_paths_and_ranges() {
        let src = "  {zm_catch.i}\n{{&ZM_CIM}cim_sosomt.i &A=B}\n";
        let sites = collect_include_sites(src);

        assert_eq!(sites.len(), 2);
        assert_eq!(sites[0].path, "zm_catch.i");
        assert_eq!(sites[1].path, "cim_sosomt.i");
        assert!(sites[0].start_offset < sites[0].end_offset);
    }
}
