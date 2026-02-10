use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::definitions::collect_definition_symbols;
use crate::backend::{Backend, DbFieldInfo};
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

fn qualifier_before_dot(text: &str, offset: usize, prefix: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let prefix_len = prefix.len();
    if offset < prefix_len + 1 {
        return None;
    }
    let dot_pos = offset - prefix_len - 1;
    if bytes.get(dot_pos).copied() != Some(b'.') {
        return None;
    }

    let mut start = dot_pos;
    while start > 0 {
        let c = bytes[start - 1];
        let is_ident = c.is_ascii_alphanumeric() || c == b'_' || c == b'-';
        if !is_ident {
            break;
        }
        start -= 1;
    }

    if start == dot_pos {
        return None;
    }
    Some(text[start..dot_pos].to_string())
}

fn lookup_case_insensitive_fields(
    map: &dashmap::DashMap<String, Vec<DbFieldInfo>>,
    key: &str,
) -> Option<Vec<DbFieldInfo>> {
    map.get(key)
        .map(|fields| fields.value().clone())
        .or_else(|| {
            map.iter().find_map(|entry| {
                if entry.key().eq_ignore_ascii_case(key) {
                    Some(entry.value().clone())
                } else {
                    None
                }
            })
        })
}

fn text_has_dot_before_cursor(text: &str, offset: usize) -> bool {
    if offset == 0 {
        return false;
    }
    text.as_bytes().get(offset - 1).copied() == Some(b'.')
}

fn field_detail(field: &DbFieldInfo, table_key: &str) -> String {
    match field.field_type.as_deref() {
        Some(ty) => format!("{ty} ({table_key})"),
        None => format!("FIELD ({table_key})"),
    }
}

fn field_documentation(field: &DbFieldInfo) -> Option<Documentation> {
    let mut lines = Vec::new();
    if let Some(label) = &field.label
        && !label.is_empty()
    {
        lines.push(format!("Label: {label}"));
    }
    if let Some(format) = &field.format
        && !format.is_empty()
    {
        lines.push(format!("Format: {format}"));
    }
    if let Some(description) = &field.description
        && !description.is_empty()
    {
        lines.push(format!("Description: {description}"));
    }

    if lines.is_empty() {
        None
    } else {
        Some(Documentation::String(lines.join("\n")))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        field_detail, field_documentation, qualifier_before_dot, text_has_dot_before_cursor,
    };
    use crate::backend::DbFieldInfo;
    use tower_lsp::lsp_types::Documentation;

    #[test]
    fn finds_qualifier_before_dot_with_dash() {
        let text = "f-lpd_det.";
        let offset = text.len();
        let prefix = "";
        assert_eq!(
            qualifier_before_dot(text, offset, prefix).as_deref(),
            Some("f-lpd_det")
        );
        assert!(text_has_dot_before_cursor(text, offset));
    }

    #[test]
    fn renders_field_detail_and_docs() {
        let field = DbFieldInfo {
            name: "z9zw_id".to_string(),
            field_type: Some("CHARACTER".to_string()),
            format: Some("x(24)".to_string()),
            label: Some("ID".to_string()),
            description: Some("Identifier".to_string()),
        };

        assert_eq!(field_detail(&field, "z9zw_mstr"), "CHARACTER (z9zw_mstr)");
        let docs = field_documentation(&field).expect("documentation");
        match docs {
            Documentation::String(s) => {
                assert!(s.contains("Label: ID"));
                assert!(s.contains("Format: x(24)"));
                assert!(s.contains("Description: Identifier"));
            }
            _ => panic!("unexpected documentation kind"),
        }
    }
}
