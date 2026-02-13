use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::definitions::collect_definition_symbols;
use crate::analysis::functions::normalize_function_name;
use crate::analysis::includes::collect_include_sites;
use crate::analysis::local_tables::collect_local_table_definitions;
use crate::backend::Backend;
use crate::utils::ts::{
    collect_nodes_by_kind, count_nodes_by_kind, direct_child_by_kind, node_to_range,
};

const MAX_SYNTAX_DIAGNOSTICS_PER_CHANGE: usize = 64;

pub async fn on_change(
    backend: &Backend,
    uri: Url,
    version: i32,
    text: String,
    include_semantic_diags: bool,
) {
    if !should_accept_version(backend, &uri, version) {
        return;
    }

    backend.doc_versions.insert(uri.clone(), version);
    backend.docs.insert(uri.clone(), text.to_owned());

    if !is_latest_version(backend, &uri, version) {
        return;
    }

    let diagnostics_enabled = backend.config.lock().await.diagnostics.enabled;
    let parsed_tree = {
        let parser_mutex = backend
            .abl_parsers
            .entry(uri.clone())
            .or_insert_with(|| std::sync::Mutex::new(backend.new_abl_parser()));
        let mut parser = parser_mutex.lock().expect("ABL parser mutex poisoned");
        if !is_latest_version(backend, &uri, version) {
            return;
        }
        parser.parse(text.clone(), None)
    };
    let tree = match parsed_tree {
        Some(t) => t,
        None => {
            if !is_latest_version(backend, &uri, version) {
                return;
            }
            backend
                .client
                .publish_diagnostics(uri.clone(), vec![], Some(version))
                .await;
            return;
        }
    };

    if !is_latest_version(backend, &uri, version) {
        return;
    }

    if !diagnostics_enabled {
        backend
            .client
            .publish_diagnostics(uri.clone(), vec![], Some(version))
            .await;
        if !is_latest_version(backend, &uri, version) {
            return;
        }
        backend.trees.insert(uri, tree);
        return;
    }

    let mut diags: Vec<Diagnostic> = Vec::new();
    collect_ts_error_diags(
        tree.root_node(),
        &mut diags,
        MAX_SYNTAX_DIAGNOSTICS_PER_CHANGE,
    );
    if !collect_function_call_arity_diags(
        backend,
        &uri,
        version,
        &text,
        tree.root_node(),
        include_semantic_diags,
        &mut diags,
    )
    .await
    {
        return;
    }
    if !collect_unknown_symbol_diags(
        backend,
        &uri,
        version,
        &text,
        tree.root_node(),
        include_semantic_diags,
        &mut diags,
    )
    .await
    {
        return;
    }
    // Keep lightweight assignment type checks active for on-change diagnostics.
    collect_assignment_type_diags(tree.root_node(), text.as_bytes(), &mut diags);
    collect_function_call_arg_type_diags(tree.root_node(), text.as_bytes(), &mut diags);
    if !is_latest_version(backend, &uri, version) {
        return;
    }
    backend
        .client
        .publish_diagnostics(uri.clone(), diags, Some(version))
        .await;

    if !is_latest_version(backend, &uri, version) {
        return;
    }
    backend.trees.insert(uri, tree);
}

async fn collect_function_call_arity_diags(
    backend: &Backend,
    uri: &Url,
    version: i32,
    text: &str,
    root: Node<'_>,
    include_from_includes: bool,
    out: &mut Vec<Diagnostic>,
) -> bool {
    if !is_latest_version(backend, uri, version) {
        return false;
    }

    let mut signatures = HashMap::<String, Vec<usize>>::new();
    collect_function_arities(root, text.as_bytes(), &mut signatures);

    // Include signatures from directly included files only on full semantic pass.
    if include_from_includes && let Ok(current_path) = uri.to_file_path() {
        let include_sites = collect_include_sites(text);
        let mut seen = HashSet::<PathBuf>::new();
        let mut include_parser = backend.new_abl_parser();
        for include in include_sites {
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            let Some(path) = backend
                .resolve_include_path_for(&current_path, &include.path)
                .await
            else {
                continue;
            };
            if !seen.insert(path.clone()) {
                continue;
            }

            let Ok(include_text) = tokio::fs::read_to_string(&path).await else {
                continue;
            };
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            let include_tree = include_parser.parse(&include_text, None);
            let Some(include_tree) = include_tree else {
                continue;
            };
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            collect_function_arities(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut signatures,
            );
        }
    }

    if !is_latest_version(backend, uri, version) {
        return false;
    }

    for arities in signatures.values_mut() {
        arities.sort_unstable();
        arities.dedup();
    }

    let mut calls = Vec::<FunctionCallSite>::new();
    collect_function_calls(root, text.as_bytes(), &mut calls);
    for call in calls {
        let Some(expected_set) = signatures.get(&call.name_upper) else {
            continue;
        };
        if expected_set.contains(&call.arg_count) {
            continue;
        }

        let expected = expected_set
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" or ");
        out.push(Diagnostic {
            range: call.range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("abl-semantic".into()),
            message: format!(
                "Function '{}' expects {} argument(s), got {}",
                call.display_name, expected, call.arg_count
            ),
            ..Default::default()
        });
    }

    true
}

fn should_accept_version(backend: &Backend, uri: &Url, version: i32) -> bool {
    match backend.doc_versions.get(uri) {
        Some(current) => *current <= version,
        None => true,
    }
}

fn is_latest_version(backend: &Backend, uri: &Url, version: i32) -> bool {
    matches!(backend.doc_versions.get(uri), Some(current) if *current == version)
}

