use log::debug;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;
use crate::handlers::diagnostics::on_change;

impl Backend {
    pub async fn handle_did_open(&self, params: DidOpenTextDocumentParams) {
        on_change(
            self,
            params.text_document.uri,
            params.text_document.version,
            params.text_document.text,
            true,
        )
        .await;
        debug!("file opened!");
    }

    pub async fn handle_did_change(&self, params: DidChangeTextDocumentParams) {
        on_change(
            self,
            params.text_document.uri,
            params.text_document.version,
            params.content_changes[0].text.clone(),
            false,
        )
        .await;
        debug!("changed!");
    }

    pub async fn handle_did_save(&self, params: DidSaveTextDocumentParams) {
        self.maybe_reload_config_for_uri(&params.text_document.uri)
            .await;
        self.maybe_reload_db_tables_for_uri(&params.text_document.uri)
            .await;

        if let (Some(version), Some(text)) = (
            self.doc_versions
                .get(&params.text_document.uri)
                .map(|v| *v.value()),
            self.docs
                .get(&params.text_document.uri)
                .map(|t| t.value().clone()),
        ) {
            on_change(self, params.text_document.uri, version, text, true).await;
        }
        debug!("file saved!");
    }

    pub async fn handle_did_close(&self, params: DidCloseTextDocumentParams) {
        self.docs.remove(&params.text_document.uri);
        self.trees.remove(&params.text_document.uri);
        self.doc_versions.remove(&params.text_document.uri);
        self.abl_parsers.remove(&params.text_document.uri);
        debug!("file closed!");
    }
}
