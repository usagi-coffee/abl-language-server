use std::path::{Path, PathBuf};

use tower_lsp::lsp_types::Url;

pub fn expand_dumpfile_pattern(workspace_root: Option<&Path>, dumpfile: &str) -> Vec<PathBuf> {
    let Some(base) = resolve_config_path(workspace_root, dumpfile) else {
        return Vec::new();
    };

    if !dumpfile.contains('*') {
        return if base.exists() {
            vec![base]
        } else {
            Vec::new()
        };
    }

    let normalized = normalize_path_for_match(&base.to_string_lossy());
    let search_root = glob_search_root(&base);
    let mut matches = Vec::new();
    collect_dumpfile_matches(&search_root, &normalized, &mut matches);
    matches.sort();
    matches.dedup();
    matches
}

fn collect_dumpfile_matches(root: &Path, pattern: &str, out: &mut Vec<PathBuf>) {
    let Ok(read_dir) = std::fs::read_dir(root) else {
        return;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_dumpfile_matches(&path, pattern, out);
            continue;
        }

        let normalized = normalize_path_for_match(path.to_string_lossy().as_ref());
        if wildcard_match(pattern, &normalized) {
            out.push(path);
        }
    }
}

fn glob_search_root(pattern: &Path) -> PathBuf {
    let pattern_str = pattern.to_string_lossy();
    let Some(idx) = pattern_str.find('*') else {
        return pattern
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| pattern.to_path_buf());
    };

    let prefix = &pattern_str[..idx];
    let prefix_path = PathBuf::from(prefix);
    if prefix_path.as_os_str().is_empty() {
        pattern
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    } else if prefix.ends_with(['/', '\\']) {
        prefix_path
    } else {
        prefix_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or(prefix_path)
    }
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

pub fn resolve_glob_pattern(
    workspace_root: Option<&Path>,
    value: &str,
) -> Option<(PathBuf, String)> {
    let candidate = PathBuf::from(value);
    if candidate.is_absolute() {
        let parent = candidate.parent()?.to_path_buf();
        let file_name = candidate.file_name()?.to_string_lossy().to_string();
        return Some((parent, file_name));
    }

    let root = workspace_root?;
    let joined = root.join(candidate);
    let parent = joined.parent()?.to_path_buf();
    let file_name = joined.file_name()?.to_string_lossy().to_string();
    Some((parent, file_name))
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
        .unwrap_or_default();

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
    use super::{
        expand_dumpfile_pattern, path_matches_any_pattern, resolve_glob_pattern,
        resolve_include_path, wildcard_match,
    };
    use std::fs;
    use std::path::{Path, PathBuf};

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

    #[test]
    fn resolve_glob_pattern_keeps_parent_directory_and_filename_pattern() {
        let workspace = Path::new("/workspace");
        let (parent, file_name) =
            resolve_glob_pattern(Some(workspace), "ambient/*.i").expect("glob pattern");
        assert_eq!(parent, PathBuf::from("/workspace/ambient"));
        assert_eq!(file_name, "*.i");
    }

    #[test]
    fn expand_dumpfile_pattern_matches_files() {
        let base = std::env::temp_dir().join(format!(
            "abl_ls_dumpfile_glob_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("epoch")
                .as_nanos()
        ));
        let dumps = base.join("dumps");
        fs::create_dir_all(&dumps).expect("create dumps");
        let a = dumps.join("a.df");
        let b = dumps.join("b.df");
        let c = dumps.join("c.txt");
        fs::write(&a, "").expect("write a");
        fs::write(&b, "").expect("write b");
        fs::write(&c, "").expect("write c");

        let mut resolved = expand_dumpfile_pattern(Some(&base), "dumps/*.df");
        resolved.sort();
        assert_eq!(resolved, vec![a, b]);

        let _ = fs::remove_dir_all(&base);
    }
}