fn collect_function_arities(node: Node<'_>, src: &[u8], out: &mut HashMap<String, Vec<usize>>) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) {
        let name = node
            .child_by_field_name("name")
            .and_then(|n| n.utf8_text(src).ok())
            .map(normalize_function_name);
        if let Some(name_upper) = name {
            let arity = function_param_count(node, src);
            out.entry(name_upper).or_default().push(arity);
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_arities(ch, src, out);
        }
    }
}

fn function_param_count(function_node: Node<'_>, src: &[u8]) -> usize {
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let count = count_nodes_by_kind(parameters_node, "parameter");
        if count > 0 {
            return count;
        }
    }

    // Fallback for alternative grammar forms.
    let mut count = 0usize;
    count_parameter_definitions(function_node, &mut count, true);
    let _ = src;
    count
}

fn count_parameter_definitions(node: Node<'_>, out: &mut usize, is_root: bool) {
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
    if node.kind() == "parameter_definition" {
        *out += 1;
        return;
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            count_parameter_definitions(ch, out, false);
        }
    }
}

fn collect_function_calls(node: Node<'_>, src: &[u8], out: &mut Vec<FunctionCallSite>) {
    if node.kind() == "function_call" {
        let function_node = node.child_by_field_name("function");
        let display_name = function_node
            .and_then(|n| n.utf8_text(src).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        if let Some(display_name) = display_name {
            let name_upper = normalize_function_name(&display_name);
            let arg_count = node
                .children(&mut node.walk())
                .find(|n| n.kind() == "arguments")
                .map(|args| count_argument_nodes(args))
                .unwrap_or(0);

            let target_node = function_node.unwrap_or(node);
            out.push(FunctionCallSite {
                display_name,
                name_upper,
                arg_count,
                range: node_to_range(target_node),
            });
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_calls(ch, src, out);
        }
    }
}

fn count_argument_nodes(arguments_node: Node<'_>) -> usize {
    let mut count = 0usize;
    for i in 0..arguments_node.child_count() {
        if let Some(ch) = arguments_node.child(i as u32)
            && ch.kind() == "argument"
        {
            count += 1;
        }
    }
    count
}

async fn collect_unknown_symbol_diags(
    backend: &Backend,
    uri: &Url,
    version: i32,
    text: &str,
    root: Node<'_>,
    include_semantic_diags: bool,
    out: &mut Vec<Diagnostic>,
) -> bool {
    // Lightweight on-change pass intentionally skips include parsing.
    // Unknown-symbol diagnostics would otherwise flap for include-provided symbols
    // until the full save/open semantic pass runs.
    if !include_semantic_diags {
        return true;
    }

    if !is_latest_version(backend, uri, version) {
        return false;
    }

    let mut known_variables = HashSet::<String>::new();
    let mut known_functions = HashSet::<String>::new();
    collect_known_symbols(
        root,
        text.as_bytes(),
        &mut known_variables,
        &mut known_functions,
    );
    collect_local_table_field_symbols(backend, root, text.as_bytes(), &mut known_variables);

    if include_semantic_diags && let Ok(current_path) = uri.to_file_path() {
        let include_sites = collect_include_sites(text);
        let mut seen = HashSet::<PathBuf>::new();
        let mut include_parser = backend.new_abl_parser();
        for include in include_sites {
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            let Some(path) = backend
                .resolve_include_path_for(&current_path, &include.path)
                .await
            else {
                continue;
            };
            if !seen.insert(path.clone()) {
                continue;
            }
            let Ok(include_text) = tokio::fs::read_to_string(&path).await else {
                continue;
            };
            if !is_latest_version(backend, uri, version) {
                return false;
            }
            let Some(include_tree) = include_parser.parse(&include_text, None) else {
                continue;
            };
            collect_known_symbols(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut known_variables,
                &mut known_functions,
            );
        }
    }

    let mut refs = Vec::<IdentifierRef>::new();
    collect_identifier_refs_for_unknown_symbol_diag(root, text.as_bytes(), &mut refs);
    refs.sort_by(|a, b| {
        a.range
            .start
            .line
            .cmp(&b.range.start.line)
            .then(a.range.start.character.cmp(&b.range.start.character))
            .then(a.name_upper.cmp(&b.name_upper))
    });
    refs.dedup_by(|a, b| a.name_upper == b.name_upper && a.range == b.range);
    let active_buffer_like_names = collect_active_buffer_like_names(root, text.as_bytes(), backend);
    let active_table_fields =
        collect_active_db_table_field_symbols(backend, &active_buffer_like_names);

    for r in refs {
        if known_variables.contains(&r.name_upper)
            || backend.db_tables.contains(&r.name_upper)
            || active_table_fields.contains(&r.name_upper)
            || is_builtin_variable_name(&r.name_upper)
            || is_builtin_function_name(&r.name_upper)
            || looks_like_table_field_reference(&r.name_upper, &active_buffer_like_names)
        {
            continue;
        }
        out.push(Diagnostic {
            range: r.range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("abl-semantic".into()),
            message: format!("Unknown variable '{}'", r.display_name),
            ..Default::default()
        });
    }

    let mut calls = Vec::<FunctionCallSite>::new();
    collect_function_calls(root, text.as_bytes(), &mut calls);
    for call in calls {
        if known_functions.contains(&call.name_upper)
            || is_builtin_function_name(&call.name_upper)
            || call.display_name.contains('.')
            || call.display_name.contains(':')
        {
            continue;
        }
        out.push(Diagnostic {
            range: call.range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("abl-semantic".into()),
            message: format!("Unknown function '{}'", call.display_name),
            ..Default::default()
        });
    }

    true
}

fn collect_local_table_field_symbols(
    backend: &Backend,
    root: Node<'_>,
    src: &[u8],
    known_variables: &mut HashSet<String>,
) {
    let mut defs = Vec::new();
    collect_local_table_definitions(root, src, &mut defs);
    for def in defs {
        for field in def.fields {
            let upper = field.name.trim().to_ascii_uppercase();
            if !upper.is_empty() {
                known_variables.insert(upper);
            }
        }
        if let Some(like_table_upper) = def.like_table_upper
            && let Some(fields) = backend.db_fields_by_table.get(&like_table_upper)
        {
            for field in fields.value().iter() {
                let upper = field.name.trim().to_ascii_uppercase();
                if !upper.is_empty() {
                    known_variables.insert(upper);
                }
            }
        }
    }
}

fn collect_active_buffer_like_names(
    root: Node<'_>,
    src: &[u8],
    backend: &Backend,
) -> HashSet<String> {
    let mut out = HashSet::<String>::new();

    let mut buffer_mappings = Vec::new();
    collect_buffer_mappings(root, src, &mut buffer_mappings);
    for mapping in buffer_mappings {
        let alias_upper = mapping.alias.trim().to_ascii_uppercase();
        if !alias_upper.is_empty() {
            out.insert(alias_upper);
        }
        let table_upper = mapping.table.trim().to_ascii_uppercase();
        if !table_upper.is_empty() {
            out.insert(table_upper);
        }
    }

    let mut identifiers = Vec::<Node>::new();
    collect_nodes_by_kind(root, "identifier", &mut identifiers);
    for ident in identifiers {
        let Ok(name_raw) = ident.utf8_text(src) else {
            continue;
        };
        let name_upper = name_raw.trim().to_ascii_uppercase();
        if name_upper.is_empty() {
            continue;
        }
        if backend.db_tables.contains(&name_upper) {
            out.insert(name_upper);
        }
    }

    out
}

fn collect_active_db_table_field_symbols(
    backend: &Backend,
    active_table_like_names: &HashSet<String>,
) -> HashSet<String> {
    let mut out = HashSet::<String>::new();
    for table_like in active_table_like_names {
        let Some(fields) = backend.db_fields_by_table.get(table_like) else {
            continue;
        };
        for field in fields.value().iter() {
            let upper = field.name.trim().to_ascii_uppercase();
            if !upper.is_empty() {
                out.insert(upper);
            }
        }
    }
    out
}

fn looks_like_table_field_reference(name_upper: &str, active_buffers: &HashSet<String>) -> bool {
    if name_upper.is_empty() || active_buffers.is_empty() {
        return false;
    }
    for buffer in active_buffers {
        if looks_like_prefixed_field(name_upper, buffer)
            || table_field_prefix_from_table_like_name(buffer)
                .is_some_and(|prefix| looks_like_prefixed_field(name_upper, &prefix))
        {
            return true;
        }
    }
    false
}

fn table_field_prefix_from_table_like_name(name_upper: &str) -> Option<String> {
    let trimmed = name_upper.trim();
    if trimmed.is_empty() {
        return None;
    }
    for sep in ['_', '-'] {
        if let Some(idx) = trimmed.find(sep)
            && idx > 0
        {
            let mut prefix = trimmed[..idx].to_string();
            prefix.push('_');
            return Some(prefix);
        }
    }
    None
}

fn looks_like_prefixed_field(name_upper: &str, prefix_upper: &str) -> bool {
    if !name_upper.starts_with(prefix_upper) || name_upper.len() <= prefix_upper.len() {
        return false;
    }
    let suffix = &name_upper[prefix_upper.len()..];
    let Some(first) = suffix.chars().next() else {
        return false;
    };
    first.is_ascii_alphabetic() || first == '_'
}

fn collect_known_symbols(
    root: Node<'_>,
    src: &[u8],
    known_variables: &mut HashSet<String>,
    known_functions: &mut HashSet<String>,
) {
    let mut symbols = Vec::new();
    collect_definition_symbols(root, src, &mut symbols);
    for sym in symbols {
        let upper = sym.label.trim().to_ascii_uppercase();
        if upper.is_empty() {
            continue;
        }
        match sym.kind {
            CompletionItemKind::FUNCTION
            | CompletionItemKind::METHOD
            | CompletionItemKind::CONSTRUCTOR => {
                known_functions.insert(normalize_function_name(&upper));
            }
            _ => {
                known_variables.insert(upper);
            }
        }
    }
}

fn collect_identifier_refs_for_unknown_symbol_diag(
    node: Node<'_>,
    src: &[u8],
    out: &mut Vec<IdentifierRef>,
) {
    match node.kind() {
        "assignment_statement" => {
            if let Some(left) = node.child_by_field_name("left")
                && left.kind() == "identifier"
                && let Ok(name_raw) = left.utf8_text(src)
            {
                let display_name = name_raw.trim().to_string();
                if !display_name.is_empty() {
                    out.push(IdentifierRef {
                        name_upper: display_name.to_ascii_uppercase(),
                        display_name,
                        range: node_to_range(left),
                    });
                }
            }
            if let Some(right) = node.child_by_field_name("right") {
                collect_identifier_refs_from_expression(right, src, out);
            }
        }
        "return_statement" => {
            if let Some(value) = node
                .child_by_field_name("value")
                .or_else(|| node.named_child(0))
            {
                collect_identifier_refs_from_expression(value, src, out);
            }
        }
        "expression_statement" => {
            if let Some(expr) = node.named_child(0) {
                collect_identifier_refs_from_expression(expr, src, out);
            }
        }
        _ => {}
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_identifier_refs_for_unknown_symbol_diag(ch, src, out);
        }
    }
}

