use dashmap::DashMap;
use log::{debug, warn};
use serde_json::Value;
use std::path::Path;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tree_sitter::{Parser, Tree};

use crate::config::{AblConfig, find_workspace_root, load_from_workspace_root};

pub struct Backend {
    pub client: Client,
    pub parser: Mutex<Parser>,
    pub trees: DashMap<Url, Tree>,
    pub docs: DashMap<Url, String>,
    pub workspace_root: Mutex<Option<std::path::PathBuf>>,
    pub config: Mutex<AblConfig>,
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

        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,

            capabilities: ServerCapabilities {
                document_formatting_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: None,
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
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
                execute_command_provider: None,
                workspace: None,
                semantic_tokens_provider: None,
                definition_provider: None,
                references_provider: None,
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
        _params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }

    async fn references(&self, _params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        Ok(None)
    }

    async fn semantic_tokens_full(
        &self,
        _params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        Ok(None)
    }

    async fn semantic_tokens_range(
        &self,
        _params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        Ok(None)
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
    pub async fn reload_workspace_config(&self) {
        let workspace_root = self.workspace_root.lock().await.clone();
        let loaded = load_from_workspace_root(workspace_root.as_deref()).await;

        let mut config = self.config.lock().await;
        *config = loaded.config;

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
}

fn is_abl_toml_uri(uri: &Url) -> bool {
    uri.to_file_path()
        .ok()
        .and_then(|path| path.file_name().map(|name| name == "abl.toml"))
        .unwrap_or(false)
}
