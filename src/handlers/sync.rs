use log::debug;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;
use crate::handlers::diagnostics::on_change;

impl Backend {
    pub async fn handle_did_open(&self, params: DidOpenTextDocumentParams) {
        on_change(self, params.text_document.uri, params.text_document.text).await;
        debug!("file opened!");
    }

    pub async fn handle_did_change(&self, params: DidChangeTextDocumentParams) {
        on_change(
            self,
            params.text_document.uri,
            params.content_changes[0].text.clone(),
        )
        .await;
        debug!("changed!");
    }

    pub async fn handle_did_save(&self, params: DidSaveTextDocumentParams) {
        self.maybe_reload_config_for_uri(&params.text_document.uri)
            .await;
        self.maybe_reload_db_tables_for_uri(&params.text_document.uri)
            .await;
        debug!("file saved!");
    }

    pub async fn handle_did_close(&self, _params: DidCloseTextDocumentParams) {
        debug!("file closed!");
    }
}
