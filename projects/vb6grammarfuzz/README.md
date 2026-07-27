# vb6grammarfuzz

Grammar-based fuzzer for `vb6parse` that uses an ANTLR4 `.g4` grammar specification to generate random VB6 source code, then checks the resulting CST for `Unknown` tokens and minimizes failing inputs via delta debugging.

The [VisualBasic6.g4](https://github.com/uwol/proleap-vb6-parser) grammar is included as a git submodule and embedded into the compiled binary, so no external grammar file is needed by default.

## Getting Started

After cloning, initialize the submodule:

```bash
git submodule update --init --recursive
```

Build:

```bash
cargo build --release -p vb6grammarfuzz
```

## How It Works

1. **Parse** — Reads an ANTLR4 `.g4` grammar file into an internal IR. The embedded `VisualBasic6.g4` is used by default.
2. **Generate** — Walks the grammar rules to produce random VB6 source. Well-known lexer tokens (keywords, identifiers, literals, symbols) use an override table to emit realistic output.
3. **Check** — Parses the generated source with `vb6parse` and inspects the CST for `SyntaxKind::Unknown` tokens. Parsing runs in a separate thread with a 2-second timeout to handle pathological inputs.
4. **Reduce** — When an `Unknown` token is found, the input is minimized using a two-phase delta-debugging strategy (line-level, then character-level) to produce the smallest reproducing case.

## Usage

The `--grammar` flag is optional on `generate` and `fuzz`. When omitted, the embedded `VisualBasic6.g4` is used.

```bash
# Generate random VB6 source (uses embedded grammar)
cargo run --release -- generate --seed 42

# Generate using a custom grammar file
cargo run --release -- generate --grammar path/to/Custom.g4 --seed 42

# Check a .bas file for Unknown tokens
cargo run --release -- check --file input.bas

# Reduce a failing input to its minimal form
cargo run --release -- reduce --file failing.bas

# Run the full fuzz loop (generate → check → reduce → save)
cargo run --release -- fuzz --iterations 1000 --seed 1
```

Findings are saved to the `findings/` directory. Each finding includes a reduced file (`unknown_seed_N.bas`) and the original pre-reduction file (`unknown_seed_N_original.bas`).

## Project Structure

```
src/
  main.rs       — CLI (clap) with generate/check/reduce/fuzz subcommands
  g4_parser.rs  — ANTLR4 .g4 file parser → Grammar IR
  generator.rs  — Random VB6 source generator with lexer overrides
  checker.rs    — CST Unknown-token checker with timeout
  reducer.rs    — Delta-debugging minimizer (ddmin)
proleap-vb6-parser/   — git submodule (ANTLR4 VB6 grammar)
```
