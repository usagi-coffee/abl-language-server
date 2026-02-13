use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::definitions::collect_definition_symbols;
use crate::analysis::functions::{find_function_signature, find_function_signature_from_includes};
use crate::analysis::hover::{
    find_db_field_matches, find_local_table_field_hover, function_signature_hover, markdown_hover,
    symbol_at_offset,
};
use crate::analysis::schema::normalize_lookup_key;
use crate::analysis::schema_lookup::has_schema_key;
use crate::backend::Backend;
use crate::utils::position::{
    ascii_ident_at_or_before, ascii_ident_or_dash_at_or_before, lsp_pos_to_utf8_byte_offset,
};

impl Backend {
    pub async fn handle_hover(&self, params: HoverParams) -> Result<Option<Hover>> {
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

        let offset = match lsp_pos_to_utf8_byte_offset(&text, pos) {
            Some(o) => o,
            None => return Ok(None),
        };
        let symbol = match symbol_at_offset(tree.root_node(), &text, offset).or_else(|| {
            ascii_ident_or_dash_at_or_before(&text, offset)
                .or_else(|| ascii_ident_at_or_before(&text, offset))
        }) {
            Some(s) => s,
            None => return Ok(None),
        };
        let symbol_upper = normalize_lookup_key(&symbol, true);

        if let Some(sig) = find_function_signature(tree.root_node(), text.as_bytes(), &symbol) {
            return Ok(Some(function_signature_hover(&sig)));
        }
        if let Some(sig) = find_function_signature_from_includes(
            self,
            &uri,
            &text,
            tree.root_node(),
            offset,
            &symbol,
        )
        .await
        {
            return Ok(Some(function_signature_hover(&sig)));
        }

        let mut defs = Vec::new();
        collect_definition_symbols(tree.root_node(), text.as_bytes(), &mut defs);
        if let Some(def) = defs
            .into_iter()
            .find(|d| d.label.eq_ignore_ascii_case(&symbol))
        {
            return Ok(Some(markdown_hover(format!(
                "**{}**\n\nType: `{}`",
                def.label, def.detail
            ))));
        }

        let mut buffers = Vec::new();
        collect_buffer_mappings(tree.root_node(), text.as_bytes(), &mut buffers);
        if let Some(buf) = buffers
            .into_iter()
            .find(|b| b.alias.eq_ignore_ascii_case(&symbol))
        {
            return Ok(Some(markdown_hover(format!(
                "**Buffer** `{}`\n\nFor table: `{}`",
                buf.alias, buf.table
            ))));
        }

        if has_schema_key(&self.db_table_definitions, &symbol_upper) {
            return Ok(Some(markdown_hover(format!("**DB Table** `{}`", symbol))));
        }

        if let Some(local_field_hover) =
            find_local_table_field_hover(tree.root_node(), &text, offset)
        {
            return Ok(Some(local_field_hover));
        }

        let field_matches = find_db_field_matches(&self.db_fields_by_table, &symbol_upper);
        if !field_matches.is_empty() {
            if field_matches.len() == 1 {
                let m = &field_matches[0];
                let mut lines = vec![format!("**DB Field** `{}`", m.field.name)];
                lines.push(format!("Table: `{}`", m.table));
                if let Some(ty) = &m.field.field_type {
                    lines.push(format!("Type: `{}`", ty));
                }
                if let Some(label) = &m.field.label {
                    lines.push(format!("Label: {}", label));
                }
                if let Some(format) = &m.field.format {
                    lines.push(format!("Format: {}", format));
                }
                if let Some(desc) = &m.field.description {
                    lines.push(format!("Description: {}", desc));
                }
                return Ok(Some(markdown_hover(lines.join("\n\n"))));
            }

            let preview = field_matches
                .iter()
                .take(8)
                .map(|m| format!("- `{}`", m.table))
                .collect::<Vec<_>>()
                .join("\n");
            let suffix = if field_matches.len() > 8 {
                "\n- ..."
            } else {
                ""
            };
            return Ok(Some(markdown_hover(format!(
                "**DB Field** `{}`\n\nFound in tables:\n{}{}",
                symbol, preview, suffix
            ))));
        }

        if has_schema_key(&self.db_index_definitions, &symbol_upper) {
            return Ok(Some(markdown_hover(format!("**DB Index** `{}`", symbol))));
        }

        Ok(None)
    }
}
