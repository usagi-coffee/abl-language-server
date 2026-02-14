use crate::analysis::definitions::PreprocessorDefineSite;
use crate::utils::ts::collect_nodes_by_kind;
use std::path::Path;
use tree_sitter::Node;

pub struct IncludeSite {
    pub path: String,
    pub prefix_macro: Option<String>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub file_start_offset: usize,
    pub file_end_offset: usize,
}

/// Best-effort scan for ABL include directives in raw source text.
#[cfg(test)]
pub fn collect_include_sites(text: &str) -> Vec<IncludeSite> {
    let mut out = Vec::new();
    for (start, end, body) in collect_braced_bodies(text) {
        let Some(path) = extract_include_path(body) else {
            continue;
        };
        out.push(IncludeSite {
            path,
            prefix_macro: extract_prefix_macro_name(body),
            start_offset: start,
            end_offset: end,
            file_start_offset: start + 1,
            file_end_offset: end.saturating_sub(1),
        });
    }

    out
}

pub fn collect_include_sites_from_tree(root: Node<'_>, src: &[u8]) -> Vec<IncludeSite> {
    let mut refs = Vec::new();
    collect_nodes_by_kind(root, "include_file_reference", &mut refs);

    let mut out = Vec::new();
    for node in refs {
        let Some(file_node) = node.child_by_field_name("file") else {
            continue;
        };
        let Ok(file_text) = file_node.utf8_text(src) else {
            continue;
        };
        let Some(path) = extract_include_path(file_text) else {
            continue;
        };
        out.push(IncludeSite {
            path,
            prefix_macro: extract_prefix_macro_name(file_text),
            start_offset: node.start_byte(),
            end_offset: node.end_byte(),
            file_start_offset: file_node.start_byte(),
            file_end_offset: file_node.end_byte(),
        });
    }
    out.sort_by_key(|s| s.start_offset);
    out
}

#[cfg(test)]
fn collect_braced_bodies(text: &str) -> Vec<(usize, usize, &str)> {
    let mut out = Vec::new();
    let mut stack = Vec::<usize>::new();

    for (idx, b) in text.as_bytes().iter().enumerate() {
        match *b {
            b'{' => stack.push(idx),
            b'}' => {
                let Some(open) = stack.pop() else {
                    continue;
                };
                if stack.is_empty() && idx > open + 1 {
                    out.push((open, idx + 1, &text[open + 1..idx]));
                }
            }
            _ => {}
        }
    }

    out
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

pub fn include_site_matches_file_offset(include: &IncludeSite, offset: usize) -> bool {
    offset >= include.file_start_offset && offset <= include.file_end_offset
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
    use super::{
        collect_include_sites, collect_include_sites_from_tree, resolve_include_site_path,
    };
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
        assert!(sites[0].file_start_offset >= sites[0].start_offset);
        assert!(sites[0].file_end_offset <= sites[0].end_offset);
    }

    #[test]
    fn extracts_multiline_include_paths_and_ranges() {
        let src = "{zm_mail.i \n  &To=cEmail\n  &Subject=cSubject\n}\n";
        let sites = collect_include_sites(src);

        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].path, "zm_mail.i");
        assert_eq!(sites[0].start_offset, 0);
        assert_eq!(sites[0].end_offset, src.find('}').expect("close brace") + 1);
    }

    #[test]
    fn extracts_include_paths_from_tree_reference_nodes() {
        let src = r#"{ETYK/pz_etyk4pojemnik.i}"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let sites = collect_include_sites_from_tree(tree.root_node(), src.as_bytes());
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].path, "ETYK/pz_etyk4pojemnik.i");
        assert!(sites[0].file_start_offset >= sites[0].start_offset);
        assert!(sites[0].file_end_offset <= sites[0].end_offset);
    }

    #[test]
    fn resolves_macro_prefixed_include_path() {
        let base = std::env::temp_dir().join(format!(
            "abl_ls_include_prefix_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("epoch")
                .as_nanos()
        ));
        std::fs::create_dir_all(&base).expect("create base dir");
        std::fs::write(base.join("zm_cim.i"), "/* include */").expect("write include file");

        let site = collect_include_sites("{{&ZM_INC}zm_cim.i CHUI}\n")
            .into_iter()
            .next()
            .expect("include site");
        let define_sites = vec![PreprocessorDefineSite {
            label: "ZM_INC".to_string(),
            value: Some(base.to_string_lossy().to_string()),
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            start_byte: 0,
            is_global: true,
        }];

        let resolved = resolve_include_site_path(&site, &define_sites);
        assert_eq!(resolved, format!("{}/zm_cim.i", base.to_string_lossy()));

        let _ = std::fs::remove_dir_all(&base);
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
