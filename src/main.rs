use dashmap::DashMap;
use tokio::sync::Mutex;
use tower_lsp::{LspService, Server};

mod analysis;
mod backend;
mod handlers;
mod utils;

use backend::Backend;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_abl::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading abl parser");

    let (service, socket) = LspService::build(|client| Backend {
        client,
        docs: DashMap::new(),
        trees: DashMap::new(),
        parser: Mutex::new(parser),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