fn collect_identifier_refs_from_expression(
    expr: Node<'_>,
    src: &[u8],
    out: &mut Vec<IdentifierRef>,
) {
    match expr.kind() {
        "identifier" => {
            if let Ok(name_raw) = expr.utf8_text(src) {
                let display_name = name_raw.trim().to_string();
                if !display_name.is_empty() {
                    out.push(IdentifierRef {
                        name_upper: display_name.to_ascii_uppercase(),
                        display_name,
                        range: node_to_range(expr),
                    });
                }
            }
            return;
        }
        "qualified_name" | "widget_qualified_name" | "scoped_name" | "object_access" => return,
        "function_call" => {
            if let Some(args) = expr
                .children(&mut expr.walk())
                .find(|n| n.kind() == "arguments")
            {
                for arg in argument_exprs(args) {
                    collect_identifier_refs_from_expression(arg, src, out);
                }
            }
            return;
        }
        _ => {}
    }

    for i in 0..expr.child_count() {
        if let Some(ch) = expr.child(i as u32) {
            collect_identifier_refs_from_expression(ch, src, out);
        }
    }
}

fn is_builtin_function_name(name_upper: &str) -> bool {
    const BUILTIN_FUNCTIONS: &[&str] = &[
        "ABS",
        "ABSOLUTE",
        "ACCUM",
        "ADD-INTERVAL",
        "ALIAS",
        "AMBIGUOUS",
        "ASC",
        "AUDIT-ENABLED",
        "AVAILABLE",
        "BASE64-DECODE",
        "BASE64-ENCODE",
        "BOX",
        "BUFFER-GROUP-ID",
        "BUFFER-GROUP-NAME",
        "BUFFER-PARTITION-ID",
        "BUFFER-TENANT-ID",
        "BUFFER-TENANT-NAME",
        "CAN-DO",
        "CAN-FIND",
        "CAN-QUERY",
        "CAN-SET",
        "CAPS",
        "CAST",
        "CHR",
        "CODEPAGE-CONVERT",
        "COMPARE",
        "CONNECTED",
        "COUNT-OF",
        "CURRENT-CHANGED",
        "CURRENT-LANGUAGE",
        "CURRENT-RESULT-ROW",
        "CURRENT-VALUE",
        "DATASERVERS",
        "DATA-SOURCE-MODIFIED",
        "DATE",
        "DATETIME",
        "DATETIME-TZ",
        "DAY",
        "DBCODEPAGE",
        "DBCOLLATION",
        "DBNAME",
        "DBPARAM",
        "DB-REMOTE-HOST",
        "DBRESTRICTIONS",
        "DBTASKID",
        "DBTYPE",
        "DBVERSION",
        "DECIMAL",
        "DECRYPT",
        "DEFINED",
        "DYNAMIC-CAST",
        "DYNAMIC-CURRENT-VALUE",
        "DYNAMIC-ENUM",
        "DYNAMIC-FUNCTION",
        "DYNAMIC-INVOKE",
        "DYNAMIC-NEXT-VALUE",
        "DYNAMIC-PROPERTY",
        "ENCODE",
        "ENCRYPT",
        "ENTERED",
        "ENTRY",
        "ERROR",
        "ETIME",
        "EXP",
        "EXTENT",
        "FILL",
        "FIRST",
        "FIRST-OF",
        "FRAME-COL",
        "FRAME-DB",
        "FRAME-DOWN",
        "FRAME-FIELD",
        "FRAME-FILE",
        "FRAME-INDEX",
        "FRAME-LINE",
        "FRAME-NAME",
        "FRAME-ROW",
        "FRAME-VALUE",
        "GATEWAYS",
        "GENERATE-PBE-KEY",
        "GENERATE-PBE-SALT",
        "GENERATE-RANDOM-KEY",
        "GENERATE-UUID",
        "GET-BITS",
        "GET-BYTE",
        "GET-BYTE-ORDER",
        "GET-BYTES",
        "GET-CLASS",
        "GET-CODEPAGE",
        "GET-CODEPAGES",
        "GET-COLLATION",
        "GET-COLLATIONS",
        "GET-DB-CLIENT",
        "GET-DOUBLE",
        "GET-EFFECTIVE-TENANT-ID",
        "GET-EFFECTIVE-TENANT-NAME",
        "GET-FLOAT",
        "GET-INT64",
        "GET-LONG",
        "GET-POINTER-VALUE",
        "GET-SHORT",
        "GET-SIZE",
        "GET-STRING",
        "GET-UNSIGNED-LONG",
        "GET-UNSIGNED-SHORT",
        "GO-PENDING",
        "GUID",
        "HANDLE",
        "HASH-CODE",
        "HEX-DECODE",
        "HEX-ENCODE",
        "IF",
        "INDEX",
        "INPUT",
        "INT64",
        "INTEGER",
        "INTERVAL",
        "IS-ATTR-SPACE",
        "IS-CODEPAGE-FIXED",
        "IS-COLUMN-CODEPAGE",
        "IS-DB-MULTI-TENANT",
        "IS-LEAD-BYTE",
        "ISO-DATE",
        "KBLABEL",
        "KEYCODE",
        "KEYFUNCTION",
        "KEYLABEL",
        "KEYWORD",
        "KEYWORD-ALL",
        "LAST",
        "LASTKEY",
        "LAST-OF",
        "LC",
        "LDBNAME",
        "LEFT-TRIM",
        "LENGTH",
        "LIBRARY",
        "LINE-COUNTER",
        "LIST-EVENTS",
        "LIST-QUERY-ATTRS",
        "LIST-SET-ATTRS",
        "LIST-WIDGETS",
        "LOCKED",
        "LOG",
        "LOGICAL",
        "LOOKUP",
        "MAXIMUM",
        "MD5-DIGEST",
        "MEMBER",
        "MESSAGE-DIGEST",
        "MESSAGE-LINES",
        "MINIMUM",
        "MONTH",
        "MTIME",
        "NEXT-VALUE",
        "NORMALIZE",
        "NOT",
        "NOW",
        "NUM-ALIASES",
        "NUM-DBS",
        "NUM-ENTRIES",
        "NUM-RESULTS",
        "OPSYS",
        "OS-DRIVES",
        "OS-ERROR",
        "OS-GETENV",
        "PAGE-NUMBER",
        "PAGE-SIZE",
        "PDBNAME",
        "PROC-HANDLE",
        "PROC-STATUS",
        "PROCESS-ARCHITECTURE",
        "PROGRAM-NAME",
        "PROGRESS",
        "PROMSGS",
        "PROPATH",
        "PROVERSION",
        "QUERY-OFF-END",
        "QUOTER",
        "R-INDEX",
        "RANDOM",
        "RAW",
        "RECID",
        "RECORD-LENGTH",
        "REJECTED",
        "REPLACE",
        "RETRY",
        "RETURN-VALUE",
        "RGB-VALUE",
        "RIGHT-TRIM",
        "ROUND",
        "ROW-STATE",
        "ROWID",
        "SCREEN-LINES",
        "SDBNAME",
        "SEARCH",
        "SEEK",
        "SET-DB-CLIENT",
        "SET-EFFECTIVE-TENANT",
        "SETUSERID",
        "SHA1-DIGEST",
        "SQRT",
        "SSL-SERVER-NAME",
        "STRING",
        "SUBSTITUTE",
        "SUBSTRING",
        "SUPER",
        "TENANT-ID",
        "TENANT-NAME",
        "TENANT-NAME-TO-ID",
        "TERMINAL",
        "TIME",
        "TIMEZONE",
        "TODAY",
        "TO-ROWID",
        "TRANSACTION",
        "TRIM",
        "TRUNCATE",
        "TYPE-OF",
        "UNBOX",
        "USERID",
        "VALID-EVENT",
        "VALID-HANDLE",
        "VALID-OBJECT",
        "WEEKDAY",
        "WIDGET-HANDLE",
        "YEAR",
    ];
    const SQL_BUILTIN_FUNCTIONS: &[&str] = &[
        "ABS",
        "ACOS",
        "ADD_MONTHS",
        "ASCII",
        "ASIN",
        "ATAN",
        "ATAN2",
        "AVG",
        "CASE",
        "CAST",
        "CDC_GET_CHANGED_COLUMNS",
        "CDC_IS_COLUMN_CHANGED",
        "CEILING",
        "CHAR",
        "CHR",
        "COALESCE",
        "CONCAT",
        "CONVERT",
        "COS",
        "COUNT",
        "CURDATE",
        "CURTIME",
        "CURRVAL",
        "DATABASE",
        "DAYNAME",
        "DAYOFMONTH",
        "DAYOFWEEK",
        "DAYOFYEAR",
        "DB_NAME",
        "DECODE",
        "DEGREES",
        "EXP",
        "FLOOR",
        "GREATEST",
        "HOUR",
        "IFNULL",
        "INITCAP",
        "INSERT",
        "INSTR",
        "ISOWEEKDAY",
        "ISOWEEK",
        "ISOYEAR",
        "LAST_DAY",
        "LCASE",
        "LEAST",
        "LEFT",
        "LENGTH",
        "LOCATE",
        "LOG10",
        "LOWER",
        "LPAD",
        "LTRIM",
        "MAX",
        "MIN",
        "MINUTE",
        "MOD",
        "MONTH",
        "MONTHNAME",
        "MONTHS_BETWEEN",
        "NEXT_DAY",
        "NEXTVAL",
        "NOW",
        "NULLIF",
        "NVL",
        "PI",
        "POWER",
        "PREFIX",
        "PRO_ARR_DESCAPE",
        "PRO_ARR_ESCAPE",
        "PRO_ELEMENT",
        "QUARTER",
        "RADIANS",
        "RAND",
        "REPEAT",
        "REPLACE",
        "RIGHT",
        "ROUND",
        "ROWID",
        "RPAD",
        "RTRIM",
        "SECOND",
        "SIGN",
        "SIN",
        "SQRT",
        "SUBSTR",
        "SUBSTRING",
        "SUFFIX",
        "SUM",
        "SYSDATE",
        "SYSTIME",
        "SYSTIMESTAMP",
        "TAN",
        "TO_CHAR",
        "TO_DATE",
        "TO_NUMBER",
        "TO_TIME",
        "TO_TIMESTAMP",
        "TRANSLATE",
        "UCASE",
        "UPPER",
        "USER",
        "WEEK",
        "YEAR",
    ];
    BUILTIN_FUNCTIONS.contains(&name_upper) || SQL_BUILTIN_FUNCTIONS.contains(&name_upper)
}

