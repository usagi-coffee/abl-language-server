use serde::Deserialize;
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
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self { enabled: true }
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
        Ok(contents) => match toml::from_str::<AblConfig>(&contents) {
            Ok(config) => LoadedAblConfig {
                config,
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

fn deserialize_dumpfile<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_string_or_vec(deserializer)
}

fn deserialize_propath<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_string_or_vec(deserializer)
}

fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
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
    Ok(match parsed {
        None => Vec::new(),
        Some(StringOrVec::Single(path)) => vec![path],
        Some(StringOrVec::Multiple(paths)) => paths,
    })
}

#[cfg(test)]
mod tests {
    use super::AblConfig;

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
}
