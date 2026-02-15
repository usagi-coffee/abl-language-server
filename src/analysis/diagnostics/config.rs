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

#[cfg(test)]
mod tests {
    use super::diagnostics_feature_enabled_for_uri;
    use crate::config::DiagnosticFeatureConfig;
    use std::path::Path;
    use tower_lsp::lsp_types::Url;

    #[test]
    fn disables_feature_when_flag_is_false() {
        let uri = Url::parse("file:///tmp/project/src/main.p").expect("uri");
        let feature = DiagnosticFeatureConfig {
            enabled: false,
            exclude: Vec::new(),
            ignore: Vec::new(),
        };

        assert!(!diagnostics_feature_enabled_for_uri(
            &uri,
            Some(Path::new("/tmp/project")),
            &feature
        ));
    }

    #[test]
    fn enables_feature_without_exclusions() {
        let uri = Url::parse("file:///tmp/project/src/main.p").expect("uri");
        let feature = DiagnosticFeatureConfig {
            enabled: true,
            exclude: Vec::new(),
            ignore: Vec::new(),
        };

        assert!(diagnostics_feature_enabled_for_uri(
            &uri,
            Some(Path::new("/tmp/project")),
            &feature
        ));
    }

    #[test]
    fn applies_exclusion_patterns() {
        let uri = Url::parse("file:///tmp/project/legacy/old.p").expect("uri");
        let feature = DiagnosticFeatureConfig {
            enabled: true,
            exclude: vec!["legacy/*".to_string()],
            ignore: Vec::new(),
        };

        assert!(!diagnostics_feature_enabled_for_uri(
            &uri,
            Some(Path::new("/tmp/project")),
            &feature
        ));
    }
}