fn is_builtin_variable_name(name_upper: &str) -> bool {
    const BUILTIN_VARIABLES: &[&str] = &[
        "SESSION",
        "ERROR-STATUS",
        "THIS-PROCEDURE",
        "SOURCE-PROCEDURE",
        "TARGET-PROCEDURE",
        "CURRENT-WINDOW",
        "DEFAULT-WINDOW",
        "ACTIVE-WINDOW",
        "SELF",
        "SUPER",
        "THIS-OBJECT",
    ];
    const GLOBAL_VARIABLE_EXCEPTIONS: &[&str] = &[
        // Project-level globals intentionally allowed without local declaration.
        "BATCHRUN",
    ];

    BUILTIN_VARIABLES.contains(&name_upper) || GLOBAL_VARIABLE_EXCEPTIONS.contains(&name_upper)
}

struct IdentifierRef {
    name_upper: String,
    display_name: String,
    range: Range,
}

struct FunctionCallSite {
    display_name: String,
    name_upper: String,
    arg_count: usize,
    range: Range,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BasicType {
    Character,
    Numeric,
    Logical,
    DateLike,
    Handle,
}

impl BasicType {
    fn label(self) -> &'static str {
        match self {
            Self::Character => "CHARACTER",
            Self::Numeric => "NUMERIC",
            Self::Logical => "LOGICAL",
            Self::DateLike => "DATE",
            Self::Handle => "HANDLE",
        }
    }
}

