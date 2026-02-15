use std::collections::HashSet;
use tower_lsp::lsp_types::Range;
use tree_sitter::Node;

use crate::utils::ts::node_to_range;

/// Collects table names from parsed DF source (`ADD TABLE "name"` statements).
pub fn collect_df_table_names(node: Node, src: &[u8], out: &mut HashSet<String>) {
    if node.kind() == "add_table_statement"
        && let Some(table_node) = node.child_by_field_name("table")
        && let Ok(raw) = table_node.utf8_text(src)
        && let Some(name) = unquote(raw)
    {
        out.insert(name.to_ascii_uppercase());
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_table_names(ch, src, out);
        }
    }
}

pub struct DfTableSite {
    pub name: String,
    pub range: Range,
}

/// Collects table definition sites from parsed DF source.
pub fn collect_df_table_sites(node: Node, src: &[u8], out: &mut Vec<DfTableSite>) {
    if node.kind() == "add_table_statement"
        && let Some(table_node) = node.child_by_field_name("table")
        && let Ok(raw) = table_node.utf8_text(src)
        && let Some(name) = unquote(raw)
    {
        out.push(DfTableSite {
            name: name.to_string(),
            range: node_to_range(table_node),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_table_sites(ch, src, out);
        }
    }
}

pub struct DfFieldSite {
    pub name: String,
    pub range: Range,
}

/// Collects field definition sites from parsed DF source.
pub fn collect_df_field_sites(node: Node, src: &[u8], out: &mut Vec<DfFieldSite>) {
    if node.kind() == "add_field_statement"
        && let Some(field_node) = node.child_by_field_name("field")
        && let Ok(raw) = field_node.utf8_text(src)
        && let Some(name) = unquote(raw)
    {
        out.push(DfFieldSite {
            name: name.to_string(),
            range: node_to_range(field_node),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_field_sites(ch, src, out);
        }
    }
}

pub struct DfTableField {
    pub table: String,
    pub field: String,
    pub field_type: Option<String>,
    pub format: Option<String>,
    pub label: Option<String>,
    pub description: Option<String>,
}

/// Collects `(table, field)` pairs from `ADD FIELD "field" OF "table" ...`.
pub fn collect_df_table_fields(node: Node, src: &[u8], out: &mut Vec<DfTableField>) {
    if node.kind() == "add_field_statement"
        && let (Some(field_node), Some(table_node)) = (
            node.child_by_field_name("field"),
            node.child_by_field_name("table"),
        )
        && let (Ok(field_raw), Ok(table_raw)) =
            (field_node.utf8_text(src), table_node.utf8_text(src))
        && let (Some(field), Some(table)) = (unquote(field_raw), unquote(table_raw))
    {
        let field_type = node
            .child_by_field_name("type")
            .and_then(|t| t.utf8_text(src).ok())
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty());
        let mut format = None;
        let mut label = None;
        let mut description = None;

        for i in 0..node.child_count() {
            let Some(ch) = node.child(i as u32) else {
                continue;
            };
            if ch.kind() != "field_tuning" {
                continue;
            }
            let Ok(raw) = ch.utf8_text(src) else {
                continue;
            };
            let upper = raw.trim().to_ascii_uppercase();
            if upper.starts_with("FORMAT ") {
                format = extract_first_quoted(raw);
            } else if upper.starts_with("LABEL ") {
                label = extract_first_quoted(raw);
            } else if upper.starts_with("DESCRIPTION ") {
                description = extract_first_quoted(raw);
            }
        }

        out.push(DfTableField {
            table: table.to_string(),
            field: field.to_string(),
            field_type,
            format,
            label,
            description,
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_table_fields(ch, src, out);
        }
    }
}

pub struct DfIndexSite {
    pub name: String,
    pub range: Range,
}

/// Collects index definition sites from parsed DF source.
pub fn collect_df_index_sites(node: Node, src: &[u8], out: &mut Vec<DfIndexSite>) {
    if node.kind() == "add_index_statement"
        && let Some(index_node) = node.child_by_field_name("index")
        && let Ok(raw) = index_node.utf8_text(src)
        && let Some(name) = unquote(raw)
    {
        out.push(DfIndexSite {
            name: name.to_string(),
            range: node_to_range(index_node),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_index_sites(ch, src, out);
        }
    }
}

pub struct DfTableIndex {
    pub table: String,
    pub index: String,
    pub fields: Vec<String>,
}

/// Collects `(table, index)` pairs from `ADD INDEX "index" ON "table"`.
pub fn collect_df_table_indexes(node: Node, src: &[u8], out: &mut Vec<DfTableIndex>) {
    if node.kind() == "add_index_statement"
        && let (Some(index_node), Some(table_node)) = (
            node.child_by_field_name("index"),
            node.child_by_field_name("table"),
        )
        && let (Ok(index_raw), Ok(table_raw)) =
            (index_node.utf8_text(src), table_node.utf8_text(src))
        && let (Some(index), Some(table)) = (unquote(index_raw), unquote(table_raw))
    {
        let fields = node
            .utf8_text(src)
            .ok()
            .map(extract_index_field_names)
            .unwrap_or_default();

        out.push(DfTableIndex {
            table: table.to_string(),
            index: index.to_string(),
            fields,
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_table_indexes(ch, src, out);
        }
    }
}

fn unquote(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0];
        let last = trimmed.as_bytes()[trimmed.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return Some(&trimmed[1..trimmed.len() - 1]);
        }
    }
    None
}

fn extract_first_quoted(raw: &str) -> Option<String> {
    let bytes = raw.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let quote = bytes[i];
        if quote != b'"' && quote != b'\'' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut j = start;
        while j < bytes.len() {
            if bytes[j] == quote {
                if let Some(s) = raw.get(start..j) {
                    return Some(s.to_string());
                }
                return None;
            }
            j += 1;
        }
        break;
    }
    None
}

fn extract_index_field_names(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in raw.lines() {
        if !line
            .trim_start()
            .to_ascii_uppercase()
            .starts_with("INDEX-FIELD")
        {
            continue;
        }
        if let Some(field_name) = extract_first_quoted(line) {
            out.push(field_name);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        collect_df_field_sites, collect_df_index_sites, collect_df_table_indexes,
        collect_df_table_names, collect_df_table_sites, extract_first_quoted,
        extract_index_field_names, unquote,
    };
    use std::collections::HashSet;

    #[test]
    fn collects_table_field_and_index_sites() {
        let src = r#"
ADD TABLE "z9zw_mstr"
  AREA "Schema Area"
.
ADD FIELD "z9zw_id" OF "z9zw_mstr" AS character
  FORMAT "x(24)"
.
ADD INDEX "z9zw_idx" ON "z9zw_mstr"
  UNIQUE
  INDEX-FIELD "z9zw_id" ASC
.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_df::LANGUAGE.into())
            .expect("set df language");
        let tree = parser.parse(src, None).expect("parse df");

        let mut tables = HashSet::new();
        collect_df_table_names(tree.root_node(), src.as_bytes(), &mut tables);
        assert!(tables.contains("Z9ZW_MSTR"));

        let mut table_sites = Vec::new();
        collect_df_table_sites(tree.root_node(), src.as_bytes(), &mut table_sites);
        assert!(
            table_sites
                .iter()
                .any(|s| s.name.eq_ignore_ascii_case("Z9ZW_MSTR"))
        );

        let mut field_sites = Vec::new();
        collect_df_field_sites(tree.root_node(), src.as_bytes(), &mut field_sites);
        assert!(
            field_sites
                .iter()
                .any(|s| s.name.eq_ignore_ascii_case("Z9ZW_ID"))
        );

        let mut index_sites = Vec::new();
        collect_df_index_sites(tree.root_node(), src.as_bytes(), &mut index_sites);
        assert!(
            index_sites
                .iter()
                .any(|s| s.name.eq_ignore_ascii_case("Z9ZW_IDX"))
        );

        let mut table_indexes = Vec::new();
        collect_df_table_indexes(tree.root_node(), src.as_bytes(), &mut table_indexes);
        assert!(table_indexes.iter().any(|i| {
            i.table.eq_ignore_ascii_case("Z9ZW_MSTR") && i.index.eq_ignore_ascii_case("Z9ZW_IDX")
        }));
        let idx = table_indexes
            .iter()
            .find(|i| i.index.eq_ignore_ascii_case("Z9ZW_IDX"))
            .expect("index fields");
        assert_eq!(idx.fields, vec!["z9zw_id"]);
    }

    #[test]
    fn parses_quoted_helpers() {
        assert_eq!(unquote(r#""abc""#), Some("abc"));
        assert_eq!(unquote("'abc'"), Some("abc"));
        assert_eq!(unquote("abc"), None);

        assert_eq!(
            extract_first_quoted(r#"FORMAT "x(24)" EXTENT 1"#).as_deref(),
            Some("x(24)")
        );
        assert_eq!(
            extract_first_quoted("LABEL 'Identifier'").as_deref(),
            Some("Identifier")
        );
        assert_eq!(extract_first_quoted("NO-QUOTES"), None);
    }

    #[test]
    fn extracts_index_field_names_from_index_lines_only() {
        let raw = r#"
ADD INDEX "idx" ON "tt"
  INDEX-FIELD "a" ASC
  UNIQUE
  INDEX-FIELD "b" DESC
."#;
        assert_eq!(extract_index_field_names(raw), vec!["a", "b"]);
    }
}
