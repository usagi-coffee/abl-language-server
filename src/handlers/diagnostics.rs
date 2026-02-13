use std::collections::HashSet;

use tower_lsp::lsp_types::*;

use crate::analysis::diagnostics::config::diagnostics_feature_enabled_for_uri;
use crate::analysis::diagnostics::semantic::{
    collect_function_call_arity_diags, collect_unknown_symbol_diags, is_latest_version,
    should_accept_version,
};
use crate::analysis::diagnostics::syntax::collect_ts_error_diags;
use crate::analysis::diagnostics::types::{
    collect_assignment_type_diags, collect_function_call_arg_type_diags,
};
use crate::backend::Backend;

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

    backend.set_document_text_version(&uri, version, text.to_owned(), false);

    if !is_latest_version(backend, &uri, version) {
        return;
    }

    let diagnostics_enabled = backend.config.lock().await.diagnostics.enabled;
    let diagnostics_cfg = backend.config.lock().await.diagnostics.clone();
    let workspace_root = backend.workspace_root.lock().await.clone();
    let unknown_variables_enabled = diagnostics_feature_enabled_for_uri(
        &uri,
        workspace_root.as_deref(),
        &diagnostics_cfg.unknown_variables,
    );
    let unknown_functions_enabled = diagnostics_feature_enabled_for_uri(
        &uri,
        workspace_root.as_deref(),
        &diagnostics_cfg.unknown_functions,
    );
    let unknown_variables_ignored: HashSet<String> = diagnostics_cfg
        .unknown_variables
        .ignore
        .iter()
        .map(|name| name.to_ascii_uppercase())
        .collect();
    let unknown_functions_ignored: HashSet<String> = diagnostics_cfg
        .unknown_functions
        .ignore
        .iter()
        .map(|name| name.to_ascii_uppercase())
        .collect();
    let parsed_tree = {
        let Some(doc) = backend.documents.get_mut(&uri) else {
            return;
        };
        let mut parser = doc.parser.lock().expect("ABL parser mutex poisoned");
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
        backend.set_document_tree_if_version(&uri, version, tree);
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
        unknown_variables_enabled,
        unknown_functions_enabled,
        &unknown_variables_ignored,
        &unknown_functions_ignored,
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
    backend.set_document_tree_if_version(&uri, version, tree);
}
