use std::collections::HashSet;
use tower_lsp::lsp_types::{Position, Range};
use tree_sitter::Node;

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
            name: name.to_ascii_uppercase(),
            range: Range::new(
                point_to_position(table_node.start_position()),
                point_to_position(table_node.end_position()),
            ),
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
            name: name.to_ascii_uppercase(),
            range: Range::new(
                point_to_position(field_node.start_position()),
                point_to_position(field_node.end_position()),
            ),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_field_sites(ch, src, out);
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
            name: name.to_ascii_uppercase(),
            range: Range::new(
                point_to_position(index_node.start_position()),
                point_to_position(index_node.end_position()),
            ),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_df_index_sites(ch, src, out);
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

fn point_to_position(point: tree_sitter::Point) -> Position {
    Position::new(point.row as u32, point.column as u32)
}

#[cfg(test)]
mod tests {
    use super::{
        collect_df_field_sites, collect_df_index_sites, collect_df_table_names,
        collect_df_table_sites,
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
        assert!(table_sites.iter().any(|s| s.name == "Z9ZW_MSTR"));

        let mut field_sites = Vec::new();
        collect_df_field_sites(tree.root_node(), src.as_bytes(), &mut field_sites);
        assert!(field_sites.iter().any(|s| s.name == "Z9ZW_ID"));

        let mut index_sites = Vec::new();
        collect_df_index_sites(tree.root_node(), src.as_bytes(), &mut index_sites);
        assert!(index_sites.iter().any(|s| s.name == "Z9ZW_IDX"));
    }
}
