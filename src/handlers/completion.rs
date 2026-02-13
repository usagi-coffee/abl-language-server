use std::collections::{HashMap, HashSet};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::completion::{
    field_detail, field_documentation, lookup_case_insensitive_fields, qualifier_before_dot,
    text_has_dot_before_cursor,
};
use crate::analysis::definitions::collect_definition_symbols;
use crate::analysis::includes::collect_include_sites;
use crate::analysis::local_tables::collect_local_table_definitions;
use crate::analysis::scopes::containing_scope;
use crate::backend::Backend;
use crate::backend::DbFieldInfo;
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
            Some(t) => t.value().clone(),
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };
        let tree = if let Some(t) = self.trees.get(&uri) {
            t.value().clone()
        } else {
            let parser_mutex = self
                .abl_parsers
                .entry(uri.clone())
                .or_insert_with(|| std::sync::Mutex::new(self.new_abl_parser()));
            let mut parser = parser_mutex.lock().expect("ABL parser mutex poisoned");
            let Some(parsed) = parser.parse(text.clone(), None) else {
                return Ok(Some(CompletionResponse::Array(vec![])));
            };
            self.trees.insert(uri.clone(), parsed.clone());
            parsed
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

            let mut local_table_defs = Vec::new();
            collect_local_table_definitions(
                tree.root_node(),
                text.as_bytes(),
                &mut local_table_defs,
            );
            let local_fields_by_table = local_table_defs
                .iter()
                .map(|d| (d.name_upper.clone(), d.fields.clone()))
                .collect::<HashMap<_, _>>();
            let local_like_by_table = local_table_defs
                .iter()
                .filter_map(|d| {
                    d.like_table_upper
                        .as_ref()
                        .map(|like| (d.name_upper.clone(), like.clone()))
                })
                .collect::<HashMap<_, _>>();

            if !self.db_fields_by_table.contains_key(&qualifier_upper)
                && !local_fields_by_table.contains_key(&qualifier_upper)
                && !local_like_by_table.contains_key(&qualifier_upper)
            {
                let mut mappings = Vec::new();
                collect_buffer_mappings(tree.root_node(), text.as_bytes(), &mut mappings);
                table_upper = mappings
                    .into_iter()
                    .find(|m| m.alias.eq_ignore_ascii_case(&qualifier_upper))
                    .map(|m| m.table.to_ascii_uppercase());
            }

            if let Some(table_key) = table_upper {
                if let Some(fields) = local_fields_by_table.get(&table_key) {
                    let items = build_field_completion_items(fields, &table_key, &field_prefix);
                    return Ok(Some(CompletionResponse::Array(items)));
                }

                if let Some(like_key) = local_like_by_table.get(&table_key)
                    && let Some(fields) =
                        lookup_case_insensitive_fields(&self.db_fields_by_table, like_key)
                {
                    let items = build_field_completion_items(&fields, &table_key, &field_prefix);
                    return Ok(Some(CompletionResponse::Array(items)));
                }

                let fields = lookup_case_insensitive_fields(&self.db_fields_by_table, &table_key);
                if let Some(fields) = fields {
                    let items = build_field_completion_items(&fields, &table_key, &field_prefix);
                    return Ok(Some(CompletionResponse::Array(items)));
                }
            }
        }
        if trigger_is_dot {
            return Ok(Some(CompletionResponse::Array(vec![])));
        }

        let mut candidates = Vec::<CompletionCandidate>::new();

        let root = tree.root_node();
        let current_scope = containing_scope(root, offset);
        let mut symbols = Vec::new();
        collect_definition_symbols(root, text.as_bytes(), &mut symbols);
        candidates.extend(
            symbols
                .into_iter()
                .filter(|s| s.start_byte <= offset)
                .filter(|s| {
                    if !is_parameter_symbol_at_byte(root, s.start_byte) {
                        return true;
                    }
                    symbol_is_in_current_scope(root, s.start_byte, current_scope)
                })
                .map(|s| CompletionCandidate {
                    label: s.label,
                    kind: s.kind,
                    detail: s.detail,
                }),
        );
        candidates.extend(
            self.collect_symbols_from_includes_for_completion(&uri, &text, offset)
                .await,
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

    async fn collect_symbols_from_includes_for_completion(
        &self,
        uri: &Url,
        text: &str,
        offset: usize,
    ) -> Vec<CompletionCandidate> {
        if !text.as_bytes().contains(&b'{') {
            return Vec::new();
        }

        let Some(current_path) = uri.to_file_path().ok() else {
            return Vec::new();
        };

        let include_sites = collect_include_sites(text);
        let mut parsed_files = HashSet::new();
        let mut out = Vec::new();
        let mut include_parser = self.new_abl_parser();

        for include in include_sites {
            if include.start_offset > offset {
                continue;
            }

            let Some(include_path) = self
                .resolve_include_path_for(&current_path, &include.path)
                .await
            else {
                continue;
            };
            if !parsed_files.insert(include_path.clone()) {
                continue;
            }

            let Ok(include_text) = tokio::fs::read_to_string(&include_path).await else {
                continue;
            };
            let include_tree = include_parser.parse(&include_text, None);
            let Some(include_tree) = include_tree else {
                continue;
            };
            let include_root = include_tree.root_node();

            let mut symbols = Vec::new();
            collect_definition_symbols(include_root, include_text.as_bytes(), &mut symbols);
            out.extend(
                symbols
                    .into_iter()
                    .filter(|s| !is_parameter_symbol_at_byte(include_root, s.start_byte))
                    .map(|s| CompletionCandidate {
                        label: s.label,
                        kind: s.kind,
                        detail: s.detail,
                    }),
            );
        }

        out
    }
}

fn build_field_completion_items(
    fields: &[DbFieldInfo],
    table_key: &str,
    field_prefix: &str,
) -> Vec<CompletionItem> {
    let pref_up = field_prefix.to_ascii_uppercase();
    let mut items = fields
        .iter()
        .filter(|f| f.name.to_ascii_uppercase().starts_with(&pref_up))
        .map(|f| CompletionItem {
            label: f.name.clone(),
            kind: Some(CompletionItemKind::FIELD),
            detail: Some(field_detail(f, table_key)),
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
    items
}

fn is_parameter_symbol_at_byte(root: tree_sitter::Node<'_>, start_byte: usize) -> bool {
    let Some(mut node) = root.named_descendant_for_byte_range(start_byte, start_byte) else {
        return false;
    };
    loop {
        if matches!(node.kind(), "parameter" | "parameter_definition") {
            return true;
        }
        let Some(parent) = node.parent() else {
            return false;
        };
        node = parent;
    }
}

fn symbol_is_in_current_scope(
    root: tree_sitter::Node<'_>,
    symbol_start_byte: usize,
    current_scope: Option<crate::analysis::scopes::ByteScope>,
) -> bool {
    let Some(current_scope) = current_scope else {
        return false;
    };
    let Some(symbol_scope) = containing_scope(root, symbol_start_byte) else {
        return false;
    };
    symbol_scope.start == current_scope.start && symbol_scope.end == current_scope.end
}
