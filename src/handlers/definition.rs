use crate::analysis::buffers::collect_buffer_mappings;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::definitions::{
    AblDefinitionSite, collect_definition_sites, collect_function_definition_sites,
};
use crate::analysis::includes::collect_include_sites;
use crate::backend::Backend;
use crate::utils::position::{
    ascii_ident_at_or_before, ascii_ident_or_dash_at_or_before, lsp_pos_to_utf8_byte_offset,
};

impl Backend {
    async fn resolve_include_location(
        &self,
        uri: &Url,
        text: &str,
        offset: usize,
    ) -> Option<Location> {
        let include_sites = collect_include_sites(text);
        let include = include_sites
            .into_iter()
            .find(|site| offset >= site.start_offset && offset <= site.end_offset)?;

        let current_path = uri.to_file_path().ok()?;
        let workspace_root = self.workspace_root.lock().await.clone();
        let include_path =
            resolve_include_path(&current_path, workspace_root.as_deref(), &include.path)?;
        let include_uri = Url::from_file_path(include_path).ok()?;

        Some(Location {
            uri: include_uri,
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
        })
    }

    pub async fn handle_goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
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
        if let Some(location) = self.resolve_include_location(&uri, &text, offset).await {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        let symbol = match ascii_ident_or_dash_at_or_before(&text, offset)
            .or_else(|| ascii_ident_at_or_before(&text, offset))
        {
            Some(s) => s,
            None => return Ok(None),
        };
        let symbol_upper = normalize_lookup_key(&symbol);

        // Buffer alias fallback: DEFINE BUFFER alias FOR table.
        let mut buffer_mappings = Vec::new();
        collect_buffer_mappings(tree.root_node(), text.as_bytes(), &mut buffer_mappings);
        let mut buffer_before: Option<(usize, String)> = None;
        let mut buffer_after: Option<(usize, String)> = None;
        for mapping in buffer_mappings {
            if !mapping.alias.eq_ignore_ascii_case(&symbol_upper) {
                continue;
            }
            let table_key = normalize_lookup_key(&mapping.table);
            if mapping.start_byte <= offset {
                let should_take = buffer_before
                    .as_ref()
                    .map(|(start, _)| mapping.start_byte > *start)
                    .unwrap_or(true);
                if should_take {
                    buffer_before = Some((mapping.start_byte, table_key));
                }
            } else {
                let should_take = buffer_after
                    .as_ref()
                    .map(|(start, _)| mapping.start_byte < *start)
                    .unwrap_or(true);
                if should_take {
                    buffer_after = Some((mapping.start_byte, table_key));
                }
            }
        }
        if let Some((_, table_key)) = buffer_before.or(buffer_after) {
            let table_defs = self.db_table_definitions.lock().await;
            if let Some(locations) = table_defs.get(&table_key)
                && let Some(location) = pick_single_location(locations)
            {
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }

        let mut sites = Vec::new();
        collect_definition_sites(tree.root_node(), text.as_bytes(), &mut sites);

        let mut best_before: Option<(usize, Range)> = None;
        let mut best_after: Option<(usize, Range)> = None;

        for site in sites {
            if !site.label.eq_ignore_ascii_case(&symbol) {
                continue;
            }

            if site.start_byte <= offset {
                let should_take = best_before
                    .as_ref()
                    .map(|(start, _)| site.start_byte > *start)
                    .unwrap_or(true);
                if should_take {
                    best_before = Some((site.start_byte, site.range));
                }
            } else {
                let should_take = best_after
                    .as_ref()
                    .map(|(start, _)| site.start_byte < *start)
                    .unwrap_or(true);
                if should_take {
                    best_after = Some((site.start_byte, site.range));
                }
            }
        }

        let target_range = best_before.or(best_after).map(|(_, range)| range);
        if let Some(range) = target_range {
            let location = Location { uri, range };
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        let Some(scope) = containing_scope(tree.root_node(), offset) else {
            return Ok(None);
        };

        let Some(current_path) = uri.to_file_path().ok() else {
            return Ok(None);
        };

        let workspace_root = self.workspace_root.lock().await.clone();
        let include_sites = collect_include_sites(&text);

        let mut parsed_include_functions: HashMap<PathBuf, Vec<AblDefinitionSite>> = HashMap::new();
        let mut include_before: Option<(usize, Location)> = None;
        let mut include_after: Option<(usize, Location)> = None;

        for include in include_sites {
            if include.start_offset < scope.start || include.start_offset > scope.end {
                continue;
            }

            let Some(include_path) =
                resolve_include_path(&current_path, workspace_root.as_deref(), &include.path)
            else {
                continue;
            };

            if !parsed_include_functions.contains_key(&include_path) {
                let Ok(include_text) = tokio::fs::read_to_string(&include_path).await else {
                    continue;
                };

                let include_tree = {
                    let mut parser = self.parser.lock().await;
                    parser.parse(&include_text, None)
                };
                let Some(include_tree) = include_tree else {
                    continue;
                };

                let mut function_sites = Vec::new();
                collect_function_definition_sites(
                    include_tree.root_node(),
                    include_text.as_bytes(),
                    &mut function_sites,
                );
                parsed_include_functions.insert(include_path.clone(), function_sites);
            }

            let Some(function_sites) = parsed_include_functions.get(&include_path) else {
                continue;
            };

            let Some(include_uri) = Url::from_file_path(&include_path).ok() else {
                continue;
            };

            for site in function_sites {
                if !site.label.eq_ignore_ascii_case(&symbol) {
                    continue;
                }

                let location = Location {
                    uri: include_uri.clone(),
                    range: site.range.clone(),
                };

                if include.start_offset <= offset {
                    let should_take = include_before
                        .as_ref()
                        .map(|(site_offset, _)| include.start_offset > *site_offset)
                        .unwrap_or(true);
                    if should_take {
                        include_before = Some((include.start_offset, location));
                    }
                } else {
                    let should_take = include_after
                        .as_ref()
                        .map(|(site_offset, _)| include.start_offset < *site_offset)
                        .unwrap_or(true);
                    if should_take {
                        include_after = Some((include.start_offset, location));
                    }
                }
            }
        }

        let target = include_before
            .or(include_after)
            .map(|(_, location)| location);
        if let Some(location) = target {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        // Fallback: DB schema definitions parsed from configured .df dumpfile(s).
        let table_defs = self.db_table_definitions.lock().await;
        if let Some(locations) = table_defs.get(&symbol_upper)
            && let Some(location) = pick_single_location(locations)
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }
        drop(table_defs);

        let field_defs = self.db_field_definitions.lock().await;
        if let Some(locations) = field_defs.get(&symbol_upper)
            && let Some(location) = pick_single_location(locations)
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }
        drop(field_defs);

        let index_defs = self.db_index_definitions.lock().await;
        if let Some(locations) = index_defs.get(&symbol_upper)
            && let Some(location) = pick_single_location(locations)
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        Ok(None)
    }
}

#[derive(Clone, Copy)]
struct ByteScope {
    start: usize,
    end: usize,
}

fn containing_scope(root: Node, offset: usize) -> Option<ByteScope> {
    let mut node = root.named_descendant_for_byte_range(offset, offset)?;
    loop {
        if is_scope_node(node.kind()) {
            return Some(ByteScope {
                start: node.start_byte(),
                end: node.end_byte(),
            });
        }
        let Some(parent) = node.parent() else {
            break;
        };
        node = parent;
    }

    Some(ByteScope {
        start: root.start_byte(),
        end: root.end_byte(),
    })
}

fn is_scope_node(kind: &str) -> bool {
    matches!(
        kind,
        "function_definition"
            | "function_forward_definition"
            | "procedure_definition"
            | "procedure_forward_definition"
            | "method_definition"
            | "constructor_definition"
            | "destructor_definition"
    )
}

fn resolve_include_path(
    current_file: &Path,
    workspace_root: Option<&Path>,
    include: &str,
) -> Option<PathBuf> {
    let candidate = PathBuf::from(include);
    if candidate.is_absolute() && candidate.exists() {
        return Some(candidate);
    }

    if let Some(current_dir) = current_file.parent() {
        let from_current = current_dir.join(include);
        if from_current.exists() {
            return Some(from_current);
        }
    }

    if let Some(root) = workspace_root {
        let from_root = root.join(include);
        if from_root.exists() {
            return Some(from_root);
        }
    }

    None
}

fn pick_single_location(locations: &[Location]) -> Option<Location> {
    locations.iter().cloned().min_by(|a, b| {
        a.uri
            .as_str()
            .cmp(b.uri.as_str())
            .then(a.range.start.line.cmp(&b.range.start.line))
            .then(a.range.start.character.cmp(&b.range.start.character))
    })
}

fn normalize_lookup_key(symbol: &str) -> String {
    symbol
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .to_ascii_uppercase()
}
