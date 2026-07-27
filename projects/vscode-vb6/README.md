# VS Code VB6 Extension Scaffold

This package is a starter extension for VB6 syntax highlighting with a path to full language tooling.

## Features Included

1. VB6 language registration for:
   - `.bas`, `.cls`, `.frm`, `.ctl`, `.dob`, `.vbp`
2. TextMate grammar in `syntaxes/vb6.tmLanguage.json`
3. Language configuration in `language-configuration.json`
4. Keyword manifest and sync script from `vb6parse/src/lexer/mod.rs`

## Development

1. `npm install`
2. `npm run compile`
3. Press `F5` in VS Code to run Extension Development Host

## Keyword Sync

Run:

- `npm run update-keywords`

Default expected parser path:

- `../vb6parse/src/lexer/mod.rs`

If your layout differs:

- `VB6PARSE_LEXER_PATH=/absolute/path/to/mod.rs npm run update-keywords`

## Next Milestones

1. Use keyword manifest to generate grammar lists automatically.
2. Add semantic tokens (WASM bridge) for richer token categories.
3. Add LSP client integration once `vb6-lsp` is implemented.
4. Add task/problem matcher integration for VB6 compile workflows.
