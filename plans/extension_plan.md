# VB6 VS Code Extension Plan

Date: 2026-07-25

## Goals

Build a VB6 extension in phases, starting with syntax highlighting and evolving toward a full language extension with project tooling, compile/build integration, and a language server.

## Current Assets in vb6parse

1. Comprehensive token model in `src/language/tokens.rs`.
2. Canonical keyword map in `src/lexer/mod.rs`.
3. Rich syntax kind enum for CST in `src/parsers/syntaxkind.rs`.
4. WASM exports for tokenization/parsing in `src/wasm.rs`.
5. File-level parsers for `.vbp`, `.bas`, `.cls`, `.frm`, `.frx` in `src/files`.

These are enough to ship syntax highlighting now and build semantic features later.

## Architecture (Target)

At parent `vb6/`:

- `vb6parse/` (existing Rust parser core)
- `vscode-vb6/` (VS Code extension client)
- `vb6-lsp/` (future Rust language server)
- `vb6-compiler/` (future compile/build adapter tooling)

Design principle: parser is source of truth for language facts, editor artifacts are generated or validated from parser data.

## Phase Plan

## Phase 1: Syntax Highlighting Extension (Immediate)

Deliverables:

1. Language registration for VB6 file types:
   - `.bas`, `.cls`, `.frm`, `.ctl`, `.dob`, `.vbp`
2. TextMate grammar:
   - keywords, literals, comments, operators, declarations
3. Language configuration:
   - comments, brackets, auto-closing pairs, indentation rules
4. Scaffolded extension activation and packaging scripts

Acceptance criteria:

1. Opening VB6 files applies syntax highlighting with clear token classes.
2. Comments (`'` and `Rem`) are highlighted correctly.
3. String literals and numeric literals are highlighted correctly.
4. Folding/indentation behaves reasonably for `Sub/End Sub`, `If/End If`, etc.

## Phase 2: Parser-Synced Grammar Data (Near Term)

Deliverables:

1. Keyword extraction script reading `vb6parse/src/lexer/mod.rs`.
2. Generated keyword manifest consumed by grammar generation or validation.
3. CI check that extension keyword list is in sync with parser.

Acceptance criteria:

1. Single source of truth for keywords.
2. No manual keyword drift between parser and extension.

## Phase 3: Semantic Tokens via WASM (Optional Bridge)

Deliverables:

1. Extension-side semantic token provider backed by wasm exports.
2. Token category mapping from parser tokens to VS Code semantic token types.

Acceptance criteria:

1. Semantic highlighting enhances TextMate without regressions.
2. Performance remains acceptable for medium/large VB6 files.

## Phase 4: Full Language Server (Primary Long-Term Path)

Deliverables:

1. `vb6-lsp` Rust crate using `vb6parse` as core.
2. LSP features:
   - diagnostics
   - document symbols
   - workspace symbols
   - hover
   - go-to definition/references
   - rename (when symbol model matures)
3. VS Code extension integration with LSP client.

Acceptance criteria:

1. Reliable diagnostics and navigation in multi-file VB6 projects.
2. Extension starts/stops LSP cleanly per workspace.

## Phase 5: Build/Compile Integration

Deliverables:

1. Task provider for VB6 project build commands.
2. Problem matcher for compiler output.
3. Project discovery via `.vbp` parsing.
4. Later: debug adapter integration if needed.

Acceptance criteria:

1. Build commands are runnable from VS Code command palette/tasks.
2. Errors map to source locations in editor.

## Testing Strategy

1. Reuse real-world test files under `vb6parse/tests/data` as editor fixtures.
2. Add highlighting regression snapshots for representative files.
3. Keep parser tests and extension tests separate but linked by shared fixtures.
4. Add CI gates for:
   - parser tests
   - extension build/lint
   - keyword sync validation

## Migration and Repo Workflow

1. Build extension in a dedicated `vscode-vb6` folder.
2. During early development, reference local `vb6parse` paths for generated assets.
3. Move scaffold into parent `vb6` and wire CI there.
4. Keep versioning independent between parser crate and extension package.

## Risks and Mitigations

1. Risk: Grammar drift from parser keywords.
   - Mitigation: generated keyword manifest + CI sync checks.
2. Risk: Trying to deliver full LSP too early.
   - Mitigation: ship in phases, starting with TextMate.
3. Risk: VB6 file variants (`.frm` header properties, legacy syntax quirks).
   - Mitigation: broaden fixture coverage from `tests/data` corpus.

## Immediate Next Steps

1. Stand up `vscode-vb6` extension with TextMate grammar and language config.
2. Add keyword extraction script from `vb6parse` lexer map.
3. Validate highlighting against a sample set from parser test data.
4. Move scaffold into parent `vb6` and continue from there.
