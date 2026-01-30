use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::backend::Backend;

pub async fn on_change(backend: &Backend, uri: Url, text: String) {
    backend.docs.insert(uri.clone(), text.to_owned());

    let mut parser = backend.parser.lock().await;
    let tree = match parser.parse(text, None) {
        Some(t) => t,
        None => {
            backend
                .client
                .publish_diagnostics(uri.clone(), vec![], None)
                .await;
            return;
        }
    };

    let mut diags: Vec<Diagnostic> = Vec::new();
    collect_ts_error_diags(tree.root_node(), &mut diags);
    backend
        .client
        .publish_diagnostics(uri.clone(), diags, None)
        .await;

    backend.trees.insert(uri, tree);
}

fn collect_ts_error_diags(node: Node, out: &mut Vec<Diagnostic>) {
    if node.is_error() || node.is_missing() {
        let sp = node.start_position();
        let ep = node.end_position();

        out.push(Diagnostic {
            range: Range::new(
                Position::new(sp.row as u32, sp.column as u32),
                Position::new(ep.row as u32, ep.column as u32),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("tree-sitter".into()),
            message: if node.is_missing() {
                "Missing token".into()
            } else {
                "Syntax error".into()
            },
            ..Default::default()
        });
    }

    // DFS
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_ts_error_diags(ch, out);
        }
    }
}
