use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tower_lsp::lsp_types::{Diagnostic, Url};
use tree_sitter::Node;

use crate::analysis::diagnostics::functions::{
    FunctionCallSite, append_function_arity_mismatch_diags, collect_function_arities,
    collect_function_calls,
};
use crate::analysis::diagnostics::symbols::{
    IdentifierRef, UnknownSymbolDiagInputs, append_unknown_symbol_diags,
    collect_active_buffer_like_names, collect_active_db_table_field_symbols,
    collect_identifier_refs_for_unknown_symbol_diag, collect_known_symbols,
    collect_local_table_field_symbols, normalize_identifier_refs,
};
use crate::analysis::includes::collect_include_sites;
use crate::backend::Backend;

pub fn should_accept_version(backend: &Backend, uri: &Url, version: i32) -> bool {
    match backend.documents.get(uri) {
        Some(current) => current.version <= version,
        None => true,
    }
}

pub fn is_latest_version(backend: &Backend, uri: &Url, version: i32) -> bool {
    matches!(backend.documents.get(uri), Some(current) if current.version == version)
}

pub async fn collect_function_call_arity_diags(
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

    if include_from_includes && let Ok(current_path) = uri.to_file_path() {
        let include_sites = collect_include_sites(text);
        let mut seen = HashSet::<PathBuf>::new();
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

            let Some((include_text, include_tree)) = backend.get_cached_include_parse(&path).await
            else {
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
    append_function_arity_mismatch_diags(&signatures, &calls, out);

    true
}

pub async fn collect_unknown_symbol_diags(
    backend: &Backend,
    uri: &Url,
    version: i32,
    text: &str,
    root: Node<'_>,
    include_semantic_diags: bool,
    unknown_variables_enabled: bool,
    unknown_functions_enabled: bool,
    unknown_variables_ignored: &HashSet<String>,
    unknown_functions_ignored: &HashSet<String>,
    out: &mut Vec<Diagnostic>,
) -> bool {
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
            let Some((include_text, include_tree)) = backend.get_cached_include_parse(&path).await
            else {
                continue;
            };
            if !is_latest_version(backend, uri, version) {
                return false;
            }
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
    normalize_identifier_refs(&mut refs);
    let active_buffer_like_names = collect_active_buffer_like_names(root, text.as_bytes(), backend);
    let active_table_fields =
        collect_active_db_table_field_symbols(backend, &active_buffer_like_names);

    let mut calls = Vec::<FunctionCallSite>::new();
    collect_function_calls(root, text.as_bytes(), &mut calls);
    append_unknown_symbol_diags(
        UnknownSymbolDiagInputs {
            refs: &refs,
            calls: &calls,
            known_variables: &known_variables,
            known_functions: &known_functions,
            unknown_variables_ignored,
            unknown_functions_ignored,
            db_tables: &backend.db_tables,
            active_table_fields: &active_table_fields,
            active_buffer_like_names: &active_buffer_like_names,
            unknown_variables_enabled,
            unknown_functions_enabled,
        },
        out,
    );

    true
}
