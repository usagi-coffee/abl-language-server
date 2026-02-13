use dashmap::{DashMap, DashSet};
use log::{debug, warn};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex as AsyncMutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tree_sitter::{Language, Parser, Tree};

use crate::config::{AblConfig, find_workspace_root, load_from_workspace_root};

#[derive(Clone)]
pub struct DbFieldInfo {
    pub name: String,
    pub field_type: Option<String>,
    pub format: Option<String>,
    pub label: Option<String>,
    pub description: Option<String>,
}

pub struct BackendState {
    pub abl_language: Language,
    pub abl_parsers: DashMap<Url, StdMutex<Parser>>,
    pub df_parser: AsyncMutex<Parser>,
    pub trees: DashMap<Url, Tree>,
    pub docs: DashMap<Url, String>,
    pub doc_versions: DashMap<Url, i32>,
    pub workspace_root: AsyncMutex<Option<std::path::PathBuf>>,
    pub config: AsyncMutex<AblConfig>,
    pub db_tables: DashSet<String>,
    pub db_table_labels: DashMap<String, String>,
    pub db_table_definitions: DashMap<String, Vec<Location>>,
    pub db_field_definitions: DashMap<String, Vec<Location>>,
    pub db_index_definitions: DashMap<String, Vec<Location>>,
    pub db_fields_by_table: DashMap<String, Vec<DbFieldInfo>>,
    pub diag_tasks: AsyncMutex<HashMap<Url, tokio::task::JoinHandle<()>>>,
}

#[derive(Clone)]
pub struct Backend {
    pub client: Client,
    pub state: Arc<BackendState>,
}

impl Deref for Backend {
    type Target = BackendState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let root = find_workspace_root(&params);
        {
            let mut workspace_root = self.workspace_root.lock().await;
            *workspace_root = root;
        }
        self.reload_workspace_config().await;
        let semantic_tokens_enabled = self.config.lock().await.semantic_tokens.enabled;

        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,

            capabilities: ServerCapabilities {
                document_formatting_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: None,
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: Some(vec![",".to_string()]),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                execute_command_provider: None,
                workspace: None,
                semantic_tokens_provider: if semantic_tokens_enabled {
                    Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: vec![SemanticTokenType::TYPE],
                                token_modifiers: vec![],
                            },
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            work_done_progress_options: WorkDoneProgressOptions::default(),
                        },
                    ))
                } else {
                    None
                },
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: None,
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        debug!("initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.handle_did_open(params).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.handle_did_change(params).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.handle_did_save(params).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.handle_did_close(params).await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        self.handle_goto_definition(params).await
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        self.handle_references(params).await
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        self.handle_hover(params).await
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        self.handle_semantic_tokens_full(params).await
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        self.handle_semantic_tokens_range(params).await
    }

    async fn inlay_hint(
        &self,
        _params: tower_lsp::lsp_types::InlayHintParams,
    ) -> Result<Option<Vec<InlayHint>>> {
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        self.handle_completion(params).await
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        self.handle_signature_help(params).await
    }

    async fn rename(&self, _params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Ok(None)
    }

    async fn formatting(&self, _params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        Ok(None)
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.reload_workspace_config().await;
        debug!("configuration changed!");
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        if let Some(folder) = params.event.added.first() {
            if let Ok(path) = folder.uri.to_file_path() {
                let mut workspace_root = self.workspace_root.lock().await;
                *workspace_root = Some(path);
            }
        } else {
            let mut workspace_root = self.workspace_root.lock().await;
            *workspace_root = None;
        }
        self.reload_workspace_config().await;
        debug!("workspace folders changed!");
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        for change in params.changes {
            if is_abl_toml_uri(&change.uri) {
                self.reload_workspace_config().await;
                break;
            } else if self.is_configured_dumpfile_uri(&change.uri).await {
                self.reload_db_tables_from_current_config().await;
                break;
            }
        }
        debug!("watched files have changed!");
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        debug!("command executed!");

        Ok(None)
    }
}

