use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tower_lsp::lsp_types::{Diagnostic, Url};
use tree_sitter::Node;

use crate::analysis::definitions::{
    PreprocessorDefineSite, collect_global_preprocessor_define_sites,
    collect_preprocessor_define_sites,
};
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
use crate::analysis::includes::{collect_include_sites, resolve_include_site_path};
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
        let include_parses =
            collect_resolved_include_parses(backend, &current_path, text, root).await;
        for (_, include_text, include_tree) in include_parses {
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
    params: UnknownSymbolDiagParams<'_>,
    out: &mut Vec<Diagnostic>,
) -> bool {
    if !params.include_semantic_diags {
        return true;
    }

    if !is_latest_version(backend, params.uri, params.version) {
        return false;
    }

    let mut known_variables = HashSet::<String>::new();
    let mut known_functions = HashSet::<String>::new();
    let mut known_function_signatures = HashMap::<String, Vec<usize>>::new();
    collect_known_symbols(
        params.root,
        params.text.as_bytes(),
        &mut known_variables,
        &mut known_functions,
    );
    collect_function_arities(
        params.root,
        params.text.as_bytes(),
        &mut known_function_signatures,
    );
    collect_local_table_field_symbols(
        backend,
        params.root,
        params.text.as_bytes(),
        &mut known_variables,
    );

    if params.include_semantic_diags
        && let Ok(current_path) = params.uri.to_file_path()
    {
        let include_parses =
            collect_resolved_include_parses(backend, &current_path, params.text, params.root).await;
        for (_, include_text, include_tree) in include_parses {
            if !is_latest_version(backend, params.uri, params.version) {
                return false;
            }
            collect_known_symbols(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut known_variables,
                &mut known_functions,
            );
            collect_function_arities(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut known_function_signatures,
            );
        }
    }

    known_functions.extend(known_function_signatures.into_keys());

    let mut refs = Vec::<IdentifierRef>::new();
    collect_identifier_refs_for_unknown_symbol_diag(params.root, params.text.as_bytes(), &mut refs);
    normalize_identifier_refs(&mut refs);
    let active_buffer_like_names =
        collect_active_buffer_like_names(params.root, params.text.as_bytes(), backend);
    let active_table_fields =
        collect_active_db_table_field_symbols(backend, &active_buffer_like_names);

    let mut calls = Vec::<FunctionCallSite>::new();
    collect_function_calls(params.root, params.text.as_bytes(), &mut calls);
    append_unknown_symbol_diags(
        UnknownSymbolDiagInputs {
            refs: &refs,
            calls: &calls,
            known_variables: &known_variables,
            known_functions: &known_functions,
            unknown_variables_ignored: params.unknown_variables_ignored,
            unknown_functions_ignored: params.unknown_functions_ignored,
            db_tables: &backend.db_tables,
            active_table_fields: &active_table_fields,
            active_buffer_like_names: &active_buffer_like_names,
            unknown_variables_enabled: params.unknown_variables_enabled,
            unknown_functions_enabled: params.unknown_functions_enabled,
        },
        out,
    );

    true
}

pub struct UnknownSymbolDiagParams<'a> {
    pub uri: &'a Url,
    pub version: i32,
    pub text: &'a str,
    pub root: Node<'a>,
    pub include_semantic_diags: bool,
    pub unknown_variables_enabled: bool,
    pub unknown_functions_enabled: bool,
    pub unknown_variables_ignored: &'a HashSet<String>,
    pub unknown_functions_ignored: &'a HashSet<String>,
}

async fn collect_resolved_include_parses(
    backend: &Backend,
    current_path: &Path,
    text: &str,
    root: Node<'_>,
) -> Vec<(PathBuf, Arc<String>, tree_sitter::Tree)> {
    let mut state = IncludeCollectState {
        seen: HashSet::new(),
        out: Vec::new(),
        pending: Vec::new(),
    };

    collect_resolved_includes_for_file(backend, current_path, text, root, &[], &mut state).await;

    while let Some(next) = state.pending.pop() {
        collect_resolved_includes_for_file(
            backend,
            &next.path,
            next.text.as_str(),
            next.tree.root_node(),
            &next.inherited_globals,
            &mut state,
        )
        .await;
    }

    state.out
}

async fn collect_resolved_includes_for_file(
    backend: &Backend,
    file_path: &Path,
    file_text: &str,
    file_root: Node<'_>,
    inherited_globals: &[PreprocessorDefineSite],
    state: &mut IncludeCollectState,
) {
    let include_sites = collect_include_sites(file_text);
    let mut available_define_sites = inherited_globals.to_vec();
    collect_preprocessor_define_sites(file_root, file_text.as_bytes(), &mut available_define_sites);

    for include in include_sites {
        let include_path_value = resolve_include_site_path(&include, &available_define_sites);
        let Some(resolved_path) = backend
            .resolve_include_path_for(file_path, &include_path_value)
            .await
        else {
            continue;
        };

        if let Some((include_text, include_tree)) =
            backend.get_cached_include_parse(&resolved_path).await
        {
            let mut include_global_defines = Vec::new();
            collect_global_preprocessor_define_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut include_global_defines,
            );
            for mut define in include_global_defines {
                define.start_byte = include.start_offset;
                available_define_sites.push(define);
            }

            if state.seen.insert(resolved_path.clone()) {
                state.out.push((
                    resolved_path.clone(),
                    include_text.clone(),
                    include_tree.clone(),
                ));
                state.pending.push(PendingInclude {
                    path: resolved_path,
                    text: include_text,
                    tree: include_tree,
                    inherited_globals: globals_visible_at_offset(
                        &available_define_sites,
                        include.start_offset,
                    ),
                });
            }
        }
    }
}

#[derive(Clone)]
struct PendingInclude {
    path: PathBuf,
    text: Arc<String>,
    tree: tree_sitter::Tree,
    inherited_globals: Vec<PreprocessorDefineSite>,
}

struct IncludeCollectState {
    seen: HashSet<PathBuf>,
    out: Vec<(PathBuf, Arc<String>, tree_sitter::Tree)>,
    pending: Vec<PendingInclude>,
}

fn globals_visible_at_offset(
    available_define_sites: &[PreprocessorDefineSite],
    offset: usize,
) -> Vec<PreprocessorDefineSite> {
    available_define_sites
        .iter()
        .filter(|d| d.is_global && d.start_byte <= offset)
        .cloned()
        .map(|mut d| {
            d.start_byte = 0;
            d
        })
        .collect()
}