struct TypedBinding {
    name_upper: String,
    ty: BasicType,
    start_byte: usize,
}

#[derive(Clone)]
struct FunctionTypeSignature {
    param_types: Vec<Option<BasicType>>,
}

fn collect_assignment_type_diags(root: Node<'_>, src: &[u8], out: &mut Vec<Diagnostic>) {
    let mut bindings = Vec::<TypedBinding>::new();
    collect_typed_bindings(root, src, &mut bindings);

    if bindings.is_empty() {
        return;
    }

    let mut function_returns = HashMap::<String, BasicType>::new();
    collect_function_return_types(root, src, &mut function_returns);

    collect_assignment_type_diags_in_node(root, src, &bindings, &function_returns, out);
}

fn collect_function_call_arg_type_diags(root: Node<'_>, src: &[u8], out: &mut Vec<Diagnostic>) {
    let mut bindings = Vec::<TypedBinding>::new();
    collect_typed_bindings(root, src, &mut bindings);

    let mut function_returns = HashMap::<String, BasicType>::new();
    collect_function_return_types(root, src, &mut function_returns);

    let mut signatures = HashMap::<String, Vec<FunctionTypeSignature>>::new();
    collect_function_type_signatures(root, src, &mut signatures);

    collect_function_call_arg_type_diags_in_node(
        root,
        src,
        &bindings,
        &function_returns,
        &signatures,
        out,
    );
}

