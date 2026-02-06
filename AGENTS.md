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
- `example.p`: handy local sample file for manual parser/completion checks.
- `Cargo.toml`: dependencies and feature flags.
- `tree-sitter-abl`: dev symlink to a `tree-sitter-abl` repository.

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

- `tree-sitter-abl` is a git dependency, so parser changes are not picked up here until dependency update/lockfile refresh.
- You can access source code of `tree-sitter-abl` through a symlink that is present in the repository at `./tree-sitter-abl` to look up syntax details.
- Use parser repo commands when grammar/syntax behavior is involved:
  - `cd /home/jk/tree-sitter-abl && bun run test`
  - `cd /home/jk/tree-sitter-abl && bun run parse example.p`
  - `cd /home/jk/tree-sitter-abl && bun run parse:snippet '<snippet>'`
  - `cd /home/jk/tree-sitter-abl && bun run reference '<query>'`
- Never run `tree-sitter` CLI directly in parser workflows; prefer the `bun run ...` commands defined by the parser repo.
- If any change is needed in parser / would be help out please tell, we will implement it in parser by other agent.

## Conventions

- Keep parser usage centralized through `Backend.parser` (`tokio::sync::Mutex<Parser>`); avoid creating ad-hoc parsers in handlers.
- Keep document and syntax-tree state in `Backend.docs` and `Backend.trees` keyed by URI.
- Diagnostics and completion should both operate on the latest parsed tree produced in `on_change`.
- Preserve existing sync mode assumptions:
  - server advertises `TextDocumentSyncKind::FULL`
  - `did_change` currently consumes `content_changes[0].text` as full text
- Maintain deterministic completion output:
  - collect symbols
  - `sort`
  - `dedup`
  - filter by typed prefix (case-insensitive)
- Prefer small, focused handler logic and move reusable analysis into `src/analysis/*`.

## Change expectations

- Any new user-visible LSP feature should include:
  - capability wiring in `src/backend.rs`
  - handler implementation in `src/handlers/*`
  - state/analysis updates if needed
  - validation via `cargo test` (or at least `cargo check` when tests are absent)
- Do not silently degrade diagnostics/completion behavior when adding new features; keep existing flows intact unless change is intentional and documented.
- Keep logging useful but restrained (`debug!` for development flow, avoid noisy logs in hot paths unless needed for diagnosis).

## Notes

- This repository may not always include extensive tests; when adding behavior-heavy logic, add tests where practical or document manual verification steps.
