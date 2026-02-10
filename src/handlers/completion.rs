use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::completion::{
    field_detail, field_documentation, lookup_case_insensitive_fields, qualifier_before_dot,
    text_has_dot_before_cursor,
};
use crate::analysis::definitions::collect_definition_symbols;
use crate::backend::Backend;
use crate::utils::position::{ascii_ident_prefix, lsp_pos_to_utf8_byte_offset};

struct CompletionCandidate {
    label: String,
    kind: CompletionItemKind,
    detail: String,
}

impl Backend {
    pub async fn handle_completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let trigger_is_dot = params
            .context
            .as_ref()
            .and_then(|ctx| ctx.trigger_character.as_deref())
            .map(|ch| ch == ".")
            .unwrap_or(false);

        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        if !self.config.lock().await.completion.enabled {
            return Ok(Some(CompletionResponse::Array(vec![])));
        }

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

        // Dot completion: table_or_buffer.<prefix>
        let dot_qualifier = qualifier_before_dot(&text, offset, &prefix).or_else(|| {
            if trigger_is_dot && !prefix.is_empty() {
                // Some clients trigger completion before '.' is reflected in document text.
                Some(prefix.clone())
            } else {
                None
            }
        });
        let field_prefix = if trigger_is_dot
            && dot_qualifier.is_some()
            && !text_has_dot_before_cursor(&text, offset)
        {
            String::new()
        } else {
            prefix.clone()
        };

        if let Some(qualifier) = dot_qualifier {
            let qualifier_upper = qualifier.to_ascii_uppercase();
            let mut table_upper = Some(qualifier_upper.clone());

            if !self.db_fields_by_table.contains_key(&qualifier_upper) {
                let mut mappings = Vec::new();
                collect_buffer_mappings(tree.root_node(), text.as_bytes(), &mut mappings);
                table_upper = mappings
                    .into_iter()
                    .find(|m| m.alias.eq_ignore_ascii_case(&qualifier_upper))
                    .map(|m| m.table.to_ascii_uppercase());
            }

            if let Some(table_key) = table_upper {
                let fields = lookup_case_insensitive_fields(&self.db_fields_by_table, &table_key);
                if let Some(fields) = fields {
                    let pref_up = field_prefix.to_ascii_uppercase();
                    let mut items = fields
                        .iter()
                        .filter(|f| f.name.to_ascii_uppercase().starts_with(&pref_up))
                        .map(|f| CompletionItem {
                            label: f.name.clone(),
                            kind: Some(CompletionItemKind::FIELD),
                            detail: Some(field_detail(f, &table_key)),
                            documentation: field_documentation(f),
                            insert_text: Some(f.name.clone()),
                            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                            ..Default::default()
                        })
                        .collect::<Vec<_>>();
                    items.sort_by(|a, b| {
                        a.label
                            .to_ascii_uppercase()
                            .cmp(&b.label.to_ascii_uppercase())
                            .then(a.label.cmp(&b.label))
                    });
                    items.dedup_by(|a, b| a.label.eq_ignore_ascii_case(&b.label));
                    return Ok(Some(CompletionResponse::Array(items)));
                }
            }
        }
        if trigger_is_dot {
            return Ok(Some(CompletionResponse::Array(vec![])));
        }

        let mut candidates = Vec::<CompletionCandidate>::new();

        let mut symbols = Vec::new();
        collect_definition_symbols(tree.root_node(), text.as_bytes(), &mut symbols);
        candidates.extend(
            symbols
                .into_iter()
                .filter(|s| s.start_byte <= offset)
                .map(|s| CompletionCandidate {
                    label: s.label,
                    kind: s.kind,
                    detail: s.detail,
                }),
        );

        let table_labels = &self.db_table_labels;
        candidates.extend(
            table_labels
                .iter()
                .map(|entry| entry.value().clone())
                .map(|label| CompletionCandidate {
                    label,
                    kind: CompletionItemKind::STRUCT,
                    detail: "DB table".to_string(),
                }),
        );

        candidates.sort_by(|a, b| {
            a.label
                .to_ascii_uppercase()
                .cmp(&b.label.to_ascii_uppercase())
                .then(a.label.cmp(&b.label))
                .then(a.detail.cmp(&b.detail))
        });
        candidates.dedup_by(|a, b| a.label.eq_ignore_ascii_case(&b.label) && a.kind == b.kind);

        let pref_up = prefix.to_ascii_uppercase();
        let items = candidates
            .into_iter()
            .filter(|s| s.label.to_ascii_uppercase().starts_with(&pref_up))
            .map(|s| CompletionItem {
                label: s.label.clone(),
                kind: Some(s.kind),
                detail: Some(s.detail),
                insert_text: Some(s.label),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Ok(Some(CompletionResponse::Array(items)))
    }
}
