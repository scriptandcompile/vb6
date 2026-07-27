# vb6-lsp (Placeholder)

Future Rust language server implementation should live here.

Planned features:

1. Diagnostics
2. Document/workspace symbols
3. Hover
4. Definition/references
5. Rename

Recommended approach:

1. Use `vb6parse` as the parsing core.
2. Implement LSP protocol via `tower-lsp` or equivalent.
