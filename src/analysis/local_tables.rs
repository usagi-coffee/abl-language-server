use tree_sitter::Node;

use crate::backend::DbFieldInfo;

pub struct LocalTableDefinition {
    pub name_upper: String,
    pub fields: Vec<DbFieldInfo>,
    pub like_table_upper: Option<String>,
}

pub fn collect_local_table_definitions(
    node: Node<'_>,
    src: &[u8],
    out: &mut Vec<LocalTableDefinition>,
) {
    if is_local_table_definition_node(node.kind())
        && let Some(def) = parse_local_table_definition(node, src)
    {
        out.push(def);
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_local_table_definitions(ch, src, out);
        }
    }
}

fn is_local_table_definition_node(kind: &str) -> bool {
    matches!(
        kind,
        "temp_table_definition" | "work_table_definition" | "workfile_definition"
    )
}

fn parse_local_table_definition(node: Node<'_>, src: &[u8]) -> Option<LocalTableDefinition> {
    let name = node
        .child_by_field_name("name")
        .and_then(|n| n.utf8_text(src).ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_uppercase())?;

    let mut fields = Vec::<DbFieldInfo>::new();
    collect_local_table_fields(node, src, &mut fields);
    fields.sort_by(|a, b| {
        a.name
            .to_ascii_uppercase()
            .cmp(&b.name.to_ascii_uppercase())
            .then(a.name.cmp(&b.name))
    });
    fields.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));

    Some(LocalTableDefinition {
        name_upper: name,
        fields,
        like_table_upper: extract_like_table_upper(node, src),
    })
}

fn collect_local_table_fields(node: Node<'_>, src: &[u8], out: &mut Vec<DbFieldInfo>) {
    if matches!(node.kind(), "temp_table_field" | "field")
        && let Some(name_node) = node.child_by_field_name("name")
        && let Ok(name) = name_node.utf8_text(src)
    {
        let name = name.trim();
        if !name.is_empty() {
            let field_type = node
                .child_by_field_name("type")
                .and_then(|n| n.utf8_text(src).ok())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);
            out.push(DbFieldInfo {
                name: name.to_string(),
                field_type,
                format: None,
                label: None,
                description: None,
            });
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_local_table_fields(ch, src, out);
        }
    }
}

fn extract_like_table_upper(node: Node<'_>, src: &[u8]) -> Option<String> {
    for i in 0..node.child_count() {
        let Some(ch) = node.child(i as u32) else {
            continue;
        };
        if !matches!(ch.kind(), "like_phrase" | "like_sequential_phrase") {
            continue;
        }
        let Some(like_node) = ch.child_by_field_name("like") else {
            continue;
        };
        let Ok(raw_like) = like_node.utf8_text(src) else {
            continue;
        };
        let like = raw_like
            .trim()
            .split('[')
            .next()
            .unwrap_or_default()
            .trim()
            .split('.')
            .next_back()
            .unwrap_or_default()
            .trim();
        if !like.is_empty() {
            return Some(like.to_ascii_uppercase());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::collect_local_table_definitions;

    #[test]
    fn collects_temp_table_and_work_table_fields() {
        let src = r#"
DEFINE TEMP-TABLE ttOrder NO-UNDO
  FIELD ordNo AS INTEGER
  FIELD ordName AS CHARACTER.

DEFINE WORK-TABLE wtCust NO-UNDO
  FIELD custNum AS INTEGER.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut defs = Vec::new();
        collect_local_table_definitions(tree.root_node(), src.as_bytes(), &mut defs);

        let tt = defs
            .iter()
            .find(|d| d.name_upper == "TTORDER")
            .expect("temp-table definition");
        assert!(
            tt.fields
                .iter()
                .any(|f| f.name.eq_ignore_ascii_case("ordNo"))
        );
        assert!(
            tt.fields
                .iter()
                .any(|f| f.name.eq_ignore_ascii_case("ordName"))
        );

        let wt = defs
            .iter()
            .find(|d| d.name_upper == "WTCUST")
            .expect("work-table definition");
        assert!(
            wt.fields
                .iter()
                .any(|f| f.name.eq_ignore_ascii_case("custNum"))
        );
    }

    #[test]
    fn collects_like_table_reference() {
        let src = r#"
DEFINE TEMP-TABLE ttCustomer LIKE sports.Customer NO-UNDO.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut defs = Vec::new();
        collect_local_table_definitions(tree.root_node(), src.as_bytes(), &mut defs);
        let tt = defs
            .iter()
            .find(|d| d.name_upper == "TTCUSTOMER")
            .expect("temp-table definition");
        assert_eq!(tt.like_table_upper.as_deref(), Some("CUSTOMER"));
    }
}
