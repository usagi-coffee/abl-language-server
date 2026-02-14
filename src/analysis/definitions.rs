use tower_lsp::lsp_types::{CompletionItemKind, Range};
use tree_sitter::Node;

use crate::utils::ts::{first_descendant_by_kind, node_to_range, node_trimmed_text};

pub struct AblSymbol {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: String,
    pub start_byte: usize,
}

pub struct AblDefinitionSite {
    pub label: String,
    pub range: Range,
    pub start_byte: usize,
}

#[derive(Clone)]
pub struct PreprocessorDefineSite {
    pub label: String,
    pub value: Option<String>,
    pub range: Range,
    pub start_byte: usize,
    pub is_global: bool,
}

fn completion_kind_for_node(node_kind: &str) -> Option<(CompletionItemKind, &'static str)> {
    use CompletionItemKind as Kind;

    let entry = match node_kind {
        "variable_definition" | "parameter_definition" | "parameter" => {
            (Kind::VARIABLE, "ABL variable")
        }
        "function_definition" | "function_forward_definition" => (Kind::FUNCTION, "ABL function"),
        "procedure_definition" => (Kind::FUNCTION, "ABL procedure"),
        "method_definition" => (Kind::METHOD, "ABL method"),
        "constructor_definition" => (Kind::CONSTRUCTOR, "ABL constructor"),
        "destructor_definition" => (Kind::METHOD, "ABL destructor"),
        "class_definition" => (Kind::CLASS, "ABL class"),
        "interface_definition" => (Kind::INTERFACE, "ABL interface"),
        "property_definition" => (Kind::PROPERTY, "ABL property"),
        "event_definition" => (Kind::EVENT, "ABL event"),
        "buffer_definition" => (Kind::VARIABLE, "ABL buffer"),
        "dataset_definition"
        | "temp_table_definition"
        | "work_table_definition"
        | "workfile_definition"
        | "query_definition"
        | "data_source_definition" => (Kind::STRUCT, "ABL data definition"),
        "stream_definition" => (Kind::VARIABLE, "ABL stream"),
        "browse_definition"
        | "button_definition"
        | "frame_definition"
        | "image_definition"
        | "menu_definition"
        | "submenu_definition"
        | "rectangle_definition" => (Kind::VARIABLE, "ABL UI definition"),
        _ if node_kind.ends_with("_definition") || node_kind.ends_with("_forward_definition") => {
            (Kind::VARIABLE, "ABL definition")
        }
        _ => return None,
    };

    Some(entry)
}

/// Walks the syntax tree and extracts names from all ABL definition nodes.
pub fn collect_definition_symbols(node: Node, src: &[u8], out: &mut Vec<AblSymbol>) {
    if let Some((kind, default_detail)) = completion_kind_for_node(node.kind()) {
        let detail = symbol_detail(node, src, default_detail);
        if let Some(name) = node.child_by_field_name("name") {
            push_symbol(name, src, kind, &detail, out);
        } else if let Some(name) = first_descendant_by_kind(node, "identifier") {
            // Fallback for definitions without a named "name" field in older grammars.
            push_symbol(name, src, kind, &detail, out);
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_definition_symbols(ch, src, out);
        }
    }
}

/// Walks the syntax tree and extracts names from preprocessor define directives.
pub fn collect_preprocessor_define_symbols(node: Node, src: &[u8], out: &mut Vec<AblSymbol>) {
    collect_preprocessor_define_symbols_internal(node, src, out, true);
}

pub fn collect_global_preprocessor_define_symbols(
    node: Node,
    src: &[u8],
    out: &mut Vec<AblSymbol>,
) {
    collect_preprocessor_define_symbols_internal(node, src, out, false);
}

pub fn collect_preprocessor_define_sites(
    node: Node,
    src: &[u8],
    out: &mut Vec<PreprocessorDefineSite>,
) {
    collect_preprocessor_define_sites_internal(node, src, out, true);
}

