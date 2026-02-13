use log::debug;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;
use crate::handlers::diagnostics::on_change;
use crate::utils::text_sync::apply_content_changes;

const DID_CHANGE_DIAG_DEBOUNCE_MS: u64 = 200;

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
            true,
            0,
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

        self.schedule_on_change(
            uri,
            params.text_document.version,
            new_text,
            false,
            DID_CHANGE_DIAG_DEBOUNCE_MS,
        )
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
            self.schedule_on_change(params.text_document.uri, version, text, true, 0)
                .await;
        }
        debug!("file saved!");
    }

    pub async fn handle_did_close(&self, params: DidCloseTextDocumentParams) {
        if let Some(task) = self.take_document_diag_task(&params.text_document.uri) {
            task.handle.abort();
        }
        self.documents.remove(&params.text_document.uri);
        debug!("file closed!");
    }

    async fn schedule_on_change(
        &self,
        uri: Url,
        version: i32,
        text: String,
        include_semantic_diags: bool,
        debounce_ms: u64,
    ) {
        let backend = self.clone();
        let task_uri = uri.clone();
        let handle = tokio::spawn(async move {
            if debounce_ms > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(debounce_ms)).await;
            }
            on_change(&backend, task_uri, version, text, include_semantic_diags).await;
        });
        self.try_set_document_diag_task(&uri, include_semantic_diags, version, handle);
    }
}
