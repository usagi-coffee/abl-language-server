# abl-language-server

Language Server Protocol (LSP) implementation for ABL (OpenEdge Advanced Business Language).

The language server is under development and supports parser-based language features for ABL and optional DB schema integration via `.df` dump files.

## Feature matrix

| Feature | Status | Notes |
| --- | --- | --- |
| Text sync | Implemented | `TextDocumentSyncKind::FULL` |
| Parser diagnostics | Implemented | Tree-sitter syntax errors (`is_error` / `is_missing`) |
| Semantic diagnostics: function arity | Implemented | Checks `function_call` argument count against known function definitions (current file + included `.i` files) |
| Completion: local symbols | Implemented | Variables/definitions with case-insensitive prefix filtering |
| Completion: DB tables | Implemented | Uses configured `.df` dump files |
| Completion: DB fields after `table.` | Implemented | Supports table names and buffer aliases (`DEFINE BUFFER ... FOR ...`) |
| Completion item details/docs | Implemented | Field type in `detail`; `LABEL` / `FORMAT` / `DESCRIPTION` in docs when available |
| Go to Definition: local | Implemented | Local definitions |
| Go to Definition: includes | Implemented | Scoped include-aware function definitions |
| Go to Definition: DB schema | Implemented | Tables, fields, indexes from `.df`; buffer alias -> table definition |
| Find References: DB table definitions | Implemented | Returns matching `ADD TABLE` locations from `.df` |
| Hover: local symbols | Implemented | Type/detail hover |
| Hover: functions | Implemented | Signature with parameters + return type, include-aware |
| Hover: DB schema | Implemented | Table / field / index; field metadata includes type/label/format/description |
| Semantic tokens | Implemented | Highlights DB table identifiers (`token type: type`) |
| Rename | Not implemented | Returns `None` |
| Formatting | Not implemented | Returns `None` |
| References (general symbol refs) | Not implemented | Only DB table-definition lookup is implemented |

## Configuration (`abl.toml`)

The server searches for `abl.toml` in the opened workspace root.

### Supported options

```toml
[completion]
enabled = true

[diagnostics]
enabled = true

# One dumpfile
dumpfile = "database.df"

# Or many dumpfiles
# dumpfile = ["schema/core.df", "schema/custom.df"]
```

### Option reference

| Key | Type | Default | Description |
| --- | --- | --- | --- |
| `completion.enabled` | `bool` | `true` | Enables completion responses |
| `diagnostics.enabled` | `bool` | `true` | Reserved config flag (currently parser diagnostics and semantic arity checks are active in server flow) |
| `dumpfile` | `string \| string[]` | `[]` | Path(s) to `.df` dump files; relative paths resolve from workspace root |

### Dumpfile behavior

- `.df` files are parsed with `tree-sitter-df`.
- Schema index includes tables, fields, and indexes.
- Index reload is triggered when:
  - `abl.toml` changes
  - configured dumpfile is saved/changed

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
