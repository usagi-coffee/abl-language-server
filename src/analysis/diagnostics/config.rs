use std::path::Path;

use tower_lsp::lsp_types::Url;

use crate::config::DiagnosticFeatureConfig;
use crate::utils::paths::uri_matches_any_path_pattern;

pub fn diagnostics_feature_enabled_for_uri(
    uri: &Url,
    workspace_root: Option<&Path>,
    feature: &DiagnosticFeatureConfig,
) -> bool {
    if !feature.enabled {
        return false;
    }
    if feature.exclude.is_empty() {
        return true;
    }
    !uri_matches_any_path_pattern(uri, workspace_root, &feature.exclude)
}
