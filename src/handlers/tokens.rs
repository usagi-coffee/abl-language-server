use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

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
        let Some(text) = self.docs.get(uri) else {
            return vec![];
        };
        let Some(tree) = self.trees.get(uri) else {
            return vec![];
        };

        let mut nodes = Vec::<Node>::new();
        collect_nodes_by_kind(tree.root_node(), "identifier", &mut nodes);

        if self.db_tables.is_empty() {
            return vec![];
        }

        let mut raw = Vec::<(u32, u32, u32)>::new();
        for node in nodes {
            let sp = node.start_position();
            let ep = node.end_position();
            let start_line = sp.row as u32;
            let start_col = sp.column as u32;
            let len = (ep.column.saturating_sub(sp.column)) as u32;

            if len == 0 {
                continue;
            }
            if !is_in_range(start_line, start_col, len, range.as_ref()) {
                continue;
            }

            let Ok(name) = node.utf8_text(text.as_bytes()) else {
                continue;
            };
            if self.db_tables.contains(&name.to_ascii_uppercase()) {
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

fn is_in_range(start_line: u32, start_col: u32, length: u32, range: Option<&Range>) -> bool {
    let Some(range) = range else {
        return true;
    };
    let token_end_col = start_col.saturating_add(length);

    if start_line < range.start.line || start_line > range.end.line {
        return false;
    }
    if start_line == range.start.line && token_end_col <= range.start.character {
        return false;
    }
    if start_line == range.end.line && start_col >= range.end.character {
        return false;
    }

    true
}
