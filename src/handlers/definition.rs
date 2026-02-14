use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::definition::{
    resolve_buffer_alias_table_location, resolve_include_directive_location,
    resolve_include_function_location, resolve_local_definition_location,
    resolve_preprocessor_define_match,
};
use crate::analysis::schema::normalize_lookup_key;
use crate::analysis::schema_lookup::lookup_schema_location;
use crate::backend::Backend;
use crate::utils::position::{
    ascii_ident_at_or_before, ascii_ident_or_dash_at_or_before, lsp_pos_to_utf8_byte_offset,
    preprocessor_name_at_or_before,
};

impl Backend {
    pub async fn handle_goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let text = match self.get_document_text(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let offset = match lsp_pos_to_utf8_byte_offset(&text, pos) {
            Some(o) => o,
            None => return Ok(None),
        };

        let tree = match self.get_document_tree_or_parse(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        if let Some(macro_name) = preprocessor_name_at_or_before(&text, offset)
            && let Some(matched) = resolve_preprocessor_define_match(
                self,
                &uri,
                &text,
                tree.root_node(),
                &macro_name,
                offset,
            )
            .await
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(matched.location)));
        }

        if let Some(location) =
            resolve_include_directive_location(self, &uri, &text, tree.root_node(), offset).await
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        let symbol = match ascii_ident_or_dash_at_or_before(&text, offset)
            .or_else(|| ascii_ident_at_or_before(&text, offset))
        {
            Some(s) => s,
            None => return Ok(None),
        };
        let symbol_upper = normalize_lookup_key(&symbol, false);

        if let Some(location) = resolve_buffer_alias_table_location(
            self,
            &uri,
            tree.root_node(),
            text.as_bytes(),
            &symbol_upper,
            offset,
        ) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        if let Some(location) = resolve_local_definition_location(
            &uri,
            tree.root_node(),
            text.as_bytes(),
            &symbol,
            offset,
        ) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        if let Some(location) =
            resolve_include_function_location(self, &uri, &text, tree.root_node(), &symbol, offset)
                .await
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        if let Some(location) = lookup_schema_location(&self.db_table_definitions, &symbol_upper) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        if let Some(location) = lookup_schema_location(&self.db_field_definitions, &symbol_upper) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        if let Some(location) = lookup_schema_location(&self.db_index_definitions, &symbol_upper) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        Ok(None)
    }
}
