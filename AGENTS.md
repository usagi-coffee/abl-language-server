# AGENTS.md

## Project

- Rust Language Server Protocol (LSP) server for OpenEdge ABL.
- Uses `tower-lsp` for protocol plumbing and `tree-sitter-abl` for parsing.

## Key paths

- `src/main.rs`: process entrypoint, parser initialization, and LSP service bootstrap.
- `src/backend.rs`: `LanguageServer` implementation and advertised capabilities.
- `src/handlers/sync.rs`: `didOpen`/`didChange`/`didSave`/`didClose` handlers.
- `src/handlers/diagnostics.rs`: parse-on-change and publish diagnostics from tree-sitter errors.
- `src/handlers/completion.rs`: variable completion flow and prefix filtering.
- `src/analysis/variables.rs`: syntax-tree walk for variable declaration extraction.
- `src/utils/position.rs`: LSP position/offset conversion and identifier prefix helpers.
- `tree-sitter-abl`: dev symlink to `tree-sitter-abl` repository.
- `tree-sitter-df`: dev symlink to `tree-sitter-df` repository.
- `playground/*`: manual testing suite.

## Workflow

- Format code:
  - `cargo fmt`
- Lint (if available locally):
  - `cargo clippy --all-targets --all-features -- -D warnings`
- Run tests:
  - `cargo test`
- Type-check/build:
  - `cargo check`
- Build server:
  - `cargo build`
- Run the server (stdio transport):
  - `cargo run`

If `cargo` commands fail with crate download errors, the environment has no network access to `crates.io`; rerun in a network-enabled environment.

## Parser repo integration

- `tree-sitter-abl` and `tree-sitter-df` are git dependencies, so parser changes are not picked up here until dependency update/lockfile refresh.
- You can access source code of parsers through a symlink that is present in the repository at `./tree-sitter-abl` and `./tree-sitter-df` to look up syntax details.
- Use parser repo commands when grammar/syntax behavior is involved:
  - `cd /home/jk/tree-sitter-abl && bun run test`
  - `cd /home/jk/tree-sitter-abl && bun run parse example.p`
  - `cd /home/jk/tree-sitter-abl && bun run parse:snippet '<snippet>'`
  - `cd /home/jk/tree-sitter-abl && bun run reference '<query>'`
- If any change is needed in parser / would be help out please tell, we will implement it in parser by other agent.

## Conventions

- Keep parser usage centralized through `Backend.parser` (`tokio::sync::Mutex<Parser>`); avoid creating ad-hoc parsers in handlers.
- Keep document and syntax-tree state in `Backend.docs` and `Backend.trees` keyed by URI.
- Preserve existing sync mode assumptions:
  - server advertises `TextDocumentSyncKind::FULL`
  - `did_change` currently consumes `content_changes[0].text` as full text
- Prefer small, focused handler logic and move reusable analysis into `src/analysis/*` and `src/utils/*`.

## Change expectations

- Any new user-visible LSP feature should include:
  - capability wiring in `src/backend.rs`
  - handler implementation in `src/handlers/*`
  - state/analysis updates if needed
  - validation via `cargo test` (or at least `cargo check` when tests are absent)
  - manual example in `playground/`
- Do not silently degrade diagnostics/completion behavior when adding new features; keep existing flows intact unless change is intentional and documented.
- Keep logging useful but restrained (`debug!` for development flow, avoid noisy logs in hot paths unless needed for diagnosis).

## Notes

- This repository may not always include extensive tests; when adding behavior-heavy logic, add tests where practical or document manual verification steps.
