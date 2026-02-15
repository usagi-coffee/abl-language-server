# AGENTS.md

## Project

- Rust Language Server Protocol (LSP) server for OpenEdge ABL.
- Uses `tower-lsp` for protocol plumbing and `tree-sitter-abl` for parsing.

## Key paths

- `src/main.rs`: process entrypoint, parser initialization, and LSP service bootstrap.
- `src/backend.rs`: `LanguageServer` implementation and advertised capabilities.
- `src/handlers/*`: LSP request/notification entrypoints. Keep only protocol orchestration, backend calls, and response shaping; avoid heavy parsing/analysis logic here.
- `src/analysis/*`: Reusable, mostly pure language-analysis logic over text/tree/schema (collectors, resolvers, symbol/type/definition helpers). Shared across handlers.
- `src/utils/*`: Low-level generic utilities (position/offset math, path helpers, text-sync helpers, tree-sitter traversal helpers) that are not domain-feature specific.
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
- Type-check/Linting:
  - `cargo check`
  - `cargo clippy --fix`
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

- Prefer small, focused handler logic and move reusable analysis into `src/analysis/*` and `src/utils/*`.
- Do not silently degrade diagnostics/completion behavior when adding new features; keep existing flows intact unless change is intentional and documented.

## Required verification sequence (after every code change)

Run in this exact order:
1. `cargo test`
2. `cargo clippy --fix --allow-dirty`
3. `cargo fmt`
4. `cargo build -r` (manual changes)

Notes:
- Do not skip `clippy --fix` unless it fails due to an external/tooling issue.
- If any step fails, report the failure and stop before further edits. Keep logging useful but restrained (`debug!` for development flow, avoid noisy logs in hot paths unless needed for diagnosis).

## Notes

- This repository may not always include extensive tests; when adding behavior-heavy logic, add tests where practical or document manual verification steps.
