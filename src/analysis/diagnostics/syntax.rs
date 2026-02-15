use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};
use tree_sitter::Node;

use crate::utils::ts::node_to_range;

pub fn collect_ts_error_diags(node: Node<'_>, out: &mut Vec<Diagnostic>, limit: usize) {
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
    use super::collect_ts_error_diags;
    use crate::analysis::parse_abl;

    #[test]
    fn collects_syntax_errors_with_limit() {
        let src = r#"
FUNCTION bad RETURNS LOGICAL (:
  RETURN TRUE
END FUNCTION
"#;
        let tree = parse_abl(src);

        let mut out = Vec::new();
        collect_ts_error_diags(tree.root_node(), &mut out, 1);
        assert_eq!(out.len(), 1);
        assert!(out[0].message == "Syntax error" || out[0].message == "Missing token");
    }
}
