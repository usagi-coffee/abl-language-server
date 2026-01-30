use log::debug;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::variables::collect_variable_decls;
use crate::backend::Backend;
use crate::utils::position::{ascii_ident_prefix, lsp_pos_to_utf8_byte_offset};

impl Backend {
    pub async fn handle_completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        debug!("{:?}", params);

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

        let mut vars = Vec::<String>::new();
        collect_variable_decls(tree.root_node(), text.as_bytes(), &mut vars);
        debug!("{:?}", vars);

        vars.sort();
        vars.dedup();

        let pref_up = prefix.to_ascii_uppercase();
        let items = vars
            .into_iter()
            .filter(|v| v.to_ascii_uppercase().starts_with(&pref_up))
            .map(|v| CompletionItem {
                label: v.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some("CHARACTER".to_string()),
                insert_text: Some(v),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Ok(Some(CompletionResponse::Array(items)))
    }
}