impl Backend {
    pub fn new_abl_parser(&self) -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(&self.abl_language)
            .expect("Error loading abl parser");
        parser
    }

    pub async fn reload_workspace_config(&self) {
        let workspace_root = self.workspace_root.lock().await.clone();
        let loaded = load_from_workspace_root(workspace_root.as_deref()).await;

        let dumpfiles = loaded.config.dumpfile.clone();
        let mut config = self.config.lock().await;
        *config = loaded.config;
        drop(config);

        self.reload_db_tables(workspace_root.as_deref(), &dumpfiles)
            .await;

        if let Some(path) = loaded.path {
            if Path::new(&path).exists() {
                debug!("loaded workspace config from {}", path.display());
            } else {
                debug!(
                    "workspace config not found, using defaults (expected path: {})",
                    path.display()
                );
            }
        } else {
            warn!("workspace root is unknown; using default config");
        }
    }

    pub async fn maybe_reload_config_for_uri(&self, uri: &Url) {
        if is_abl_toml_uri(uri) {
            self.reload_workspace_config().await;
        }
    }

    pub async fn maybe_reload_db_tables_for_uri(&self, uri: &Url) {
        if self.is_configured_dumpfile_uri(uri).await {
            self.reload_db_tables_from_current_config().await;
        }
    }

    pub async fn resolve_include_path_for(
        &self,
        current_file: &Path,
        include: &str,
    ) -> Option<std::path::PathBuf> {
        let workspace_root = self.workspace_root.lock().await.clone();
        let propath = self.config.lock().await.propath.clone();
        resolve_include_path(workspace_root.as_deref(), &propath, current_file, include)
    }

    async fn reload_db_tables(&self, workspace_root: Option<&Path>, dumpfiles: &[String]) {
        let mut tables = HashSet::<String>::new();
        let mut table_labels = HashMap::<String, String>::new();
        let mut definitions = HashMap::<String, Vec<Location>>::new();
        let mut field_definitions = HashMap::<String, Vec<Location>>::new();
        let mut index_definitions = HashMap::<String, Vec<Location>>::new();
        let mut fields_by_table = HashMap::<String, Vec<DbFieldInfo>>::new();
        for dumpfile in dumpfiles {
            let Some(path) = resolve_dumpfile_path(workspace_root, dumpfile) else {
                continue;
            };
            let Ok(contents) = tokio::fs::read_to_string(&path).await else {
                continue;
            };

            let tree = {
                let mut parser = self.df_parser.lock().await;
                parser.parse(&contents, None)
            };
            let Some(tree) = tree else {
                continue;
            };

            crate::analysis::df::collect_df_table_names(
                tree.root_node(),
                contents.as_bytes(),
                &mut tables,
            );
            let Some(uri) = Url::from_file_path(&path).ok() else {
                continue;
            };
            let mut sites = Vec::new();
            crate::analysis::df::collect_df_table_sites(
                tree.root_node(),
                contents.as_bytes(),
                &mut sites,
            );
            for site in sites {
                let key = site.name.to_ascii_uppercase();
                tables.insert(key.clone());
                table_labels.entry(key.clone()).or_insert(site.name);
                definitions.entry(key).or_default().push(Location {
                    uri: uri.clone(),
                    range: site.range,
                });
            }

            let mut field_sites = Vec::new();
            crate::analysis::df::collect_df_field_sites(
                tree.root_node(),
                contents.as_bytes(),
                &mut field_sites,
            );
            for site in field_sites {
                field_definitions
                    .entry(site.name.to_ascii_uppercase())
                    .or_default()
                    .push(Location {
                        uri: uri.clone(),
                        range: site.range,
                    });
            }

            let mut table_fields = Vec::new();
            crate::analysis::df::collect_df_table_fields(
                tree.root_node(),
                contents.as_bytes(),
                &mut table_fields,
            );
            for pair in table_fields {
                fields_by_table
                    .entry(pair.table.to_ascii_uppercase())
                    .or_default()
                    .push(DbFieldInfo {
                        name: pair.field,
                        field_type: pair.field_type,
                        format: pair.format,
                        label: pair.label,
                        description: pair.description,
                    });
            }

            let mut index_sites = Vec::new();
            crate::analysis::df::collect_df_index_sites(
                tree.root_node(),
                contents.as_bytes(),
                &mut index_sites,
            );
            for site in index_sites {
                index_definitions
                    .entry(site.name.to_ascii_uppercase())
                    .or_default()
                    .push(Location {
                        uri: uri.clone(),
                        range: site.range,
                    });
            }
        }

        self.db_tables.clear();
        for table in tables {
            self.db_tables.insert(table);
        }
        self.db_table_definitions.clear();
        for (k, v) in definitions {
            self.db_table_definitions.insert(k, v);
        }
        self.db_table_labels.clear();
        for (k, v) in table_labels {
            self.db_table_labels.insert(k, v);
        }
        self.db_field_definitions.clear();
        for (k, v) in field_definitions {
            self.db_field_definitions.insert(k, v);
        }
        self.db_index_definitions.clear();
        for (k, v) in index_definitions {
            self.db_index_definitions.insert(k, v);
        }
        for fields in fields_by_table.values_mut() {
            fields.sort_by(|a, b| {
                a.name
                    .to_ascii_uppercase()
                    .cmp(&b.name.to_ascii_uppercase())
                    .then(a.name.cmp(&b.name))
            });
            fields.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
        }
        self.db_fields_by_table.clear();
        for (k, v) in fields_by_table {
            self.db_fields_by_table.insert(k, v);
        }
        debug!(
            "loaded schema from dumpfile(s): tables={}, fields={}, indexes={}, table_field_sets={}",
            self.db_tables.len(),
            self.db_field_definitions.len(),
            self.db_index_definitions.len(),
            self.db_fields_by_table.len()
        );
    }

    async fn reload_db_tables_from_current_config(&self) {
        let workspace_root = self.workspace_root.lock().await.clone();
        let dumpfiles = self.config.lock().await.dumpfile.clone();
        self.reload_db_tables(workspace_root.as_deref(), &dumpfiles)
            .await;
    }

    async fn is_configured_dumpfile_uri(&self, uri: &Url) -> bool {
        let Ok(uri_path) = uri.to_file_path() else {
            return false;
        };

        let workspace_root = self.workspace_root.lock().await.clone();
        let dumpfiles = self.config.lock().await.dumpfile.clone();
        dumpfiles.iter().any(|dumpfile| {
            resolve_dumpfile_path(workspace_root.as_deref(), dumpfile)
                .map(|p| p == uri_path)
                .unwrap_or(false)
        })
    }
}

fn is_abl_toml_uri(uri: &Url) -> bool {
    uri.to_file_path()
        .ok()
        .and_then(|path| path.file_name().map(|name| name == "abl.toml"))
        .unwrap_or(false)
}

fn resolve_dumpfile_path(
    workspace_root: Option<&Path>,
    dumpfile: &str,
) -> Option<std::path::PathBuf> {
    resolve_config_path(workspace_root, dumpfile)
}

fn resolve_include_path(
    workspace_root: Option<&Path>,
    propath: &[String],
    current_file: &Path,
    include: &str,
) -> Option<std::path::PathBuf> {
    let candidate = std::path::PathBuf::from(include);
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

fn resolve_config_path(workspace_root: Option<&Path>, value: &str) -> Option<std::path::PathBuf> {
    let candidate = std::path::PathBuf::from(value);
    if candidate.is_absolute() {
        return Some(candidate);
    }
    workspace_root.map(|root| root.join(candidate))
}

#[cfg(test)]
mod tests {
    use super::resolve_include_path;
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
}
