use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;
use crate::utils::position::{ascii_ident_at_or_before, lsp_pos_to_utf8_byte_offset};

impl Backend {
    pub async fn handle_references(
        &self,
        params: ReferenceParams,
    ) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let text = match self.docs.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let offset = match lsp_pos_to_utf8_byte_offset(&text, pos) {
            Some(o) => o,
            None => return Ok(None),
        };

        let symbol = match ascii_ident_at_or_before(&text, offset) {
            Some(s) => s.to_ascii_uppercase(),
            None => return Ok(None),
        };

        let locations = self
            .db_table_definitions
            .get(&symbol)
            .map(|entry| entry.value().clone())
            .unwrap_or_default();
        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(locations))
        }
    }
}
