use serde::Deserialize;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tower_lsp::lsp_types::InitializeParams;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct AblConfig {
    pub completion: CompletionConfig,
    pub diagnostics: DiagnosticsConfig,
    #[serde(default, deserialize_with = "deserialize_dumpfile")]
    pub dumpfile: Vec<String>,
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
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum DumpFileConfig {
        Single(String),
        Multiple(Vec<String>),
    }

    let parsed = Option::<DumpFileConfig>::deserialize(deserializer)?;
    Ok(match parsed {
        None => Vec::new(),
        Some(DumpFileConfig::Single(path)) => vec![path],
        Some(DumpFileConfig::Multiple(paths)) => paths,
    })
}