fn collect_typed_bindings(node: Node<'_>, src: &[u8], out: &mut Vec<TypedBinding>) {
    if matches!(node.kind(), "variable_definition" | "parameter_definition")
        && let (Some(name_node), Some(type_node)) = (
            node.child_by_field_name("name"),
            node.child_by_field_name("type"),
        )
        && let (Ok(name), Ok(raw_ty)) = (name_node.utf8_text(src), type_node.utf8_text(src))
        && let Some(ty) = parse_basic_type(raw_ty)
    {
        out.push(TypedBinding {
            name_upper: name.trim().to_ascii_uppercase(),
            ty,
            start_byte: name_node.start_byte(),
        });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_typed_bindings(ch, src, out);
        }
    }
}

fn collect_function_return_types(node: Node<'_>, src: &[u8], out: &mut HashMap<String, BasicType>) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) && let (Some(name_node), Some(type_node)) = (
        node.child_by_field_name("name"),
        node.child_by_field_name("type"),
    ) && let (Ok(name), Ok(raw_ty)) = (name_node.utf8_text(src), type_node.utf8_text(src))
        && let Some(ty) = parse_basic_type(raw_ty)
    {
        out.insert(normalize_function_name(name), ty);
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_return_types(ch, src, out);
        }
    }
}

fn collect_function_type_signatures(
    node: Node<'_>,
    src: &[u8],
    out: &mut HashMap<String, Vec<FunctionTypeSignature>>,
) {
    if matches!(
        node.kind(),
        "function_definition" | "function_forward_definition"
    ) && let Some(name_node) = node.child_by_field_name("name")
        && let Ok(name) = name_node.utf8_text(src)
    {
        let param_types = function_param_types(node, src);
        out.entry(normalize_function_name(name))
            .or_default()
            .push(FunctionTypeSignature { param_types });
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_type_signatures(ch, src, out);
        }
    }
}

fn collect_assignment_type_diags_in_node(
    node: Node<'_>,
    src: &[u8],
    bindings: &[TypedBinding],
    function_returns: &HashMap<String, BasicType>,
    out: &mut Vec<Diagnostic>,
) {
    if node.kind() == "assignment_statement"
        && let (Some(left), Some(right)) = (
            node.child_by_field_name("left"),
            node.child_by_field_name("right"),
        )
        && left.kind() == "identifier"
        && let Ok(name_raw) = left.utf8_text(src)
    {
        let left_name_upper = name_raw.trim().to_ascii_uppercase();
        if let Some(left_ty) = resolve_binding_type(bindings, &left_name_upper, left.start_byte())
            && let Some(right_ty) = infer_expr_type(right, src, bindings, function_returns)
            && left_ty != right_ty
        {
            out.push(Diagnostic {
                range: node_to_range(right),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("abl-semantic".into()),
                message: format!(
                    "Type mismatch: cannot assign {} to {} variable '{}'",
                    right_ty.label(),
                    left_ty.label(),
                    left_name_upper
                ),
                ..Default::default()
            });
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_assignment_type_diags_in_node(ch, src, bindings, function_returns, out);
        }
    }
}

