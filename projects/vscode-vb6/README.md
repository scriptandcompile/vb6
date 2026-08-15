# VS Code VB6 Extension Scaffold

This package is a starter extension for VB6 syntax highlighting with a path to full language tooling.

## Features Included

1. VB6 language registration for:
   - `.bas`, `.cls`, `.frm`, `.ctl`, `.dob`, `.vbp`
2. TextMate grammar in `syntaxes/vb6.tmLanguage.json`
3. Language configuration in `language-configuration.json`
4. Keyword manifest and sync script from `vb6parse/src/lexer/mod.rs`
5. Parser diagnostics from `vb6parse` via a vendored WASM build, surfaced in the
   Problems panel as the document changes:
   - `vb6.parseFile` command ("VB6: Parse Current File") to re-parse on demand
   - Configurable via `vb6.diagnostics.enabled` and `vb6.diagnostics.debounceMs`
   - Parse errors map to errors; lexer warnings and recovery events map to warnings

## Development

1. `npm install`
2. `npm run compile`
3. Press `F5` in VS Code to run Extension Development Host

## Rebuilding the Parser WASM

The parser diagnostics are backed by a WASM build of `vb6parse` vendored into
`src/vendor/vb6parse`. To rebuild after changing the parser or lexer:

1. `npm run build-wasm`
2. `npm run compile` (copies `src/vendor` into `out/vendor`)
3. `npm run check-diagnostics` (parses the fixtures in `test-fixtures/` and
   verifies expected error/recovery counts)

The build honors `VB6PARSE_ROOT`, `WASMPACK_PATH`, and `WASMOPT_PATH` overrides.

## Keyword Sync

Run:

- `npm run update-keywords`

Default expected parser path:

- `../vb6parse/src/lexer/mod.rs`

If your layout differs:

- `VB6PARSE_LEXER_PATH=/absolute/path/to/mod.rs npm run update-keywords`

## Next Milestones

1. ~~Parser diagnostics via WASM bridge~~ (done)
2. Use keyword manifest to generate grammar lists automatically.
3. Add semantic tokens (WASM bridge) for richer token categories.
4. Add LSP client integration once `vb6-lsp` is implemented.
5. Add task/problem matcher integration for VB6 compile workflows.
