use std::collections::HashSet;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::definitions::collect_definition_symbols;
use crate::analysis::functions::{FunctionSignature, find_function_signature};
use crate::analysis::includes::collect_include_sites;
use crate::analysis::schema::normalize_lookup_key;
use crate::analysis::schema_lookup::has_schema_key;
use crate::analysis::scopes::containing_scope;
use crate::backend::Backend;
use crate::utils::position::{
    ascii_ident_at_or_before, ascii_ident_or_dash_at_or_before, lsp_pos_to_utf8_byte_offset,
};
use crate::utils::ts::{direct_child_by_kind, node_trimmed_text};

impl Backend {
    pub async fn handle_hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let text = match self.get_document_text(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        let tree = match self.get_document_tree_or_parse(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let offset = match lsp_pos_to_utf8_byte_offset(&text, pos) {
            Some(o) => o,
            None => return Ok(None),
        };
        let symbol = match symbol_at_offset(tree.root_node(), &text, offset).or_else(|| {
            ascii_ident_or_dash_at_or_before(&text, offset)
                .or_else(|| ascii_ident_at_or_before(&text, offset))
        }) {
            Some(s) => s,
            None => return Ok(None),
        };
        let symbol_upper = normalize_lookup_key(&symbol, true);

        if let Some(sig) = find_function_signature(tree.root_node(), text.as_bytes(), &symbol) {
            let header = match sig.return_type {
                Some(ret) => format!(
                    "`FUNCTION {}({}) RETURNS {}`",
                    sig.name,
                    sig.params.join(", "),
                    ret
                ),
                None => format!("`FUNCTION {}({})`", sig.name, sig.params.join(", ")),
            };
            return Ok(Some(markdown_hover(header)));
        }
        if let Some(sig) = self
            .find_function_signature_from_includes(&uri, &text, tree.root_node(), offset, &symbol)
            .await
        {
            let header = match sig.return_type {
                Some(ret) => format!(
                    "`FUNCTION {}({}) RETURNS {}`",
                    sig.name,
                    sig.params.join(", "),
                    ret
                ),
                None => format!("`FUNCTION {}({})`", sig.name, sig.params.join(", ")),
            };
            return Ok(Some(markdown_hover(header)));
        }

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

        if has_schema_key(&self.db_table_definitions, &symbol_upper) {
            return Ok(Some(markdown_hover(format!("**DB Table** `{}`", symbol))));
        }

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

        if has_schema_key(&self.db_index_definitions, &symbol_upper) {
            return Ok(Some(markdown_hover(format!("**DB Index** `{}`", symbol))));
        }

        Ok(None)
    }

    async fn find_db_field_matches(&self, field_upper: &str) -> Vec<DbFieldMatch> {
        let mut out = Vec::new();
        for entry in self.db_fields_by_table.iter() {
            let table = entry.key();
            let fields = entry.value();
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

    async fn find_function_signature_from_includes(
        &self,
        uri: &Url,
        text: &str,
        root: Node<'_>,
        offset: usize,
        symbol: &str,
    ) -> Option<FunctionSignature> {
        let scope = containing_scope(root, offset)?;
        let current_path = uri.to_file_path().ok()?;

        let include_sites = collect_include_sites(text);
        let mut seen_files = HashSet::new();

        for include in include_sites {
            if include.start_offset < scope.start || include.start_offset > scope.end {
                continue;
            }

            let Some(include_path) = self
                .resolve_include_path_for(&current_path, &include.path)
                .await
            else {
                continue;
            };
            if !seen_files.insert(include_path.clone()) {
                continue;
            }

            let Some((include_text, include_tree)) =
                self.get_cached_include_parse(&include_path).await
            else {
                continue;
            };

            if let Some(sig) =
                find_function_signature(include_tree.root_node(), include_text.as_bytes(), symbol)
            {
                return Some(sig);
            }
        }

        None
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

fn symbol_at_offset(root: Node<'_>, text: &str, offset: usize) -> Option<String> {
    let node = root.named_descendant_for_byte_range(offset, offset)?;
    if node.kind() == "identifier" {
        return node_trimmed_text(node, text.as_bytes());
    }

    direct_child_by_kind(node, "identifier").and_then(|n| node_trimmed_text(n, text.as_bytes()))
}
