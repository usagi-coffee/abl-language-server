use crate::analysis::definitions::PreprocessorDefineSite;
use std::path::Path;

pub struct IncludeSite {
    pub path: String,
    pub prefix_macro: Option<String>,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Best-effort scan for ABL include directives in raw source text.
pub fn collect_include_sites(text: &str) -> Vec<IncludeSite> {
    let mut out = Vec::new();
    let mut line_start = 0usize;

    for line in text.split_inclusive('\n') {
        if let Some((relative_start, relative_end, path, prefix_macro)) =
            parse_include_from_line(line)
        {
            out.push(IncludeSite {
                path,
                prefix_macro,
                start_offset: line_start + relative_start,
                end_offset: line_start + relative_end,
            });
        }
        line_start += line.len();
    }

    out
}

fn parse_include_from_line(line: &str) -> Option<(usize, usize, String, Option<String>)> {
    let trimmed = line.trim_start();
    let trim_delta = line.len().saturating_sub(trimmed.len());

    let open = trimmed.find('{')?;
    let close = trimmed.rfind('}')?;
    if close <= open {
        return None;
    }

    let body = &trimmed[open + 1..close];
    let path = extract_include_path(body)?;
    let prefix_macro = extract_prefix_macro_name(body);
    Some((
        trim_delta + open,
        trim_delta + close + 1,
        path,
        prefix_macro,
    ))
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

fn extract_prefix_macro_name(body: &str) -> Option<String> {
    let trimmed = body.trim_start();
    let macro_body = trimmed.strip_prefix("{&")?;
    let close = macro_body.find('}')?;
    let name = macro_body[..close].trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

pub fn resolve_include_site_path(
    include: &IncludeSite,
    define_sites: &[PreprocessorDefineSite],
) -> String {
    let Some(prefix_macro) = include.prefix_macro.as_deref() else {
        return include.path.clone();
    };

    let define_value = define_sites
        .iter()
        .filter(|d| {
            d.label.eq_ignore_ascii_case(prefix_macro) && d.start_byte <= include.start_offset
        })
        .max_by_key(|d| d.start_byte)
        .and_then(|d| d.value.as_deref())
        .map(normalize_define_prefix_path);

    let Some(prefix) = define_value else {
        return include.path.clone();
    };
    let combined = join_prefix_with_include_path(&prefix, &include.path);
    let combined_path = Path::new(&combined);
    if combined_path.is_absolute() && !combined_path.exists() {
        return include.path.clone();
    }

    combined
}

fn normalize_define_prefix_path(value: &str) -> String {
    let trimmed = value.trim();
    let unquoted = if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        &trimmed[1..trimmed.len().saturating_sub(1)]
    } else {
        trimmed
    };
    unquoted.trim().to_string()
}

fn join_prefix_with_include_path(prefix: &str, include_path: &str) -> String {
    let left = prefix.trim_end_matches(['/', '\\']);
    let right = include_path.trim_start_matches(['/', '\\']);
    if left.is_empty() {
        right.to_string()
    } else if right.is_empty() {
        left.to_string()
    } else {
        format!("{left}/{right}")
    }
}

#[cfg(test)]
mod tests {
    use super::{collect_include_sites, resolve_include_site_path};
    use crate::analysis::definitions::PreprocessorDefineSite;
    use tower_lsp::lsp_types::{Position, Range};

    #[test]
    fn extracts_include_paths_and_ranges() {
        let src = "  {zm_catch.i}\n{{&ZM_CIM}cim_sosomt.i &A=B}\n";
        let sites = collect_include_sites(src);

        assert_eq!(sites.len(), 2);
        assert_eq!(sites[0].path, "zm_catch.i");
        assert_eq!(sites[1].path, "cim_sosomt.i");
        assert_eq!(sites[0].prefix_macro, None);
        assert_eq!(sites[1].prefix_macro.as_deref(), Some("ZM_CIM"));
        assert!(sites[0].start_offset < sites[0].end_offset);
    }

    #[test]
    fn resolves_macro_prefixed_include_path() {
        let site = collect_include_sites("{{&ZM_INC}zm_cim.i CHUI}\n")
            .into_iter()
            .next()
            .expect("include site");
        let define_sites = vec![PreprocessorDefineSite {
            label: "ZM_INC".to_string(),
            value: Some("/zmd/dev".to_string()),
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            start_byte: 0,
            is_global: true,
        }];

        let resolved = resolve_include_site_path(&site, &define_sites);
        assert_eq!(resolved, "/zmd/dev/zm_cim.i");
    }

    #[test]
    fn falls_back_to_raw_include_path_when_absolute_macro_prefix_missing() {
        let site = collect_include_sites("{{&ZM_INC}zm_cim.i CHUI}\n")
            .into_iter()
            .next()
            .expect("include site");
        let define_sites = vec![PreprocessorDefineSite {
            label: "ZM_INC".to_string(),
            value: Some("/path/that/does/not/exist".to_string()),
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            start_byte: 0,
            is_global: true,
        }];

        let resolved = resolve_include_site_path(&site, &define_sites);
        assert_eq!(resolved, "zm_cim.i");
    }
}