fn resolve_binding_type(
    bindings: &[TypedBinding],
    name_upper: &str,
    at_byte: usize,
) -> Option<BasicType> {
    bindings
        .iter()
        .filter(|b| b.name_upper == name_upper && b.start_byte <= at_byte)
        .max_by_key(|b| b.start_byte)
        .map(|b| b.ty)
}

fn infer_expr_type(
    expr: Node<'_>,
    src: &[u8],
    bindings: &[TypedBinding],
    function_returns: &HashMap<String, BasicType>,
) -> Option<BasicType> {
    match expr.kind() {
        "string_literal" => Some(BasicType::Character),
        "number_literal" => Some(BasicType::Numeric),
        "boolean_literal" => Some(BasicType::Logical),
        "identifier" => expr
            .utf8_text(src)
            .ok()
            .map(|s| s.trim().to_ascii_uppercase())
            .and_then(|name| resolve_binding_type(bindings, &name, expr.start_byte())),
        "parenthesized_expression" => expr
            .named_child(0)
            .and_then(|inner| infer_expr_type(inner, src, bindings, function_returns)),
        "function_call" => {
            let function_name = expr
                .child_by_field_name("function")
                .and_then(|n| n.utf8_text(src).ok())
                .map(normalize_function_name)?;
            function_returns.get(&function_name).copied()
        }
        _ => None,
    }
}

