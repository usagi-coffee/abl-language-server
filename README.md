# abl-language-server

Language Server Protocol (LSP) implementation for ABL (OpenEdge Advanced Business Language), supports parser-based language features for ABL and optional DB schema integration via `.df` dump files.

The language server supports optional document formatting (auto-indent only). Formatting is disabled by default.

## Extensions

[zed-openedge-abl](https://github.com/usagi-coffee/zed-openedge-abl)

[vscode-openedge-abl](https://github.com/usagi-coffee/vscode-openedge-abl)

## Features

| Feature                               | Notes                                                                                                         |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| Text sync                             | `TextDocumentSyncKind::FULL`                                                                                  |
| Parser diagnostics                    | Tree-sitter syntax errors (`is_error` / `is_missing`)                                                         |
| Semantic diagnostics: function arity  | Checks `function_call` argument count against known function definitions (current file + included `.i` files) |
| Completion: local symbols             | Variables/definitions with case-insensitive prefix filtering                                                  |
| Completion: DB tables                 | Uses configured `.df` dump files                                                                              |
| Completion: DB fields after `table.`  | Supports table names and buffer aliases (`DEFINE BUFFER ... FOR ...`)                                         |
| Completion item details/docs          | Field type in `detail`; `LABEL` / `FORMAT` / `DESCRIPTION` in docs when available                             |
| Go to Definition: local               | Local definitions                                                                                             |
| Go to Definition: includes            | Scoped include-aware function definitions                                                                     |
| Go to Definition: DB schema           | Tables, fields, indexes from `.df`; buffer alias -> table definition                                          |
| Find References: DB table definitions | Returns matching `ADD TABLE` locations from `.df`                                                             |
| Hover: local symbols                  | Type/detail hover                                                                                             |
| Hover: functions                      | Signature with parameters + return type, include-aware                                                        |
| Hover: DB schema                      | Table / field / index; field metadata includes type/label/format/description                                  |
| Semantic tokens                       | Highlights DB table identifiers (`token type: type`)                                                          |
| Formatting (auto-indent)              | Parser-aware indentation only; guarded by AST-shape check and optional idempotence check                      |

## Configuration (`abl.toml`)

The server searches for `abl.toml` in the opened workspace root.

### Supported options

```toml
# Optional
# Inherit one or more other config files (relative to this file or absolute path).
inherits = ["shared/abl.base.toml"]

# Optional (by defualt searches relative to the opened root directory)
# Supports absolute paths and workspace-root-relative paths, order is preserved.
propath = ["/global/a", "includes", "shared/includes"]

# Optional
# Databse schemas: so we can pull types/go to definition will go to the entry
dumpfile = ["schema/core.df", "schema/custom.df"]

[completion]
enabled = true

[diagnostics]
enabled = true

[diagnostics.unknown_variables]
enabled = true
exclude = ["legacy/*.p", "generated/procedures"]
ignore = ["batchrun", "today", "now"]

[diagnostics.unknown_functions]
enabled = true
exclude = "generated/sql/*"
ignore = ["abs", "round", "my_dynamic_fn"]

[semantic_tokens]
enabled = true

[formatting]
enabled = false
indent_size = 2
use_tabs = false
idempotence = true
```

### Option reference

| Key                       | Type                 | Default | Description                                                                           |
| ------------------------- | -------------------- | ------- | ------------------------------------------------------------------------------------- |
| `inherits`                | `string \| string[]` | `[]`    | Parent config file(s) to load first; child config overrides parent values            |
| `completion.enabled`      | `bool`               | `true`  | Enables completion responses                                                          |
| `diagnostics.enabled`     | `bool`               | `true`  | Enables/disables all diagnostic publishing (syntax + semantic arity)                 |
| `diagnostics.unknown_variables.enabled`  | `bool`               | `true`  | Enables/disables unknown-variable diagnostics                                           |
| `diagnostics.unknown_variables.exclude`  | `string \| string[]` | `[]`    | File/path patterns where unknown-variable diagnostics are skipped; relative patterns resolve from the config file that defines them |
| `diagnostics.unknown_variables.ignore`   | `string \| string[]` | `[]`    | Symbol names ignored by unknown-variable diagnostics (case-insensitive)               |
| `diagnostics.unknown_functions.enabled`  | `bool`               | `true`  | Enables/disables unknown-function diagnostics                                           |
| `diagnostics.unknown_functions.exclude`  | `string \| string[]` | `[]`    | File/path patterns where unknown-function diagnostics are skipped; relative patterns resolve from the config file that defines them |
| `diagnostics.unknown_functions.ignore`   | `string \| string[]` | `[]`    | Function names ignored by unknown-function diagnostics (case-insensitive)             |
| `semantic_tokens.enabled` | `bool`               | `true`  | Enables semantic token responses (DB table identifier highlighting)                   |
| `formatting.enabled`      | `bool`               | `false` | Enables/disables `textDocument/formatting` response                                    |
| `formatting.indent_size`  | `usize`              | `2`     | Spaces per indent level for formatter fallback/default behavior                        |
| `formatting.use_tabs`      | `bool`               | `false` | Prefer tabs for indentation (LSP editor options may override per request)             |
| `formatting.idempotence`   | `bool`               | `true`  | Runs second-pass formatting equality check before applying edits                       |
| `dumpfile`                | `string \| string[]` | `[]`    | Path(s) to `.df` dump files; relative paths resolve from the config file that defines them |
| `propath`                 | `string \| string[]` | `[]`    | Include search roots for `{...}` includes; relative paths resolve from the config file that defines them |

### Inheritance behavior

- `inherits` supports a single path or a list of paths.
- Relative paths are resolved from the current `abl.toml` directory.
- Parent config(s) are merged first, then the current file overrides them.
- `dumpfile` and `propath` are concatenated in merge order (parent entries first, then child entries).

### Dumpfile behavior

- `.df` files are parsed with `tree-sitter-df`.
- Schema index includes tables, fields, and indexes.
- Index reload is triggered when:
  - `abl.toml` changes
  - configured dumpfile is saved/changed

### Include resolution behavior

When resolving `{include.i}`, the server checks in this order:

1. Absolute include path (if include is absolute)
2. Each `propath` entry in merge order (including inherited configs)
3. Each config file directory participating in inheritance merge order (implicit include roots)

## License

```LICENSE
MIT License

Copyright (c) Kamil Jakubus and contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
