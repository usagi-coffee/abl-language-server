pub mod buffers;
pub mod builtins;
pub mod completion;
pub mod completion_support;
pub mod definition;
pub mod definitions;
pub mod df;
pub mod diagnostics;
pub mod formatting;
pub mod functions;
pub mod hover;
pub mod includes;
pub mod local_tables;
pub mod schema;
pub mod schema_lookup;
pub mod scopes;
pub mod semantic_tokens;
pub mod signature;
pub mod types;

#[cfg(test)]
pub(crate) fn parse_abl(src: &str) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_abl::LANGUAGE.into())
        .expect("set abl language");
    parser.parse(src, None).expect("parse source")
}
