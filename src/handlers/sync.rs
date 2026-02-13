use log::debug;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;
use crate::handlers::diagnostics::on_change;
use crate::utils::position::lsp_pos_to_utf8_byte_offset;

const DID_CHANGE_DIAG_DEBOUNCE_MS: u64 = 200;

impl Backend {
    pub async fn handle_did_open(&self, params: DidOpenTextDocumentParams) {
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
        let current = self
            .docs
            .get(&uri)
            .map(|doc| doc.value().clone())
            .unwrap_or_default();
        let Some(new_text) = apply_content_changes(current, &params.content_changes) else {
            return;
        };

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
            self.schedule_on_change(params.text_document.uri, version, text, true, 0)
                .await;
        }
        debug!("file saved!");
    }

    pub async fn handle_did_close(&self, params: DidCloseTextDocumentParams) {
        if let Some(handle) = self
            .diag_tasks
            .lock()
            .await
            .remove(&params.text_document.uri)
        {
            handle.abort();
        }
        self.docs.remove(&params.text_document.uri);
        self.trees.remove(&params.text_document.uri);
        self.doc_versions.remove(&params.text_document.uri);
        self.abl_parsers.remove(&params.text_document.uri);
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
        let mut tasks = self.diag_tasks.lock().await;
        if let Some(prev) = tasks.remove(&uri) {
            prev.abort();
        }

        let backend = self.clone();
        let task_uri = uri.clone();
        let handle = tokio::spawn(async move {
            if debounce_ms > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(debounce_ms)).await;
            }
            on_change(&backend, task_uri, version, text, include_semantic_diags).await;
        });
        tasks.insert(uri, handle);
    }
}

fn apply_content_changes(
    mut text: String,
    changes: &[TextDocumentContentChangeEvent],
) -> Option<String> {
    if changes.is_empty() {
        return Some(text);
    }

    for change in changes {
        match change.range {
            None => {
                text = change.text.clone();
            }
            Some(range) => {
                let start = lsp_pos_to_utf8_byte_offset(&text, range.start)?;
                let end = lsp_pos_to_utf8_byte_offset(&text, range.end)?;
                if start > end || end > text.len() {
                    return None;
                }
                text.replace_range(start..end, &change.text);
            }
        }
    }

    Some(text)
}

#[cfg(test)]
mod tests {
    use super::apply_content_changes;
    use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    #[test]
    fn applies_full_text_change() {
        let out = apply_content_changes(
            "abc".to_string(),
            &[TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: "xyz".to_string(),
            }],
        )
        .expect("updated text");
        assert_eq!(out, "xyz");
    }

    #[test]
    fn applies_incremental_change() {
        let out = apply_content_changes(
            "test_a".to_string(),
            &[TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(0, 5), Position::new(0, 6))),
                range_length: None,
                text: "b".to_string(),
            }],
        )
        .expect("updated text");
        assert_eq!(out, "test_b");
    }
}
