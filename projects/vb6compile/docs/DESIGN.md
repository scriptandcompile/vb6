# vb6compile Design Document

## Overview

`vb6compile` (command-line tool: `vb6c`) is a transpiler and build orchestrator that transforms VB6 source code into Rust, then delegates compilation to `rustc` via `cargo`. It uses `vb6convert` as the transpiler library and `vb6runtime` as the linked runtime library. The generated Rust project is self-contained and compiles with standard Cargo tooling.

## Goals

1. **Correctness**: Preserve exact VB6 semantics through faithful Rust transpilation
2. **Performance**: Native binaries via rustc — leverage LLVM optimizations
3. **Cross-Platform**: Compile to any target rustc supports (Windows, Linux, macOS, Web)
4. **Maintainability**: Generate readable, debuggable Rust code with source maps
5. **Simplicity**: No custom IR, no custom optimizer — let rustc do the heavy lifting

## Architecture

### Component Structure

```
vb6compile/
├── src/
│   ├── main.rs              # CLI entry point (thin, delegates to library)
│   ├── lib.rs               # Library interface
│   ├── pipeline/
│   │   ├── mod.rs           # Pipeline orchestration: parse → convert → build
│   │   ├── project.rs       # Project file resolution, module enumeration
│   │   └── build.rs         # Invoke cargo/rustc with proper config
│   └── config.rs            # Conversion configuration, feature flags
├── tests/
│   ├── integration/
│   │   ├── simple_form/     # Form-only VB6 project
│   │   ├── modules_only/    # .bas/.cls-only VB6 project
│   │   └── mixed/           # Full VB6 project with forms, modules, classes
│   └── snapshots/           # Golden tests for generated Rust code
└── benches/
    └── compilation.rs
```

### High-Level Pipeline

```
┌──────────────────────────────────────────────────────────────────────┐
│  vb6compile (CLI: vb6c)                                              │
│                                                                      │
│  1. Parse CLI args                                                   │
│  2. Resolve VB6 project (.vbp) → enumerate .frm/.bas/.cls files      │
│  3. Call vb6convert::convert_project(project, config)                │
│     ├── vb6parse: parse each file into AST                           │
│     ├── vb6convert: transpile AST → Rust source                      │
│     │   ├── ModuleConverter: .bas files → Rust modules               │
│     │   ├── ClassConverter: .cls files → Rust structs + impl blocks  │
│     │   ├── FormConverter: .frm files → Rust code-behind + layout    │
│     │   ├── ExpressionConverter: VB6 expressions → Rust              │
│     │   └── TypeConverter: VB6 types → Rust types                    │
│     └── Output: Rust source files + Cargo.toml in temp dir           │
│  4. Invoke cargo build (or rustc) with proper config                 │
│     ├── Link against vb6runtime                                      │
│     ├── Apply rustc optimization flags (from -O level)               │
│     └── Produce native executable                                    │
└──────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Role |
|---|---|
| `vb6parse` | Parse VB6 source → AST (`FormRoot`, `ModuleFile`, `ClassFile`) |
| `vb6convert` | Transpile AST → Rust source code |
| `vb6runtime` | Runtime library linked into the generated binary |
| `vb6compile` (pipeline) | Orchestrate conversion + invoke cargo |
| `rustc`/`cargo` | Compile generated Rust → native binary |

**Key principle:** vb6compile does not generate IR, does not implement optimization passes, and does not emit code itself. It delegates to vb6convert for transpilation and to rustc for compilation.

## Transpilation

### Transpilation Pipeline

vb6convert handles all source-to-source conversion. vb6compile invokes it and receives Rust source code.

#### Module Conversion (.bas files)

VB6 standard modules become Rust modules. Subroutines and functions become `pub fn` items:

```rust
// VB6 input (Module1.bas)
Public Sub Hello()
    MsgBox "Hello, World!"
End Sub

Private Function Add(a As Long, b As Long) As Long
    Add = a + b
