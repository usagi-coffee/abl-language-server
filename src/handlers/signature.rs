use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{SignatureHelp, SignatureHelpParams};

use crate::analysis::functions::{find_function_signature, find_function_signature_from_includes};
use crate::analysis::signature::{call_context_at_offset, to_signature_information};
use crate::backend::Backend;
use crate::utils::position::lsp_pos_to_utf8_byte_offset;

impl Backend {
    pub async fn handle_signature_help(
        &self,
        params: SignatureHelpParams,
    ) -> Result<Option<SignatureHelp>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let text = match self.get_document_text(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        let tree = match self.get_document_tree_or_parse(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let Some(offset) = lsp_pos_to_utf8_byte_offset(&text, pos) else {
            return Ok(None);
        };

        let Some(call) = call_context_at_offset(tree.root_node(), text.as_bytes(), offset) else {
            return Ok(None);
        };

        let local_sig = find_function_signature(tree.root_node(), text.as_bytes(), &call.name);
        let sig = match local_sig {
            Some(sig) => sig,
            None => match find_function_signature_from_includes(
                self,
                &uri,
                &text,
                tree.root_node(),
                offset,
                &call.name,
            )
            .await
            {
                Some(sig) => sig,
                None => return Ok(None),
            },
        };

        let sig_info = to_signature_information(&sig);
        let active_param = if sig.params.is_empty() {
            None
        } else {
            Some((call.active_param.min(sig.params.len().saturating_sub(1))) as u32)
        };

        Ok(Some(SignatureHelp {
            signatures: vec![sig_info],
            active_signature: Some(0),
            active_parameter: active_param,
        }))
    }
}
