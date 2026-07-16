use log::debug;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;
use crate::handlers::diagnostics::on_change;
use crate::utils::text_sync::apply_content_changes;

impl Backend {
    pub async fn handle_did_open(&self, params: DidOpenTextDocumentParams) {
        self.set_document_text_version(
            &params.text_document.uri,
            params.text_document.version,
            params.text_document.text.clone(),
            true,
        );
        self.schedule_on_change(
            params.text_document.uri,
            params.text_document.version,
            params.text_document.text,
        )
        .await;
        debug!("file opened!");
    }

    pub async fn handle_did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let current = self.get_document_text(&uri).unwrap_or_default();
        let Some(new_text) = apply_content_changes(current, &params.content_changes) else {
            return;
        };
        self.set_document_text_version(&uri, params.text_document.version, new_text.clone(), false);

        self.schedule_on_change(uri, params.text_document.version, new_text)
            .await;
        debug!("changed!");
    }

    pub async fn handle_did_save(&self, params: DidSaveTextDocumentParams) {
        self.invalidate_include_caches_for_uri(&params.text_document.uri);
        self.maybe_reload_config_for_uri(&params.text_document.uri)
            .await;
        self.maybe_reload_db_tables_for_uri(&params.text_document.uri)
            .await;

        if let (Some(version), Some(text)) = (
            self.get_document_version(&params.text_document.uri),
            self.get_document_text(&params.text_document.uri),
        ) {
            self.schedule_on_change(params.text_document.uri, version, text)
                .await;
        }
        debug!("file saved!");
    }

    pub async fn handle_did_close(&self, params: DidCloseTextDocumentParams) {
        if let Some(task) = self.take_document_diag_task(&params.text_document.uri) {
            task.abort();
        }
        self.documents.remove(&params.text_document.uri);
        debug!("file closed!");
    }

    async fn schedule_on_change(&self, uri: Url, version: i32, text: String) {
        let backend = self.clone();
        let task_uri = uri.clone();
        let handle = tokio::spawn(async move {
            on_change(&backend, task_uri, version, text).await;
        });
        self.replace_document_diag_task(&uri, handle);
    }
}

#[cfg(test)]
mod tests {
    use super::Backend;
    use crate::backend::BackendState;
    use crate::config::AblConfig;
    use dashmap::{DashMap, DashSet};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex as AsyncMutex;
    use tower_lsp::lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, TextDocumentContentChangeEvent,
        TextDocumentItem, Url, VersionedTextDocumentIdentifier,
    };
    use tower_lsp::{Client, LspService};

    fn test_backend() -> Backend {
        let (service, _socket) = LspService::build(|client: Client| Backend {
            client,
            state: Arc::new(BackendState {
                abl_language: tree_sitter_abl::LANGUAGE.into(),
                df_parser: AsyncMutex::new({
                    let mut parser = tree_sitter::Parser::new();
                    parser
                        .set_language(&tree_sitter_df::LANGUAGE.into())
                        .expect("set df language");
                    parser
                }),
                documents: DashMap::new(),
                workspace_root: AsyncMutex::new(None),
                config: AsyncMutex::new(AblConfig::default()),
                db_tables: DashSet::new(),
                db_sequences: DashSet::new(),
                db_table_labels: DashMap::new(),
                db_table_definitions: DashMap::new(),
                db_sequence_definitions: DashMap::new(),
                db_field_definitions: DashMap::new(),
                db_index_definitions: DashMap::new(),
                db_indexes_by_table: DashMap::new(),
                db_index_fields_by_table_index: DashMap::new(),
                db_fields_by_table: DashMap::new(),
                include_completion_cache: DashMap::new(),
                include_parse_cache: DashMap::new(),
                embedded_ambient_paths: AsyncMutex::new(None),
            }),
        })
        .finish();
        let backend = service.inner().clone();
        drop(service);
        backend
    }

    #[tokio::test]
    async fn completed_open_pass_does_not_block_next_change_pass() {
        let backend = test_backend();
        let uri = Url::parse("file:///diagnostics-after-change.p").expect("document uri");

        backend
            .handle_did_open(DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri.clone(),
                    language_id: "abl".to_string(),
                    version: 1,
                    text: "MESSAGE first.".to_string(),
                },
            })
            .await;

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let open_pass_finished = backend.documents.get(&uri).is_some_and(|doc| {
                    doc.diag_task
                        .as_ref()
                        .is_some_and(tokio::task::JoinHandle::is_finished)
                });
                if open_pass_finished {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("open diagnostics pass finished");

        backend
            .handle_did_change(DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: uri.clone(),
                    version: 2,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: "MESSAGE second.".to_string(),
                }],
            })
            .await;

        let change_task = backend
            .take_document_diag_task(&uri)
            .expect("change diagnostics task");
        tokio::time::timeout(Duration::from_secs(2), change_task)
            .await
            .expect("change diagnostics pass finished")
            .expect("change diagnostics task succeeded");

        let document = backend.documents.get(&uri).expect("open document");
        assert_eq!(document.version, 2);
        assert_eq!(document.tree_version, 2);
        assert_eq!(document.text, "MESSAGE second.");
    }
}
