use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::definitions::collect_definition_symbols;
use crate::backend::Backend;
use crate::utils::position::{ascii_ident_prefix, lsp_pos_to_utf8_byte_offset};

impl Backend {
    pub async fn handle_completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let text = match self.docs.get(&uri) {
            Some(t) => t,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };
        let tree = match self.trees.get(&uri) {
            Some(t) => t,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };

        let offset = match lsp_pos_to_utf8_byte_offset(&text, pos) {
            Some(o) => o,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };

        let prefix = ascii_ident_prefix(&text, offset);

        let mut symbols = Vec::new();
        collect_definition_symbols(tree.root_node(), text.as_bytes(), &mut symbols);

        symbols.sort_by(|a, b| {
            a.label
                .to_ascii_uppercase()
                .cmp(&b.label.to_ascii_uppercase())
                .then(a.label.cmp(&b.label))
                .then(a.detail.cmp(&b.detail))
        });
        symbols.dedup_by(|a, b| a.label.eq_ignore_ascii_case(&b.label) && a.kind == b.kind);

        let pref_up = prefix.to_ascii_uppercase();
        let items = symbols
            .into_iter()
            .filter(|s| s.label.to_ascii_uppercase().starts_with(&pref_up))
            .map(|s| CompletionItem {
                label: s.label.clone(),
                kind: Some(s.kind),
                detail: Some(s.detail.to_string()),
                insert_text: Some(s.label),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Ok(Some(CompletionResponse::Array(items)))
    }
}