fn collect_function_call_arg_type_diags_in_node(
    node: Node<'_>,
    src: &[u8],
    bindings: &[TypedBinding],
    function_returns: &HashMap<String, BasicType>,
    signatures: &HashMap<String, Vec<FunctionTypeSignature>>,
    out: &mut Vec<Diagnostic>,
) {
    if node.kind() == "function_call" {
        let function_name = node
            .child_by_field_name("function")
            .and_then(|n| n.utf8_text(src).ok())
            .map(normalize_function_name);
        let args = node
            .children(&mut node.walk())
            .find(|n| n.kind() == "arguments")
            .map(argument_exprs)
            .unwrap_or_default();

        if let Some(function_name) = function_name
            && let Some(all_signatures) = signatures.get(&function_name)
        {
            let matching_arity = all_signatures
                .iter()
                .filter(|sig| sig.param_types.len() == args.len())
                .collect::<Vec<_>>();

            if !matching_arity.is_empty() {
                for (idx, arg_expr) in args.into_iter().enumerate() {
                    let expected = unify_expected_param_type(&matching_arity, idx);
                    let actual = infer_expr_type(arg_expr, src, bindings, function_returns);
                    if let (Some(expected), Some(actual)) = (expected, actual)
                        && expected != actual
                    {
                        out.push(Diagnostic {
                            range: node_to_range(arg_expr),
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("abl-semantic".into()),
                            message: format!(
                                "Function '{}' argument {} expects {}, got {}",
                                function_name,
                                idx + 1,
                                expected.label(),
                                actual.label()
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_function_call_arg_type_diags_in_node(
                ch,
                src,
                bindings,
                function_returns,
                signatures,
                out,
            );
        }
    }
}

fn unify_expected_param_type(
    signatures: &[&FunctionTypeSignature],
    index: usize,
) -> Option<BasicType> {
    let mut expected = None;
    for sig in signatures {
        let ty = sig.param_types.get(index).copied().flatten()?;
        match expected {
            None => expected = Some(ty),
            Some(prev) if prev == ty => {}
            Some(_) => return None,
        }
    }
    expected
}

fn function_param_types(function_node: Node<'_>, src: &[u8]) -> Vec<Option<BasicType>> {
    if let Some(parameters_node) = direct_child_by_kind(function_node, "parameters") {
        let mut header_param_types = Vec::new();
        collect_param_types_by_kind(parameters_node, src, "parameter", &mut header_param_types);
        if !header_param_types.is_empty() {
            return header_param_types;
        }
    }

    let mut out = Vec::new();
    collect_param_types_recursive(function_node, src, &mut out, true);
    out
}

fn collect_param_types_by_kind(
    node: Node<'_>,
    src: &[u8],
    target_kind: &str,
    out: &mut Vec<Option<BasicType>>,
) {
    if node.kind() == target_kind {
        out.push(
            node.child_by_field_name("type")
                .and_then(|n| n.utf8_text(src).ok())
                .and_then(parse_basic_type),
        );
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_param_types_by_kind(ch, src, target_kind, out);
        }
    }
}

fn collect_param_types_recursive(
    node: Node<'_>,
    src: &[u8],
    out: &mut Vec<Option<BasicType>>,
    is_root: bool,
) {
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

    if node.kind() == "parameter_definition" {
        out.push(
            node.child_by_field_name("type")
                .and_then(|n| n.utf8_text(src).ok())
                .and_then(parse_basic_type),
        );
        return;
    }

    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_param_types_recursive(ch, src, out, false);
        }
    }
}

fn argument_exprs(arguments_node: Node<'_>) -> Vec<Node<'_>> {
    let mut out = Vec::new();
    for i in 0..arguments_node.child_count() {
        let Some(ch) = arguments_node.child(i as u32) else {
            continue;
        };
        if ch.kind() != "argument" {
            continue;
        }
        if let Some(arg_expr) = ch.child_by_field_name("name").or_else(|| ch.named_child(0)) {
            out.push(arg_expr);
        }
    }
    out
}

fn parse_basic_type(raw: &str) -> Option<BasicType> {
    let upper = raw
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();

    match upper.as_str() {
        "CHARACTER" | "CHAR" | "LONGCHAR" | "CLOB" => Some(BasicType::Character),
        "INTEGER" | "INT" | "INT64" | "DECIMAL" | "DEC" | "NUMERIC" | "NUM" => {
            Some(BasicType::Numeric)
        }
        "LOGICAL" | "LOG" | "BOOLEAN" => Some(BasicType::Logical),
        "DATE" | "DATETIME" | "DATETIME-TZ" => Some(BasicType::DateLike),
        "HANDLE" | "COM-HANDLE" | "WIDGET-HANDLE" => Some(BasicType::Handle),
        _ => None,
    }
}

fn collect_ts_error_diags(node: Node, out: &mut Vec<Diagnostic>, limit: usize) {
    if out.len() >= limit {
        return;
    }

    if node.is_error() || node.is_missing() {
        out.push(Diagnostic {
            range: node_to_range(node),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("tree-sitter".into()),
            message: if node.is_missing() {
                "Missing token".into()
            } else {
                "Syntax error".into()
            },
            ..Default::default()
        });
        if out.len() >= limit {
            return;
        }
    }

    // DFS
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_ts_error_diags(ch, out, limit);
            if out.len() >= limit {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        collect_assignment_type_diags, collect_function_arities,
        collect_function_call_arg_type_diags, collect_function_calls, is_builtin_function_name,
    };
    use std::collections::HashMap;

    #[test]
    fn extracts_function_arities_and_call_arg_counts() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL (INPUT p1 AS CHARACTER, OUTPUT p2 AS INTEGER):
  RETURN TRUE.
END FUNCTION.

DEFINE VARIABLE x AS LOGICAL NO-UNDO.
x = foo("a", 1).
x = foo().
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut signatures = HashMap::<String, Vec<usize>>::new();
        collect_function_arities(tree.root_node(), src.as_bytes(), &mut signatures);
        assert_eq!(signatures.get("FOO").cloned(), Some(vec![2]));

        let mut calls = Vec::new();
        collect_function_calls(tree.root_node(), src.as_bytes(), &mut calls);
        let foo_calls = calls
            .into_iter()
            .filter(|c| c.name_upper == "FOO")
            .map(|c| c.arg_count)
            .collect::<Vec<_>>();
        assert_eq!(foo_calls, vec![2, 0]);
    }

    #[test]
    fn counts_nested_function_call_as_single_argument() {
        let src = r#"
FUNCTION foo RETURNS LOGICAL (INPUT p1 AS INTEGER):
  RETURN TRUE.
END FUNCTION.

DEFINE VARIABLE y AS LOGICAL NO-UNDO.
DEFINE VARIABLE pzd_linia AS CHARACTER NO-UNDO.
y = foo(INTEGER(pzd_linia)).
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut calls = Vec::new();
        collect_function_calls(tree.root_node(), src.as_bytes(), &mut calls);
        let foo_calls = calls
            .into_iter()
            .filter(|c| c.name_upper == "FOO")
            .map(|c| c.arg_count)
            .collect::<Vec<_>>();
        assert_eq!(foo_calls, vec![1]);
    }

    #[test]
    fn reports_assignment_type_mismatches_for_variables_and_function_returns() {
        let src = r#"
FUNCTION ret_int RETURNS INTEGER ():
  RETURN 1.
END FUNCTION.

DEFINE VARIABLE c AS CHARACTER NO-UNDO.
DEFINE VARIABLE i AS INTEGER NO-UNDO.
DEFINE VARIABLE okc AS CHARACTER NO-UNDO.

c = i.
i = c.
c = ret_int().
okc = "abc".
i = 42.
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut diags = Vec::new();
        collect_assignment_type_diags(tree.root_node(), src.as_bytes(), &mut diags);

        assert_eq!(diags.len(), 3);
        let messages = diags.into_iter().map(|d| d.message).collect::<Vec<_>>();
        assert!(
            messages
                .iter()
                .any(|m| m.contains("cannot assign NUMERIC to CHARACTER variable 'C'"))
        );
        assert!(
            messages
                .iter()
                .any(|m| m.contains("cannot assign CHARACTER to NUMERIC variable 'I'"))
        );
        assert!(
            messages
                .iter()
                .any(|m| m.contains("cannot assign NUMERIC to CHARACTER variable 'C'"))
        );
    }

    #[test]
    fn reports_function_argument_type_mismatches() {
        let src = r#"
FUNCTION local_mul RETURNS INTEGER (INPUT a AS INTEGER, INPUT b AS INTEGER):
  RETURN a * b.
END FUNCTION.

local_mul("5", 1).
"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        let tree = parser.parse(src, None).expect("parse source");

        let mut diags = Vec::new();
        collect_function_call_arg_type_diags(tree.root_node(), src.as_bytes(), &mut diags);

        assert_eq!(diags.len(), 1);
        assert!(
            diags[0]
                .message
                .contains("Function 'LOCAL_MUL' argument 1 expects NUMERIC, got CHARACTER")
        );
    }

    #[test]
    fn treats_abl_builtins_as_known_functions_for_unknown_checks() {
        assert!(is_builtin_function_name("ABSOLUTE"));
        assert!(is_builtin_function_name("DYNAMIC-FUNCTION"));
        assert!(is_builtin_function_name("ADD-INTERVAL"));
        assert!(is_builtin_function_name("TENANT-NAME-TO-ID"));
        assert!(is_builtin_function_name("WIDGET-HANDLE"));
        assert!(is_builtin_function_name("CDC_GET_CHANGED_COLUMNS"));
        assert!(is_builtin_function_name("ADD_MONTHS"));
        assert!(is_builtin_function_name("TO_TIMESTAMP"));
        assert!(is_builtin_function_name("PRO_ARR_ESCAPE"));
        assert!(is_builtin_function_name("SUBSTRING"));
    }
}
