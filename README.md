# abl-language-server

Language Server Protocol (LSP) implementation for ABL (OpenEdge Advanced Business Language), supports parser-based language features for ABL and optional DB schema integration via `.df` dump files.

The language server currently does not touch your files, it's strictly read-only, there should be no risk of file corruption.

## Extensions 

[zed-openedge-abl](https://github.com/usagi-coffee/zed-openedge-abl)

[vscode-openedge-abl](https://github.com/usagi-coffee/vscode-openedge-abl)

## Features

| Feature | Notes |
| --- | --- |
| Text sync | `TextDocumentSyncKind::FULL` |
| Parser diagnostics | Tree-sitter syntax errors (`is_error` / `is_missing`) |
| Semantic diagnostics: function arity | Checks `function_call` argument count against known function definitions (current file + included `.i` files) |
| Completion: local symbols | Variables/definitions with case-insensitive prefix filtering |
| Completion: DB tables | Uses configured `.df` dump files |
| Completion: DB fields after `table.` | Supports table names and buffer aliases (`DEFINE BUFFER ... FOR ...`) |
| Completion item details/docs | Field type in `detail`; `LABEL` / `FORMAT` / `DESCRIPTION` in docs when available |
| Go to Definition: local | Local definitions |
| Go to Definition: includes | Scoped include-aware function definitions |
| Go to Definition: DB schema | Tables, fields, indexes from `.df`; buffer alias -> table definition |
| Find References: DB table definitions | Returns matching `ADD TABLE` locations from `.df` |
| Hover: local symbols | Type/detail hover |
| Hover: functions | Signature with parameters + return type, include-aware |
| Hover: DB schema | Table / field / index; field metadata includes type/label/format/description |
| Semantic tokens | Highlights DB table identifiers (`token type: type`) |

## Configuration (`abl.toml`)

The server searches for `abl.toml` in the opened workspace root.

### Supported options

```toml
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

[semantic_tokens]
enabled = true
```

### Option reference

| Key | Type | Default | Description |
| --- | --- | --- | --- |
| `completion.enabled` | `bool` | `true` | Enables completion responses |
| `diagnostics.enabled` | `bool` | `true` | Enables/disables all diagnostic publishing (syntax + semantic arity) |
| `semantic_tokens.enabled` | `bool` | `true` | Enables semantic token responses (DB table identifier highlighting) |
| `dumpfile` | `string \| string[]` | `[]` | Path(s) to `.df` dump files; relative paths resolve from workspace root |
| `propath` | `string \| string[]` | `[]` | Include search roots for `{...}` includes; relative paths resolve from workspace root |

### Dumpfile behavior

- `.df` files are parsed with `tree-sitter-df`.
- Schema index includes tables, fields, and indexes.
- Index reload is triggered when:
  - `abl.toml` changes
  - configured dumpfile is saved/changed

### Include resolution behavior

When resolving `{include.i}`, the server checks in this order:

1. Absolute include path (if include is absolute)
2. Each `propath` entry in the order listed in `abl.toml`
3. Workspace root

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