End Function
```

```rust
// Generated Rust (module1.rs)
pub fn hello() {
    vb6runtime::library::statements::msgbox::vb6_msgbox(&"Hello, World!".into())
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### Class Conversion (.cls files)

VB6 class modules become Rust `struct` + `impl` blocks. Class events (`Class_Initialize`, `Class_Terminate`) become explicit methods.

#### Form Conversion (.frm files)

VB6 forms are split into two parts:

1. **Code-behind** — the event handlers and module-level code, converted to Rust code that will be linked against the layout system.
2. **Layout** — the form's visual design is loaded into `vb6runtime::layout` at runtime. The generated Rust calls `vb6runtime::layout::load_form()` to construct the UI tree.

The `FormConverter` trait in vb6convert defines:
- `convert_layout()` — produces the `vb6runtime::layout` call tree (or delegates to the layout engine)
- `convert_code_behind()` — produces the Rust event handler code

Forms are **not** compiled into standalone HTML/CSS. They are rendered by `vb6runtime::layout` at runtime, which supports both WASM (`WebSysRenderer`) and Tauri (`TauriRenderer`) through a shared `Renderer` trait. The generated Rust code invokes the layout system — the renderer selection is a runtime choice made by the host application.

#### Type Mapping

| VB6 Type | Rust Type |
|---|---|
| `Byte` | `u8` |
| `Integer` | `i16` |
| `Long` | `i32` |
| `Single` | `f32` |
| `Double` | `f64` |
| `Currency` | `f64` (with VB6 rounding semantics) |
| `String` | `String` |
| `Boolean` | `bool` |
| `Variant` | `vb6runtime::VBVariant` |
| `Object` | `vb6runtime::VBObject` |
| `Date` | `jiff::CivilDateTime` |
| Arrays | `vb6runtime::ArrayValue` |
| User-defined types | Rust `struct` with matching fields |

#### Expression Conversion

VB6 expressions are converted to equivalent Rust expressions. Key differences handled:

- **1-based arrays** → Rust generates `ArrayValue` wrapper with 1-based indexing semantics
- **Variant coercion** → automatic `VBVariant` boxing/unboxing where needed
- **String concatenation** — `+` vs `&` — VB6's `+` for strings uses `&` in Rust
- **Division** — `/` always produces floating point; `\"` (integer division) produces `i32`
- **Integer overflow** — VB6 wraps by default; Rust uses `wrapping_*` methods in debug builds
- **Date literals** — `#1/1/2024#` → `jiff::CivilDateTime::from_ymd(2024, 1, 1).unwrap()`

### Form-to-Rust Integration

When a form is loaded (via the VB6 `Load` statement), the generated code calls:

```rust
// Generated by FormConverter.convert_code_behind()
fn load_form1() -> vb6runtime::layout::FormHandle {
    let form_file = include_bytes!("form1.frm");
    let root = vb6parse::FormFile::parse(form_file).expect("parse form1");
    let handle = vb6runtime::layout::load_form(&root.form, &vb6runtime::layout::LayoutConfig::default());
    handle
}
```

The layout system (`vb6runtime::layout`) owns the canonical form tree. The generated code loads forms into it; the runtime renders them. The host application decides which renderer to use (WASM or Tauri) — the transpiled code doesn't make that choice.

## Build Orchestration

After vb6convert produces Rust source files, vb6compile invokes cargo:

```rust
pub struct BuildOrchestrator {
    output_dir: PathBuf,
    opt_level: OptLevel,
    target: Option<String>,
    debug: bool,
    lto: Option<LtoLevel>,
}

impl BuildOrchestrator {
    pub fn build(&self) -> Result<BuildResult> {
        // 1. Write a Cargo.toml that depends on vb6runtime
        self.write_cargo_toml()?;
        
        // 2. Write source files to output_dir/src/
        //    (already written by vb6convert during conversion)
        
        // 3. Invoke cargo
        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("build")
           .arg("--release")  // or debug based on flags
           .current_dir(&self.output_dir);
        
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }
        
        let output = cmd.output()?;
        // ... handle success/failure
    }
}
```

**Generated Cargo.toml** (simplified):

```toml
[package]
name = "converted-app"
version = "0.1.0"
edition = "2024"

[dependencies]
vb6runtime = { path = "path/to/vb6runtime" }
vb6parse = { path = "path/to/vb6parse" }
jiff = "0.2"

[profile.release]
opt-level = 3
lto = true
```

## CLI Design

```
vb6c compile <path> [options]

Positional:
  path              Path to .vbp project or individual .frm/.bas/.cls file

Options:
  -o, --out <dir>   Output directory (default: target/vb6c)
  -O, --opt <level> Optimization level: 0, 1, 2 (default), 3, s
  --debug           Include debug symbols
  --target <triple> Rust target triple (e.g., x86_64-unknown-linux-gnu)
  --verbose         Verbose output
  --clean           Remove previous build artifacts
```

**Command mapping to the old CLI:**

| Old Command | New Behavior |
|---|---|
| `Compile` | Convert + build (default) |
| `Build` | Convert + build |
| `Check` | Convert only (no cargo build), report errors |
| `Clean` | Remove output directory |
| `Ir` | Removed — no IR layer exists |
| `Asm` | `rustc --emit=asm` via cargo flag |
| `--backend rust` | (removed) — Rust is the only backend |
| `--backend llvm` | `rustc` with LLVM; vb6compile doesn't generate LLVM IR directly |
| `--backend js` | Removed — no JS backend |
| `--emit rust` | Emit only: convert and write Rust files, do not invoke cargo |
| `--emit exe` | Emit + build (default) |
| `--lto` | Pass through to generated Cargo.toml profile |

## Testing Strategy

### Snapshot Tests
- For each test fixture, capture the generated Rust source
- Verify structure, type mappings, and VB6 semantics
- Update snapshots when converter logic changes

### Integration Tests
- Full VB6 projects compile end-to-end
- Generated binary runs correctly
- Forms load and render via `vb6runtime::layout`

### Cross-Platform Tests
- Same project compiles on Windows, Linux, macOS
- VB6 semantics preserved across platforms (especially dates, currency, string encoding)

## Performance

### Compilation Speed
- vb6convert runs once per incremental build
- cargo handles incremental compilation of generated Rust
- No custom optimization passes to delay compilation

### Runtime Performance
- rustc's LLVM optimizations (O1-O3) handle all optimization
- vb6runtime uses zero-cost abstractions where possible
- Generated code is idiomatic Rust — rustc optimizes it effectively

## Dependencies

- `vb6parse`: parse VB6 source → AST
- `vb6convert`: transpile AST → Rust source
- `vb6runtime`: linked into generated binaries at runtime
- `clap`: CLI argument parsing
- `anyhow`/`thiserror`: error handling

## Comparison: vb6compile vs vb6interpret

| | vb6compile | vb6interpret |
|---|---|---|
| **Purpose** | Compile VB6 → native binary | Execute VB6 in-place |
| **Output** | `.exe` or `.dll` | Nothing — executes directly |
| **VB6 forms** | Transpiled to Rust, uses `vb6runtime::layout` at runtime | Interpreted, uses `vb6runtime::layout` at runtime |
| **Runtime** | Generated binary links `vb6runtime` | Binary links `vb6runtime` |
| **Uses vb6convert?** | Yes | No |
| **Uses vb6core?** | No | Yes |

Both use `vb6runtime` as the runtime and `vb6runtime::layout` for forms, but vb6compile generates Rust code that calls into `vb6runtime`, while vb6interpret executes VB6 statements directly through the interpreter.
