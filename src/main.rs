use dashmap::DashMap;
use log::debug;
use serde_json::Value;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Node, Parser, Tree};

struct Backend {
    pub client: Client,
    pub parser: Mutex<Parser>,
    pub trees: DashMap<Url, Tree>,
    pub docs: DashMap<Url, String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,

            capabilities: ServerCapabilities {
                document_formatting_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: None,
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                execute_command_provider: None,
                workspace: None,
                semantic_tokens_provider: None,
                definition_provider: None,
                references_provider: None,
                rename_provider: None,
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        debug!("initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(TextDocumentChange {
            uri: params.text_document.uri,
            text: params.text_document.text,
        })
        .await;
        debug!("file opened!");
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.on_change(TextDocumentChange {
            uri: params.text_document.uri,
            text: params.content_changes[0].text.clone(),
        })
        .await;
        debug!("changed!");
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        debug!("file saved!");
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        debug!("file closed!");
    }

    async fn goto_definition(
        &self,
        _params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }

    async fn references(&self, _params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        Ok(None)
    }

    async fn semantic_tokens_full(
        &self,
        _params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        Ok(None)
    }

    async fn semantic_tokens_range(
        &self,
        _params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        Ok(None)
    }

    async fn inlay_hint(
        &self,
        _params: tower_lsp::lsp_types::InlayHintParams,
    ) -> Result<Option<Vec<InlayHint>>> {
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        debug!("{:?}", params);
        self.get_completion(params).await
    }

    async fn rename(&self, _params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Ok(None)
    }

    async fn formatting(&self, _params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        Ok(None)
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        debug!("configuration changed!");
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        debug!("workspace folders changed!");
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        debug!("watched files have changed!");
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        debug!("command executed!");

        Ok(None)
    }
}

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

struct TextDocumentChange {
    uri: Url,
    text: String,
}

impl Backend {
    pub async fn on_change(&self, change: TextDocumentChange) {
        self.docs.insert(change.uri.clone(), change.text.to_owned());

        let mut parser = self.parser.lock().await;
        let tree = match parser.parse(change.text, None) {
            Some(t) => t,
            None => {
                self.client
                    .publish_diagnostics(change.uri.clone(), vec![], None)
                    .await;
                return;
            }
        };

        let mut diags: Vec<Diagnostic> = Vec::new();
        self.collect_ts_error_diags(tree.root_node(), &mut diags);
        self.client
            .publish_diagnostics(change.uri.clone(), diags, None)
            .await;

        self.trees.insert(change.uri, tree);
    }

    fn collect_ts_error_diags(&self, node: Node, out: &mut Vec<Diagnostic>) {
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
                self.collect_ts_error_diags(ch, out);
            }
        }
    }

    pub async fn get_completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let text = match self.docs.get(&uri) {
            Some(t) => t,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };
        let tree = match self.trees.get(&uri) {
            Some(t) => t,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };

        let offset = match lsp_pos_to_utf8_byte_offset(&text, pos) {
            Some(o) => o,
            None => return Ok(Some(CompletionResponse::Array(vec![]))),
        };

        let prefix = ascii_ident_prefix(&text, offset);

        let mut vars = Vec::<String>::new();
        collect_variable_decls(tree.root_node(), text.as_bytes(), &mut vars);
        debug!("{:?}", vars);

        vars.sort();
        vars.dedup();

        let pref_up = prefix.to_ascii_uppercase();
        let items = vars
            .into_iter()
            .filter(|v| v.to_ascii_uppercase().starts_with(&pref_up))
            .map(|v| CompletionItem {
                label: v.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some("CHARACTER".to_string()),
                insert_text: Some(v),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Ok(Some(CompletionResponse::Array(items)))
    }
}

// Assumes Position.character is a UTF-8 byte column within that line.
fn lsp_pos_to_utf8_byte_offset(text: &str, pos: Position) -> Option<usize> {
    let line = pos.line as usize;
    let col = pos.character as usize;

    let mut cur_line = 0usize;
    let mut line_start = 0usize;

    for (i, b) in text.bytes().enumerate() {
        if cur_line == line {
            line_start = i;
            break;
        }
        if b == b'\n' {
            cur_line += 1;
        }
    }

    if line == 0 {
        line_start = 0;
    } else if cur_line != line {
        return None;
    }

    // find line end
    let line_end = text[line_start..]
        .find('\n')
        .map(|d| line_start + d)
        .unwrap_or(text.len());

    let target = line_start.saturating_add(col);
    if target > line_end {
        Some(line_end)
    } else {
        Some(target)
    }
}

// Walk backward from offset and capture [A-Za-z0-9_]* as prefix.
fn ascii_ident_prefix(text: &str, mut offset: usize) -> String {
    let bytes = text.as_bytes();
    if offset > bytes.len() {
        offset = bytes.len();
    }
    let mut start = offset;
    while start > 0 {
        let c = bytes[start - 1];
        let is_ident = c.is_ascii_alphanumeric() || c == b'_';
        if !is_ident {
            break;
        }
        start -= 1;
    }
    text[start..offset].to_string()
}

/// Walk the syntax tree and extract declared variable names.
///
/// You MUST adapt the patterns here to your grammar:
/// - choose the node kinds that represent variable declarations
/// - pick where the declared identifier sits (field "name", or a child "identifier", etc.)
fn collect_variable_decls(node: Node, src: &[u8], out: &mut Vec<String>) {
    // Pattern A (preferred): declaration node has a "name" field containing identifier
    // Example kinds you might have: "variable_declaration", "var_decl", "define_variable"
    debug!("{}", node.kind());
    if matches!(node.kind(), "variable_definition") {
        if let Some(name) = node.child_by_field_name("name") {
            if name.kind() == "identifier"
                && let Ok(s) = name.utf8_text(src)
            {
                out.push(s.to_string());
            }
        } else {
            // Pattern B: declaration contains an identifier somewhere inside (fallback)
            find_first_identifier(node, src, out);
        }
    }

    // DFS
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            collect_variable_decls(ch, src, out);
        }
    }
}

fn find_first_identifier(node: Node, src: &[u8], out: &mut Vec<String>) {
    if node.kind() == "identifier" {
        if let Ok(s) = node.utf8_text(src) {
            out.push(s.to_string());
        }
        return;
    }
    for i in 0..node.child_count() {
        if let Some(ch) = node.child(i as u32) {
            find_first_identifier(ch, src, out);
        }
    }
}
