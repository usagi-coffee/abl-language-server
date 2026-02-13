use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::local_tables::collect_local_table_definitions;
use crate::analysis::semantic_tokens::{
    is_in_range, line_start_offsets, point_column_byte_to_utf16,
};
use crate::backend::Backend;
use crate::utils::ts::collect_nodes_by_kind;

const TABLE_TOKEN_TYPE_INDEX: u32 = 0;

impl Backend {
    pub async fn handle_semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        if !self.config.lock().await.semantic_tokens.enabled {
            return Ok(None);
        }
        let uri = params.text_document.uri;
        let tokens = self.collect_table_semantic_tokens(&uri, None).await;
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        })))
    }

    pub async fn handle_semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        if !self.config.lock().await.semantic_tokens.enabled {
            return Ok(None);
        }
        let uri = params.text_document.uri;
        let tokens = self
            .collect_table_semantic_tokens(&uri, Some(params.range))
            .await;
        Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        })))
    }

    async fn collect_table_semantic_tokens(
        &self,
        uri: &Url,
        range: Option<Range>,
    ) -> Vec<SemanticToken> {
        let Some(text) = self.get_document_text(uri) else {
            return vec![];
        };
        let tree = match self.get_document_tree_or_parse(uri) {
            Some(tree) => tree,
            None => {
                return vec![];
            }
        };

        let mut nodes = Vec::<Node>::new();
        collect_nodes_by_kind(tree.root_node(), "identifier", &mut nodes);

        let mut buffer_mappings = Vec::new();
        collect_buffer_mappings(tree.root_node(), text.as_bytes(), &mut buffer_mappings);
        let buffer_aliases = buffer_mappings
            .into_iter()
            .map(|m| m.alias.to_ascii_uppercase())
            .collect::<std::collections::HashSet<_>>();
        let mut local_table_defs = Vec::new();
        collect_local_table_definitions(tree.root_node(), text.as_bytes(), &mut local_table_defs);
        let local_table_names = local_table_defs
            .into_iter()
            .map(|d| d.name_upper)
            .collect::<std::collections::HashSet<_>>();
        if self.db_tables.is_empty() && buffer_aliases.is_empty() && local_table_names.is_empty() {
            return vec![];
        }

        let line_starts = line_start_offsets(text.as_str());
        let mut raw = Vec::<(u32, u32, u32)>::new();
        for node in nodes {
            let sp = node.start_position();
            let start_line = sp.row as u32;
            let Ok(name) = node.utf8_text(text.as_bytes()) else {
                continue;
            };
            let name_upper = name.to_ascii_uppercase();
            if self.db_tables.contains(&name_upper)
                || buffer_aliases.contains(&name_upper)
                || local_table_names.contains(&name_upper)
            {
                let Some(start_col) =
                    point_column_byte_to_utf16(text.as_str(), &line_starts, start_line, sp.column)
                else {
                    continue;
                };
                let len = name.encode_utf16().count() as u32;
                if len == 0 {
                    continue;
                }
                if !is_in_range(start_line, start_col, len, range.as_ref()) {
                    continue;
                }
                raw.push((start_line, start_col, len));
            }
        }
        raw.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        raw.dedup();

        let mut out = Vec::<SemanticToken>::new();
        let mut prev_line = 0u32;
        let mut prev_start = 0u32;
        for (line, start, length) in raw {
            let delta_line = line.saturating_sub(prev_line);
            let delta_start = if delta_line == 0 {
                start.saturating_sub(prev_start)
            } else {
                start
            };
            out.push(SemanticToken {
                delta_line,
                delta_start,
                length,
                token_type: TABLE_TOKEN_TYPE_INDEX,
                token_modifiers_bitset: 0,
            });
            prev_line = line;
            prev_start = start;
        }

        out
    }
}
