use tower_lsp::lsp_types::{CompletionItemKind, Position, Range};
use tree_sitter::Node;

pub struct AblSymbol {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: String,
}

pub struct AblDefinitionSite {
    pub label: String,
    pub range: Range,
    pub start_byte: usize,
}

fn completion_kind_for_node(node_kind: &str) -> Option<(CompletionItemKind, &'static str)> {
    use CompletionItemKind as Kind;

    let entry = match node_kind {
        "variable_definition" | "parameter_definition" => (Kind::VARIABLE, "ABL variable"),
        "function_definition" | "function_forward_definition" => (Kind::FUNCTION, "ABL function"),
        "procedure_definition" | "procedure_forward_definition" => {
            (Kind::FUNCTION, "ABL procedure")
        }
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
        } else {
            // Fallback for definitions without a named "name" field in older grammars.
            find_first_identifier(node, src, kind, &detail, out);
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_definition_symbols(ch, src, out);
        }
    }
}

/// Walks the syntax tree and extracts locations for all definition names.
pub fn collect_definition_sites(node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if completion_kind_for_node(node.kind()).is_some() {
        if let Some(name) = node.child_by_field_name("name") {
            push_site(name, src, out);
        } else {
            find_first_identifier_site(node, src, out);
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_definition_sites(ch, src, out);
        }
    }
}

/// Walks the syntax tree and extracts locations for function definition names only.
pub fn collect_function_definition_sites(node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if is_function_definition_node(node.kind()) {
        if let Some(name) = node.child_by_field_name("name") {
            push_site(name, src, out);
        } else {
            find_first_identifier_site(node, src, out);
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
    if let Ok(name) = name_node.utf8_text(src) {
        let label = name.trim();
        if !label.is_empty() {
            out.push(AblSymbol {
                label: label.to_string(),
                kind,
                detail: detail.to_string(),
            });
        }
    }
}

fn find_first_identifier(
    node: Node,
    src: &[u8],
    kind: CompletionItemKind,
    detail: &str,
    out: &mut Vec<AblSymbol>,
) {
    if node.kind() == "identifier" {
        if let Ok(name) = node.utf8_text(src) {
            let label = name.trim();
            if !label.is_empty() {
                out.push(AblSymbol {
                    label: label.to_string(),
                    kind,
                    detail: detail.to_string(),
                });
            }
        }
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            find_first_identifier(ch, src, kind, detail, out);
        }
    }
}

fn push_site(name_node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if let Ok(name) = name_node.utf8_text(src) {
        let label = name.trim();
        if !label.is_empty() {
            out.push(AblDefinitionSite {
                label: label.to_string(),
                range: Range::new(
                    point_to_position(name_node.start_position()),
                    point_to_position(name_node.end_position()),
                ),
                start_byte: name_node.start_byte(),
            });
        }
    }
}

fn find_first_identifier_site(node: Node, src: &[u8], out: &mut Vec<AblDefinitionSite>) {
    if node.kind() == "identifier" {
        push_site(node, src, out);
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            find_first_identifier_site(ch, src, out);
        }
    }
}

fn point_to_position(point: tree_sitter::Point) -> Position {
    Position::new(point.row as u32, point.column as u32)
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
