# VB6 Rust Workspace

A complete Rust toolchain for VB6 projects — parsing, semantic analysis,
cross-compilation, interpretation, and formatting.

## Project Status

| Crate | Status | Description |
|-------|--------|-------------|
| vb6parse | Active | VB6 source code parser |
| vb6core | Active | VB6 core type system |
| vb6runtime | Active | VB6 standard library and runtime |
| vb6semantic | Active | Semantic analysis and type checking |
| vb6interpret | Active | VB6 interpreter and REPL |
| vb6format | Active | VB6 source code formatter |
| vb6harness | Active | Test harness and golden-file comparisons |
| vb6grammarfuzz | Active | Grammar fuzzing tool |
| aspen | Active | Workspace-level analyzer and formatter |
| vb6compile | Planning | Compiler pipeline (not yet implemented) |
| vb6codegen | Planning | Code generation backends (not yet implemented) |
| vb6libraries | Planning | Library detection and mapping (not yet implemented) |
| vb6convert | Planning | Cross-language conversion tools (not yet implemented) |
| vb6lsp | Frozen | Language server (stub only) |
| vscode-vb6 | Frozen | VS Code extension (stub only) |

## Getting Started

```bash
# Build the workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Run the interpreter
cargo run -p vb6interpret -- run example.bas
```

## Architecture

The workspace is organized into layers:

1. **Foundation** — vb6core (types), vb6parse (parsing)
2. **Runtime** — vb6runtime (standard library, I/O)
3. **Analysis** — vb6semantic (symbol tables, type checking), aspen (workspace analysis)
4. **Execution** — vb6interpret (interpreted execution)
5. **Codegen** — vb6compile, vb6codegen (future: compiled output)
6. **Tooling** — vb6harness (testing), vb6format (formatting), vb6grammarfuzz (fuzzing)

## Links

- [Workspace hub](https://scriptandcompile.github.io/vb6)
- [Workspace status board](https://scriptandcompile.github.io/vb6/status.html)

## Contributing

Each crate has its own documentation. See `projects/<crate>/README.md` for per-project guides.
