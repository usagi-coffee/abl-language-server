use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tower_lsp::lsp_types::InitializeParams;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct AblConfig {
    pub completion: CompletionConfig,
    pub diagnostics: DiagnosticsConfig,
    pub semantic_tokens: SemanticTokensConfig,
    #[serde(default, deserialize_with = "deserialize_dumpfile")]
    pub dumpfile: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_propath")]
    pub propath: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct CompletionConfig {
    pub enabled: bool,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DiagnosticsConfig {
    pub enabled: bool,
    pub unknown_variables: DiagnosticFeatureConfig,
    pub unknown_functions: DiagnosticFeatureConfig,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            unknown_variables: DiagnosticFeatureConfig::default(),
            unknown_functions: DiagnosticFeatureConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DiagnosticFeatureConfig {
    pub enabled: bool,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub exclude: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub ignore: Vec<String>,
}

impl Default for DiagnosticFeatureConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            exclude: Vec::new(),
            ignore: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SemanticTokensConfig {
    pub enabled: bool,
}

impl Default for SemanticTokensConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone)]
pub struct LoadedAblConfig {
    pub config: AblConfig,
    pub path: Option<PathBuf>,
}

pub fn find_workspace_root(params: &InitializeParams) -> Option<PathBuf> {
    if let Some(folders) = &params.workspace_folders {
        for folder in folders {
            if let Ok(path) = folder.uri.to_file_path() {
                return Some(path);
            }
        }
    }

    if let Some(root_uri) = &params.root_uri
        && let Ok(path) = root_uri.to_file_path()
    {
        return Some(path);
    }

    None
}

pub async fn load_from_workspace_root(root: Option<&Path>) -> LoadedAblConfig {
    let Some(root) = root else {
        return LoadedAblConfig {
            config: AblConfig::default(),
            path: None,
        };
    };

    let path = root.join("abl.toml");
    match tokio::fs::read_to_string(&path).await {
        Ok(contents) => match toml::from_str::<PartialAblConfig>(&contents) {
            Ok(root_partial) => LoadedAblConfig {
                config: load_with_inheritance(&path, root_partial).await,
                path: Some(path),
            },
            Err(_) => LoadedAblConfig {
                config: AblConfig::default(),
                path: Some(path),
            },
        },
        Err(err) if err.kind() == ErrorKind::NotFound => LoadedAblConfig {
            config: AblConfig::default(),
            path: Some(path),
        },
        Err(_) => LoadedAblConfig {
            config: AblConfig::default(),
            path: Some(path),
        },
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct PartialAblConfig {
    #[serde(default, deserialize_with = "deserialize_optional_string_or_vec")]
    inherits: Option<Vec<String>>,
    completion: Option<PartialCompletionConfig>,
    diagnostics: Option<PartialDiagnosticsConfig>,
    semantic_tokens: Option<PartialSemanticTokensConfig>,
    #[serde(default, deserialize_with = "deserialize_optional_string_or_vec")]
    dumpfile: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_string_or_vec")]
    propath: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct PartialCompletionConfig {
    enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct PartialDiagnosticsConfig {
    enabled: Option<bool>,
    unknown_variables: Option<PartialDiagnosticFeatureConfig>,
    unknown_functions: Option<PartialDiagnosticFeatureConfig>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct PartialDiagnosticFeatureConfig {
    enabled: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_string_or_vec")]
    exclude: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_string_or_vec")]
    ignore: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct PartialSemanticTokensConfig {
    enabled: Option<bool>,
}

async fn load_with_inheritance(path: &Path, root_partial: PartialAblConfig) -> AblConfig {
    let root_identity = path_identity(path);
    let mut partials = HashMap::<PathBuf, PartialAblConfig>::new();
    partials.insert(root_identity.clone(), root_partial);

    let mut visited = HashSet::<PathBuf>::new();
    let mut visiting = HashSet::<PathBuf>::new();
    let mut order = Vec::<PathBuf>::new();
    let mut stack = vec![(root_identity, false)];

    while let Some((current, exit)) = stack.pop() {
        if exit {
            visiting.remove(&current);
            visited.insert(current.clone());
            order.push(current);
            continue;
        }

        if visited.contains(&current) || visiting.contains(&current) {
            continue;
        }
        visiting.insert(current.clone());

        let current_partial = if let Some(cfg) = partials.get(&current).cloned() {
            cfg
        } else {
            match read_partial_config(&current).await {
                Some(cfg) => {
                    partials.insert(current.clone(), cfg.clone());
                    cfg
                }
                None => {
                    visiting.remove(&current);
                    visited.insert(current);
                    continue;
                }
            }
        };

        stack.push((current.clone(), true));

        if let Some(inherits) = current_partial.inherits {
            for inherited in inherits.iter().rev() {
                let inherited_path = resolve_inherited_path(&current, inherited);
                let inherited_identity = path_identity(&inherited_path);
                if visited.contains(&inherited_identity) || visiting.contains(&inherited_identity) {
                    continue;
                }

                if let std::collections::hash_map::Entry::Vacant(entry) =
                    partials.entry(inherited_identity.clone())
                    && let Some(cfg) = read_partial_config(&inherited_identity).await
                {
                    entry.insert(cfg);
                    stack.push((inherited_identity, false));
                }
            }
        }
    }

    let mut merged = AblConfig::default();
    for config_path in order {
        if let Some(partial) = partials.get(&config_path) {
            merge_partial_into(&mut merged, partial);
        }
    }
    merged
}

async fn read_partial_config(path: &Path) -> Option<PartialAblConfig> {
    let contents = tokio::fs::read_to_string(path).await.ok()?;
    toml::from_str::<PartialAblConfig>(&contents).ok()
}

fn resolve_inherited_path(current_config_path: &Path, inherited: &str) -> PathBuf {
    let inherited_path = PathBuf::from(inherited);
    if inherited_path.is_absolute() {
        inherited_path
    } else {
        current_config_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(inherited_path)
    }
}

fn path_identity(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn merge_partial_into(base: &mut AblConfig, partial: &PartialAblConfig) {
    if let Some(completion) = &partial.completion
        && let Some(enabled) = completion.enabled
    {
        base.completion.enabled = enabled;
    }

    if let Some(diagnostics) = &partial.diagnostics {
        if let Some(enabled) = diagnostics.enabled {
            base.diagnostics.enabled = enabled;
        }
        if let Some(unknown_variables) = &diagnostics.unknown_variables {
            if let Some(enabled) = unknown_variables.enabled {
                base.diagnostics.unknown_variables.enabled = enabled;
            }
            if let Some(exclude) = &unknown_variables.exclude {
                base.diagnostics.unknown_variables.exclude = exclude.clone();
            }
            if let Some(ignore) = &unknown_variables.ignore {
                base.diagnostics.unknown_variables.ignore = ignore.clone();
            }
        }
        if let Some(unknown_functions) = &diagnostics.unknown_functions {
            if let Some(enabled) = unknown_functions.enabled {
                base.diagnostics.unknown_functions.enabled = enabled;
            }
            if let Some(exclude) = &unknown_functions.exclude {
                base.diagnostics.unknown_functions.exclude = exclude.clone();
            }
            if let Some(ignore) = &unknown_functions.ignore {
                base.diagnostics.unknown_functions.ignore = ignore.clone();
            }
        }
    }

    if let Some(semantic_tokens) = &partial.semantic_tokens
        && let Some(enabled) = semantic_tokens.enabled
    {
        base.semantic_tokens.enabled = enabled;
    }

    if let Some(dumpfile) = &partial.dumpfile {
        base.dumpfile = dumpfile.clone();
    }
    if let Some(propath) = &partial.propath {
        base.propath = propath.clone();
    }
}

fn deserialize_dumpfile<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_optional_string_or_vec(deserializer).map(|v| v.unwrap_or_default())
}

fn deserialize_propath<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_optional_string_or_vec(deserializer).map(|v| v.unwrap_or_default())
}

fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_optional_string_or_vec(deserializer).map(|v| v.unwrap_or_default())
}

fn deserialize_optional_string_or_vec<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        Single(String),
        Multiple(Vec<String>),
    }

    let parsed = Option::<StringOrVec>::deserialize(deserializer)?;
    Ok(parsed.map(|v| match v {
        StringOrVec::Single(path) => vec![path],
        StringOrVec::Multiple(paths) => paths,
    }))
}

#[cfg(test)]
mod tests {
    use super::{AblConfig, load_from_workspace_root};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parses_dumpfile_and_propath_as_single_string() {
        let cfg: AblConfig = toml::from_str(
            r#"
dumpfile = "database.df"
propath = "src/includes"
"#,
        )
        .expect("parse config");

        assert_eq!(cfg.dumpfile, vec!["database.df"]);
        assert_eq!(cfg.propath, vec!["src/includes"]);
    }

    #[test]
    fn parses_dumpfile_and_propath_as_arrays() {
        let cfg: AblConfig = toml::from_str(
            r#"
dumpfile = ["a.df", "b.df"]
propath = ["/global/a", "relative/includes"]
"#,
        )
        .expect("parse config");

        assert_eq!(cfg.dumpfile, vec!["a.df", "b.df"]);
        assert_eq!(cfg.propath, vec!["/global/a", "relative/includes"]);
    }

    #[test]
    fn parses_diagnostic_feature_excludes() {
        let cfg: AblConfig = toml::from_str(
            r#"
[diagnostics.unknown_variables]
exclude = ["legacy/*.p", "tmp/**/*.p"]
ignore = ["BatchRun", "Today"]

[diagnostics.unknown_functions]
enabled = false
exclude = "special.p"
ignore = "custom_func"
"#,
        )
        .expect("parse config");

        assert_eq!(
            cfg.diagnostics.unknown_variables.exclude,
            vec!["legacy/*.p", "tmp/**/*.p"]
        );
        assert_eq!(
            cfg.diagnostics.unknown_variables.ignore,
            vec!["BatchRun", "Today"]
        );
        assert!(!cfg.diagnostics.unknown_functions.enabled);
        assert_eq!(cfg.diagnostics.unknown_functions.exclude, vec!["special.p"]);
        assert_eq!(
            cfg.diagnostics.unknown_functions.ignore,
            vec!["custom_func"]
        );
    }

    #[tokio::test]
    async fn loads_inherited_config_and_applies_child_overrides() {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let base_dir = std::env::temp_dir().join(format!("abl-ls-config-{ts}"));
        std::fs::create_dir_all(&base_dir).expect("create temp dir");

        let parent = base_dir.join("base.toml");
        let child = base_dir.join("abl.toml");

        std::fs::write(
            &parent,
            r#"
dumpfile = "parent.df"
propath = ["parent/includes"]

[completion]
enabled = false

[diagnostics]
enabled = false

[diagnostics.unknown_variables]
ignore = ["PARENT-GLOBAL"]
"#,
        )
        .expect("write parent config");

        std::fs::write(
            &child,
            r#"
inherits = "base.toml"
propath = ["child/includes"]

[diagnostics.unknown_variables]
ignore = ["CHILD-GLOBAL"]
"#,
        )
        .expect("write child config");

        let loaded = load_from_workspace_root(Some(&base_dir)).await;
        assert!(!loaded.config.completion.enabled);
        assert!(!loaded.config.diagnostics.enabled);
        assert_eq!(
            loaded.config.diagnostics.unknown_variables.ignore,
            vec!["CHILD-GLOBAL"]
        );
        assert_eq!(loaded.config.dumpfile, vec!["parent.df"]);
        assert_eq!(loaded.config.propath, vec!["child/includes"]);

        let _ = std::fs::remove_dir_all(&base_dir);
    }
}
