use std::path::{Path, PathBuf};

use tower_lsp::lsp_types::Url;

pub fn resolve_dumpfile_path(workspace_root: Option<&Path>, dumpfile: &str) -> Option<PathBuf> {
    resolve_config_path(workspace_root, dumpfile)
}

pub fn resolve_include_path(
    workspace_root: Option<&Path>,
    propath: &[String],
    current_file: &Path,
    include: &str,
) -> Option<PathBuf> {
    let candidate = PathBuf::from(include);
    if candidate.is_absolute() {
        return Some(candidate);
    }

    for entry in propath {
        let Some(base) = resolve_config_path(workspace_root, entry) else {
            continue;
        };
        let from_propath = base.join(include);
        if from_propath.exists() {
            return Some(from_propath);
        }
    }

    if let Some(current_dir) = current_file.parent() {
        let from_current = current_dir.join(include);
        if from_current.exists() {
            return Some(from_current);
        }
    }

    if let Some(root) = workspace_root {
        let from_root = root.join(include);
        if from_root.exists() {
            return Some(from_root);
        }
    }

    None
}

pub fn resolve_config_path(workspace_root: Option<&Path>, value: &str) -> Option<PathBuf> {
    let candidate = PathBuf::from(value);
    if candidate.is_absolute() {
        return Some(candidate);
    }
    workspace_root.map(|root| root.join(candidate))
}

pub fn uri_matches_any_path_pattern(
    uri: &Url,
    workspace_root: Option<&Path>,
    patterns: &[String],
) -> bool {
    let Ok(path) = uri.to_file_path() else {
        return false;
    };
    path_matches_any_pattern(&path, workspace_root, patterns)
}

pub fn path_matches_any_pattern(
    path: &Path,
    workspace_root: Option<&Path>,
    patterns: &[String],
) -> bool {
    let abs = normalize_path_for_match(path.to_string_lossy().as_ref());
    let base = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let base_norm = normalize_path_for_match(&base);
    let rel = workspace_root
        .and_then(|r| path.strip_prefix(r).ok().map(|p| p.to_path_buf()))
        .map(|p| normalize_path_for_match(p.to_string_lossy().as_ref()))
        .unwrap_or_else(String::new);

    patterns.iter().any(|p| {
        let pat = normalize_path_for_match(p);
        wildcard_match(&pat, &abs)
            || (!rel.is_empty() && wildcard_match(&pat, &rel))
            || wildcard_match(&pat, &base_norm)
    })
}

pub fn normalize_path_for_match(raw: &str) -> String {
    raw.replace('\\', "/").to_ascii_lowercase()
}

pub fn wildcard_match(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }
    if !pattern.contains('*') {
        return text == pattern || text.starts_with(&(pattern.to_string() + "/"));
    }

    let mut p = 0usize;
    let mut t = 0usize;
    let pb = pattern.as_bytes();
    let tb = text.as_bytes();
    let mut star_idx: Option<usize> = None;
    let mut match_idx = 0usize;

    while t < tb.len() {
        if p < pb.len() && (pb[p] == tb[t]) {
            p += 1;
            t += 1;
        } else if p < pb.len() && pb[p] == b'*' {
            star_idx = Some(p);
            p += 1;
            match_idx = t;
        } else if let Some(si) = star_idx {
            p = si + 1;
            match_idx += 1;
            t = match_idx;
        } else {
            return false;
        }
    }
    while p < pb.len() && pb[p] == b'*' {
        p += 1;
    }
    p == pb.len()
}

#[cfg(test)]
mod tests {
    use super::{path_matches_any_pattern, resolve_include_path, wildcard_match};
    use std::fs;

    #[test]
    fn include_resolution_uses_propath_order() {
        let base = std::env::temp_dir().join(format!(
            "abl_ls_backend_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("epoch")
                .as_nanos()
        ));
        let workspace = base.join("workspace");
        let propath_a = base.join("a");
        let propath_b = base.join("b");
        let current_dir = base.join("current");
        fs::create_dir_all(&workspace).expect("create workspace");
        fs::create_dir_all(&propath_a).expect("create propath a");
        fs::create_dir_all(&propath_b).expect("create propath b");
        fs::create_dir_all(&current_dir).expect("create current dir");

        let include = "include.i";
        let a_file = propath_a.join(include);
        let b_file = propath_b.join(include);
        let current_file = current_dir.join("main.p");
        let current_include = current_dir.join(include);
        let root_include = workspace.join(include);
        fs::write(&a_file, "/* a */").expect("write a");
        fs::write(&b_file, "/* b */").expect("write b");
        fs::write(&current_file, "").expect("write current");
        fs::write(&current_include, "/* current */").expect("write current include");
        fs::write(&root_include, "/* root */").expect("write root include");

        let propath = vec![propath_a.to_string_lossy().to_string(), ".".to_string()];
        let resolved = resolve_include_path(Some(&workspace), &propath, &current_file, include)
            .expect("resolved include");
        assert_eq!(resolved, a_file);

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn include_resolution_falls_back_to_current_then_workspace() {
        let base = std::env::temp_dir().join(format!(
            "abl_ls_backend_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("epoch")
                .as_nanos()
        ));
        let workspace = base.join("workspace");
        let current_dir = base.join("current");
        fs::create_dir_all(&workspace).expect("create workspace");
        fs::create_dir_all(&current_dir).expect("create current dir");

        let include = "include.i";
        let current_file = current_dir.join("main.p");
        let current_include = current_dir.join(include);
        let root_include = workspace.join(include);
        fs::write(&current_file, "").expect("write current");
        fs::write(&current_include, "/* current */").expect("write current include");
        fs::write(&root_include, "/* root */").expect("write root include");

        let resolved =
            resolve_include_path(Some(&workspace), &[], &current_file, include).expect("resolved");
        assert_eq!(resolved, current_include);

        fs::remove_file(&current_include).expect("remove current include");
        let resolved =
            resolve_include_path(Some(&workspace), &[], &current_file, include).expect("resolved");
        assert_eq!(resolved, root_include);

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn wildcard_match_supports_star_patterns() {
        assert!(wildcard_match("legacy/*.p", "legacy/a.p"));
        assert!(wildcard_match("legacy/*", "legacy/dir/file.p"));
        assert!(!wildcard_match("legacy/*.p", "other/a.p"));
    }

    #[test]
    fn path_matching_checks_abs_rel_and_basename() {
        let base = std::env::temp_dir().join(format!(
            "abl_ls_path_match_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("epoch")
                .as_nanos()
        ));
        let workspace = base.join("workspace");
        let subdir = workspace.join("legacy");
        let file = subdir.join("special.p");
        fs::create_dir_all(&subdir).expect("create subdir");
        fs::write(&file, "").expect("write file");

        assert!(path_matches_any_pattern(
            &file,
            Some(&workspace),
            &["legacy/*".to_string()]
        ));
        assert!(path_matches_any_pattern(
            &file,
            Some(&workspace),
            &["special.p".to_string()]
        ));

        let _ = fs::remove_dir_all(&base);
    }
}
