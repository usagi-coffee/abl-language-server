use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::buffers::collect_buffer_mappings;
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
        let Some(text_entry) = self.docs.get(uri) else {
            return vec![];
        };
        let text = text_entry.value().clone();
        drop(text_entry);

        let tree = if let Some(tree) = self.trees.get(uri) {
            tree.value().clone()
        } else {
            let parser_mutex = self
                .abl_parsers
                .entry(uri.clone())
                .or_insert_with(|| std::sync::Mutex::new(self.new_abl_parser()));
            let mut parser = parser_mutex.lock().expect("ABL parser mutex poisoned");
            let Some(parsed) = parser.parse(text.clone(), None) else {
                return vec![];
            };
            self.trees.insert(uri.clone(), parsed.clone());
            parsed
        };

        let mut nodes = Vec::<Node>::new();
        collect_nodes_by_kind(tree.root_node(), "identifier", &mut nodes);

        let mut buffer_mappings = Vec::new();
        collect_buffer_mappings(tree.root_node(), text.as_bytes(), &mut buffer_mappings);
        let buffer_aliases = buffer_mappings
            .into_iter()
            .map(|m| m.alias.to_ascii_uppercase())
            .collect::<std::collections::HashSet<_>>();
        if self.db_tables.is_empty() && buffer_aliases.is_empty() {
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
            if self.db_tables.contains(&name_upper) || buffer_aliases.contains(&name_upper) {
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

fn line_start_offsets(text: &str) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

fn point_column_byte_to_utf16(
    text: &str,
    line_starts: &[usize],
    line: u32,
    column_byte: usize,
) -> Option<u32> {
    let line_start = *line_starts.get(line as usize)?;
    let abs = line_start.saturating_add(column_byte);
    if abs > text.len() || !text.is_char_boundary(abs) {
        return None;
    }
    Some(text[line_start..abs].encode_utf16().count() as u32)
}

#[cfg(test)]
mod tests {
    use super::{line_start_offsets, point_column_byte_to_utf16};

    #[test]
    fn converts_byte_column_to_utf16_with_non_ascii_prefix() {
        let text = "oNestedObject:Add(\"Ilość\", lpopak_mstr.lpopak_ilosc).";
        let starts = line_start_offsets(text);

        let token = "lpopak_mstr";
        let byte_col = text.find(token).expect("token byte column");
        let utf16_col = point_column_byte_to_utf16(text, &starts, 0, byte_col)
            .expect("utf16 column conversion");

        let expected_utf16 = text[..byte_col].encode_utf16().count() as u32;
        assert_eq!(utf16_col, expected_utf16);
    }
}