pub fn collect_global_preprocessor_define_sites(
    node: Node,
    src: &[u8],
    out: &mut Vec<PreprocessorDefineSite>,
) {
    collect_preprocessor_define_sites_internal(node, src, out, false);
}

fn collect_preprocessor_define_symbols_internal(
    node: Node,
    src: &[u8],
    out: &mut Vec<AblSymbol>,
    include_scoped: bool,
) {
    let is_global_define = node.kind() == "global_define_preprocessor_directive";
    let is_scoped_define = node.kind() == "scoped_define_preprocessor_directive";

    if (is_global_define || (include_scoped && is_scoped_define))
        && let Some(name) = node.child_by_field_name("name")
        && let Some(raw_name) = node_trimmed_text(name, src)
    {
        out.push(AblSymbol {
            label: format!("{{&{raw_name}}}"),
            kind: CompletionItemKind::CONSTANT,
            detail: "ABL preprocessor define".to_string(),
            start_byte: name.start_byte(),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_preprocessor_define_symbols_internal(ch, src, out, include_scoped);
        }
    }
}

fn collect_preprocessor_define_sites_internal(
    node: Node,
    src: &[u8],
    out: &mut Vec<PreprocessorDefineSite>,
    include_scoped: bool,
) {
    let is_global_define = node.kind() == "global_define_preprocessor_directive";
    let is_scoped_define = node.kind() == "scoped_define_preprocessor_directive";

    if (is_global_define || (include_scoped && is_scoped_define))
        && let Some(name) = node.child_by_field_name("name")
        && let Some(raw_name) = node_trimmed_text(name, src)
    {
        let value = node
            .child_by_field_name("value")
            .and_then(|n| n.utf8_text(src).ok())
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        out.push(PreprocessorDefineSite {
            label: raw_name,
            value,
            range: node_to_range(name),
            start_byte: name.start_byte(),
            is_global: is_global_define,
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_preprocessor_define_sites_internal(ch, src, out, include_scoped);
        }
    }
}

/// Walks the syntax tree and extracts locations for all definition names.
pub fn collect_definition_sites(node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if completion_kind_for_node(node.kind()).is_some() {
        if let Some(name) = node.child_by_field_name("name") {
            push_site(name, src, out);
        } else if let Some(name) = first_descendant_by_kind(node, "identifier") {
            push_site(name, src, out);
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_definition_sites(ch, src, out);
        }
    }
}

/// Walks the syntax tree and extracts locations for local table field names.
pub fn collect_local_table_field_sites(node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if matches!(node.kind(), "temp_table_field" | "field")
        && let Some(name) = node.child_by_field_name("name")
    {
        push_site(name, src, out);
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_local_table_field_sites(ch, src, out);
        }
    }
}

/// Walks the syntax tree and extracts locations for function definition names only.
pub fn collect_function_definition_sites(node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if is_function_definition_node(node.kind()) {
        if let Some(name) = node.child_by_field_name("name") {
            push_site(name, src, out);
        } else if let Some(name) = first_descendant_by_kind(node, "identifier") {
            push_site(name, src, out);
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_definition_sites(ch, src, out);
        }
    }
}

fn push_symbol(
    name_node: Node,
    src: &[u8],
    kind: CompletionItemKind,
    detail: &str,
    out: &mut Vec<AblSymbol>,
) {
    if let Some(label) = node_trimmed_text(name_node, src) {
        out.push(AblSymbol {
            label,
            kind,
            detail: detail.to_string(),
            start_byte: name_node.start_byte(),
        });
    }
}

fn push_site(name_node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if let Some(label) = node_trimmed_text(name_node, src) {
        out.push(AblDefinitionSite {
            label,
            range: node_to_range(name_node),
            start_byte: name_node.start_byte(),
        });
    }
}

fn symbol_detail(node: Node, src: &[u8], default_detail: &'static str) -> String {
    if let Some(type_node) = node.child_by_field_name("type")
        && let Ok(ty) = type_node.utf8_text(src)
    {
        let ty = ty.trim();
        if !ty.is_empty() {
            return ty.to_string();
        }
    }

    default_detail.to_string()
}

fn is_function_definition_node(node_kind: &str) -> bool {
    matches!(
        node_kind,
        "function_definition" | "function_forward_definition"
    )
}

#[cfg(test)]
mod tests {
    use super::{
        collect_definition_symbols, collect_global_preprocessor_define_sites,
        collect_global_preprocessor_define_symbols, collect_local_table_field_sites,
        collect_preprocessor_define_sites, collect_preprocessor_define_symbols,
    };

    #[test]
    fn collects_function_parameters_as_symbols() {
        let src = r#"
FUNCTION local_mul RETURNS INTEGER (INPUT p_a AS INTEGER, INPUT p_b AS INTEGER):
  RETURN p_a * p_b.
END FUNCTION.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut symbols = Vec::new();
        collect_definition_symbols(tree.root_node(), src.as_bytes(), &mut symbols);

        assert!(symbols.iter().any(|s| s.label.eq_ignore_ascii_case("p_a")));
        assert!(symbols.iter().any(|s| s.label.eq_ignore_ascii_case("p_b")));
    }

    #[test]
    fn collects_preprocessor_define_symbols_for_completion() {
        let src = r#"
&SCOPED-DEFINE Test "test"
&GLOBAL-DEFINE APP_MODE "dev"
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut symbols = Vec::new();
        collect_preprocessor_define_symbols(tree.root_node(), src.as_bytes(), &mut symbols);

        assert!(symbols.iter().any(|s| s.label == "{&Test}"));
        assert!(symbols.iter().any(|s| s.label == "{&APP_MODE}"));
    }

    #[test]
    fn collects_only_global_preprocessor_define_symbols_when_requested() {
        let src = r#"
&SCOPED-DEFINE Test "test"
&GLOBAL-DEFINE APP_MODE "dev"
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut symbols = Vec::new();
        collect_global_preprocessor_define_symbols(tree.root_node(), src.as_bytes(), &mut symbols);

        assert!(!symbols.iter().any(|s| s.label == "{&Test}"));
        assert!(symbols.iter().any(|s| s.label == "{&APP_MODE}"));
    }

    #[test]
    fn collects_preprocessor_define_sites_with_values() {
        let src = r#"
&SCOPED-DEFINE Test "test"
&GLOBAL-DEFINE APP_MODE "dev"
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut all_sites = Vec::new();
        collect_preprocessor_define_sites(tree.root_node(), src.as_bytes(), &mut all_sites);
        assert!(all_sites.iter().any(|s| s.label == "Test" && !s.is_global));
        assert!(all_sites.iter().any(|s| s.label == "APP_MODE"
            && s.is_global
            && s.value.as_deref() == Some("\"dev\"")));

        let mut global_only = Vec::new();
        collect_global_preprocessor_define_sites(
            tree.root_node(),
            src.as_bytes(),
            &mut global_only,
        );
        assert!(global_only.iter().all(|s| s.is_global));
        assert!(!global_only.iter().any(|s| s.label == "Test"));
    }

    #[test]
    fn collects_local_table_field_sites() {
        let src = r#"
DEFINE TEMP-TABLE ttCustomer NO-UNDO
  FIELD custNum AS INTEGER
  FIELD custName AS CHARACTER.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut sites = Vec::new();
        collect_local_table_field_sites(tree.root_node(), src.as_bytes(), &mut sites);
        assert!(
            sites
                .iter()
                .any(|s| s.label.eq_ignore_ascii_case("custNum"))
        );
        assert!(
            sites
                .iter()
                .any(|s| s.label.eq_ignore_ascii_case("custName"))
        );
    }
}
