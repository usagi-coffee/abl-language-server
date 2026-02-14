use dashmap::{DashMap, DashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::{LspService, Server};

mod analysis;
mod backend;
mod config;
mod handlers;
mod utils;

use backend::Backend;
use backend::BackendState;
use config::AblConfig;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let abl_language = tree_sitter_abl::LANGUAGE.into();

    let mut df_parser = tree_sitter::Parser::new();
    let df_language = tree_sitter_df::LANGUAGE;
    df_parser
        .set_language(&df_language.into())
        .expect("Error loading df parser");

    let (service, socket) = LspService::build(|client| Backend {
        client,
        state: Arc::new(BackendState {
            abl_language,
            df_parser: Mutex::new(df_parser),
            documents: DashMap::new(),
            workspace_root: Mutex::new(None),
            config: Mutex::new(AblConfig::default()),
            db_tables: DashSet::new(),
            db_table_labels: DashMap::new(),
            db_table_definitions: DashMap::new(),
            db_field_definitions: DashMap::new(),
            db_index_definitions: DashMap::new(),
            db_indexes_by_table: DashMap::new(),
            db_index_fields_by_table_index: DashMap::new(),
            db_fields_by_table: DashMap::new(),
            include_completion_cache: DashMap::new(),
            include_parse_cache: DashMap::new(),
        }),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
