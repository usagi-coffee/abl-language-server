use std::collections::HashMap;

use dashmap::{DashMap, DashSet};
use tokio::sync::Mutex;
use tower_lsp::{LspService, Server};

mod analysis;
mod backend;
mod config;
mod handlers;
mod utils;

use backend::Backend;
use config::AblConfig;

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

    let mut df_parser = tree_sitter::Parser::new();
    let df_language = tree_sitter_df::LANGUAGE;
    df_parser
        .set_language(&df_language.into())
        .expect("Error loading df parser");

    let (service, socket) = LspService::build(|client| Backend {
        client,
        docs: DashMap::new(),
        trees: DashMap::new(),
        parser: Mutex::new(parser),
        df_parser: Mutex::new(df_parser),
        workspace_root: Mutex::new(None),
        config: Mutex::new(AblConfig::default()),
        db_tables: DashSet::new(),
        db_table_definitions: Mutex::new(HashMap::new()),
        db_field_definitions: Mutex::new(HashMap::new()),
        db_index_definitions: Mutex::new(HashMap::new()),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
