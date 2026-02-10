use std::collections::HashSet;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::definitions::collect_definition_symbols;
use crate::analysis::includes::collect_include_sites;
use crate::analysis::schema::normalize_lookup_key;
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

fn has_schema_key(map: &dashmap::DashMap<String, Vec<Location>>, key_upper: &str) -> bool {
    map.contains_key(key_upper)
        || map
            .iter()
            .any(|entry| entry.key().eq_ignore_ascii_case(key_upper))
}

struct FunctionSignature {
    name: String,
    params: Vec<String>,
    return_type: Option<String>,
    is_forward: bool,
}

fn find_function_signature(root: Node, src: &[u8], symbol: &str) -> Option<FunctionSignature> {
    let mut matches = Vec::new();
    collect_function_signatures(root, src, symbol, &mut matches);
    matches.into_iter().max_by_key(signature_score)
}

fn collect_function_signatures(
    node: Node,
    src: &[u8],
    symbol: &str,
    out: &mut Vec<FunctionSignature>,
) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) && let Some(name_node) = node.child_by_field_name("name")
        && let Ok(name) = name_node.utf8_text(src)
        && name.eq_ignore_ascii_case(symbol)
    {
        let params = collect_function_params(node, src);
        let return_type = node
            .child_by_field_name("type")
            .and_then(|n| n.utf8_text(src).ok())
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty());

        out.push(FunctionSignature {
            name: name.to_string(),
            params,
            return_type,
            is_forward: node.kind() == "function_forward_definition",
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_signatures(ch, src, symbol, out);
        }
    }
}

fn collect_function_params(function_node: Node, src: &[u8]) -> Vec<String> {
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let mut header_params = Vec::new();
        collect_params_by_kind(parameters_node, src, "parameter", &mut header_params);
        if !header_params.is_empty() {
            return header_params;
        }
    }

    let mut out = Vec::new();
    collect_params_recursive(function_node, src, &mut out, true);
    out
}

fn collect_params_recursive(node: Node, src: &[u8], out: &mut Vec<String>, is_root: bool) {
    if !is_root
        && matches!(
            node.kind(),
            "function_definition"
                | "function_forward_definition"
                | "procedure_definition"
                | "method_definition"
                | "constructor_definition"
                | "destructor_definition"
        )
    {
        return;
    }

    if matches!(node.kind(), "parameter" | "parameter_definition")
        && let Some(rendered) = render_param(node, src)
    {
        out.push(rendered);
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_params_recursive(ch, src, out, false);
        }
    }
}

fn collect_params_by_kind(node: Node, src: &[u8], target_kind: &str, out: &mut Vec<String>) {
    if node.kind() == target_kind
        && let Some(rendered) = render_param(node, src)
    {
        out.push(rendered);
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_params_by_kind(ch, src, target_kind, out);
        }
    }
}

fn render_param(node: Node, src: &[u8]) -> Option<String> {
    let name = node
        .child_by_field_name("name")
        .and_then(|n| n.utf8_text(src).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "param".to_string());

    let ty = node
        .child_by_field_name("type")
        .and_then(|n| n.utf8_text(src).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            node.child_by_field_name("table")
                .and_then(|n| n.utf8_text(src).ok())
                .map(|s| format!("TABLE {}", s.trim()))
        })
        .or_else(|| {
            node.child_by_field_name("dataset")
                .and_then(|n| n.utf8_text(src).ok())
                .map(|s| format!("DATASET {}", s.trim()))
        })
        .unwrap_or_else(|| "ANY".to_string());

    let mode = node
        .utf8_text(src)
        .ok()
        .map(|raw| raw.trim().to_ascii_uppercase())
        .and_then(|raw| {
            if raw.starts_with("INPUT-OUTPUT ") {
                Some("INPUT-OUTPUT")
            } else if raw.starts_with("INPUT ") {
                Some("INPUT")
            } else if raw.starts_with("OUTPUT ") {
                Some("OUTPUT")
            } else {
                None
            }
        });

    Some(match mode {
        Some(mode) => format!("{mode} {name}: {ty}"),
        None => format!("{name}: {ty}"),
    })
}

fn signature_score(sig: &FunctionSignature) -> (usize, usize, usize) {
    (
        sig.params.len(),
        usize::from(sig.return_type.is_some()),
        usize::from(!sig.is_forward),
    )
}

#[cfg(test)]
mod tests {
    use super::find_function_signature;

    #[test]
    fn picks_richest_function_signature_and_renders_params() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL FORWARD.

FUNCTION foo RETURNS LOGICAL (INPUT p1 AS CHARACTER, OUTPUT p2 AS INTEGER):
  RETURN TRUE.
END FUNCTION.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let sig = find_function_signature(tree.root_node(), src.as_bytes(), "foo")
            .expect("function signature");
        assert_eq!(sig.name, "foo");
        assert_eq!(sig.return_type.as_deref(), Some("LOGICAL"));
        assert_eq!(sig.params.len(), 2);
        assert!(sig.params[0].contains("INPUT"));
        assert!(sig.params[0].contains("p1"));
        assert!(sig.params[1].contains("OUTPUT"));
        assert!(sig.params[1].contains("p2"));
    }
}
