use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};

use crate::analysis::formatting::{IndentOptions, autoindent_text, preserves_ast_shape};
use crate::backend::Backend;

impl Backend {
    pub async fn handle_formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let config = self.config.lock().await.clone();
        if !config.formatting.enabled {
            return Ok(None);
        }

        let Some(text) = self.get_document_text(&uri) else {
            return Ok(None);
        };

        let indent_size = if params.options.tab_size > 0 {
            params.options.tab_size as usize
        } else {
            config.formatting.indent_size
        };
        let options = IndentOptions {
            indent_size,
            use_tabs: !params.options.insert_spaces || config.formatting.use_tabs,
        };

        let formatted = autoindent_text(&text, options);
        if formatted == text {
            return Ok(Some(vec![]));
        }

        let mut parser = self.new_abl_parser();
        if !preserves_ast_shape(&text, &formatted, &mut parser) {
            return Ok(None);
        }

        if config.formatting.idempotence {
            let formatted_again = autoindent_text(&formatted, options);
            if formatted_again != formatted {
                return Ok(None);
            }
        }

        Ok(Some(vec![TextEdit {
            range: full_document_range(&text),
            new_text: formatted,
        }]))
    }
}

fn full_document_range(text: &str) -> Range {
    let mut line = 0u32;
    let mut col = 0u32;
    for b in text.bytes() {
        if b == b'\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    Range::new(Position::new(0, 0), Position::new(line, col))
}

#[cfg(test)]
mod tests {
    use super::full_document_range;
    use tower_lsp::lsp_types::{Position, Range};

    #[test]
    fn calculates_range_for_multiline_text() {
        let text = "a\nbc\n";
        let got = full_document_range(text);
        assert_eq!(got, Range::new(Position::new(0, 0), Position::new(2, 0)));
    }
}
