use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::definitions::collect_definition_symbols;
use crate::backend::Backend;
use crate::utils::position::{
    ascii_ident_at_or_before, ascii_ident_or_dash_at_or_before, lsp_pos_to_utf8_byte_offset,
};

impl Backend {
    pub async fn handle_hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let text = match self.docs.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        let tree = match self.trees.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let offset = match lsp_pos_to_utf8_byte_offset(&text, pos) {
            Some(o) => o,
            None => return Ok(None),
        };
        let symbol = match ascii_ident_or_dash_at_or_before(&text, offset)
            .or_else(|| ascii_ident_at_or_before(&text, offset))
        {
            Some(s) => s,
            None => return Ok(None),
        };
        let symbol_upper = normalize_lookup_key(&symbol);

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

        let table_defs = self.db_table_definitions.lock().await;
        if table_defs.contains_key(&symbol_upper) {
            return Ok(Some(markdown_hover(format!("**DB Table** `{}`", symbol))));
        }
        drop(table_defs);

        let field_matches = self.find_db_field_matches(&symbol_upper).await;
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

        let index_defs = self.db_index_definitions.lock().await;
        if index_defs
            .keys()
            .any(|k| k.eq_ignore_ascii_case(&symbol_upper))
        {
            return Ok(Some(markdown_hover(format!("**DB Index** `{}`", symbol))));
        }

        Ok(None)
    }

    async fn find_db_field_matches(&self, field_upper: &str) -> Vec<DbFieldMatch> {
        let fields_by_table = self.db_fields_by_table.lock().await;
        let mut out = Vec::new();
        for (table, fields) in fields_by_table.iter() {
            for field in fields {
                if field.name.eq_ignore_ascii_case(field_upper) {
                    out.push(DbFieldMatch {
                        table: table.clone(),
                        field: field.clone(),
                    });
                }
            }
        }
        out
    }
}

#[derive(Clone)]
struct DbFieldMatch {
    table: String,
    field: crate::backend::DbFieldInfo,
}

fn markdown_hover(markdown: String) -> Hover {
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: markdown,
        }),
        range: None,
    }
}

fn normalize_lookup_key(symbol: &str) -> String {
    symbol
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
        .to_ascii_uppercase()
}
