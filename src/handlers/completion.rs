use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::{Duration, Instant};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::completion::{
    lookup_case_insensitive_fields, qualifier_before_dot, text_has_dot_before_cursor,
};
use crate::analysis::completion_support::{
    build_field_completion_items, completion_response, is_parameter_symbol_at_byte,
    symbol_is_in_current_scope,
};
use crate::analysis::definitions::collect_definition_symbols;
use crate::analysis::includes::collect_include_sites;
use crate::analysis::local_tables::collect_local_table_definitions;
use crate::analysis::scopes::containing_scope;
use crate::backend::Backend;
use crate::backend::CachedCompletionSymbol;
use crate::utils::position::{ascii_ident_prefix, lsp_pos_to_utf8_byte_offset};

struct CompletionCandidate {
    label: String,
    kind: CompletionItemKind,
    detail: String,
}

const COMPLETION_INCLUDE_BUDGET_MS: u64 = 120;

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

        let text = match self.get_document_text(&uri) {
            Some(t) => t,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };
        // Completion should remain responsive while typing; prefer cached tree
        // rather than blocking on reparse for every document version.
        let (tree, tree_is_stale) = match self.get_document_tree_prefer_cached_with_freshness(&uri)
        {
            Some(t) => t,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };
        let mut is_incomplete = tree_is_stale;
        let include_deadline = Instant::now() + Duration::from_millis(COMPLETION_INCLUDE_BUDGET_MS);

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
                    return Ok(Some(completion_response(items, is_incomplete)));
                }

                if let Some(like_key) = local_like_by_table.get(&table_key)
                    && let Some(fields) =
                        lookup_case_insensitive_fields(&self.db_fields_by_table, like_key)
                {
                    let items = build_field_completion_items(&fields, &table_key, &field_prefix);
                    return Ok(Some(completion_response(items, is_incomplete)));
                }

                let fields = lookup_case_insensitive_fields(&self.db_fields_by_table, &table_key);
                if let Some(fields) = fields {
                    let items = build_field_completion_items(&fields, &table_key, &field_prefix);
                    return Ok(Some(completion_response(items, is_incomplete)));
                }
            }
        }
        if trigger_is_dot {
            return Ok(Some(completion_response(vec![], is_incomplete)));
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
        let (include_candidates, include_timed_out) = self
            .collect_symbols_from_includes_for_completion(&uri, &text, offset, include_deadline)
            .await;
        is_incomplete |= include_timed_out;
        candidates.extend(include_candidates);

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

        Ok(Some(completion_response(items, is_incomplete)))
    }

    async fn collect_symbols_from_includes_for_completion(
        &self,
        uri: &Url,
        text: &str,
        offset: usize,
        deadline: Instant,
    ) -> (Vec<CompletionCandidate>, bool) {
        if !text.as_bytes().contains(&b'{') {
            return (Vec::new(), false);
        }

        let Some(current_path) = uri.to_file_path().ok() else {
            return (Vec::new(), false);
        };

        let include_sites = collect_include_sites(text);
        let mut parsed_files = HashSet::new();
        let mut out = Vec::new();
        let mut timed_out = false;

        for include in include_sites {
            if Instant::now() >= deadline {
                timed_out = true;
                break;
            }
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

            out.extend(
                self.get_cached_include_completion_candidates(&include_path)
                    .await,
            );
        }

        (out, timed_out)
    }

    async fn get_cached_include_completion_candidates(
        &self,
        include_path: &Path,
    ) -> Vec<CompletionCandidate> {
        if let Some(entry) = self.include_completion_cache.get(include_path) {
            return entry
                .symbols
                .iter()
                .map(|s| CompletionCandidate {
                    label: s.label.clone(),
                    kind: s.kind,
                    detail: s.detail.clone(),
                })
                .collect();
        }

        let Some((include_text_cached, include_tree)) =
            self.get_cached_include_parse(include_path).await
        else {
            return Vec::new();
        };
        let include_root = include_tree.root_node();
        let mut symbols = Vec::new();
        collect_definition_symbols(include_root, include_text_cached.as_bytes(), &mut symbols);
        let filtered = symbols
            .into_iter()
            .filter(|s| !is_parameter_symbol_at_byte(include_root, s.start_byte))
            .map(|s| CachedCompletionSymbol {
                label: s.label,
                kind: s.kind,
                detail: s.detail,
            })
            .collect::<Vec<_>>();
        self.include_completion_cache.insert(
            include_path.to_path_buf(),
            crate::backend::IncludeCompletionCacheEntry {
                symbols: filtered.clone(),
            },
        );
        filtered
            .into_iter()
            .map(|s| CompletionCandidate {
                label: s.label,
                kind: s.kind,
                detail: s.detail,
            })
            .collect()
    }
}
