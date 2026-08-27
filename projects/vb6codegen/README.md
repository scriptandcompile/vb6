# vb6codegen

Shared code generation library for VB6 conversion and compilation projects.

## Implementation Status: PLANNING

> This crate is in the planning phase. The documentation below describes
> the intended design. `src/lib.rs` is currently empty and all described types
> (`CodegenBackend`, `RustBackend`, `JavaScriptBackend`, `CodegenConfig`, `CaseStyle`)
> are not yet implemented.

## Overview

`vb6codegen` provides a unified code generation framework used by both `vb6convert` (source-to-source converter) and `vb6compile` (ahead-of-time compiler). It includes backend implementations for multiple target languages and platforms.

**Note:** All described functionality is planned but not yet implemented.

## Architecture

```
┌──────────────────────────────────────────────┐
│              vb6codegen                      │
│  ┌────────────────────────────────────────┐ │
│  │  Backend Traits & Interfaces           │ │
│  └────────────────────────────────────────┘ │
│  ┌────────┐ ┌──────────┐ ┌──────────────┐  │
│  │  Rust  │ │JavaScript│ │ TypeScript   │  │
│  │Backend │ │ Backend  │ │   Backend    │  │
│  └───┬────┘ └────┬─────┘ └──────┬───────┘  │
└──────┼───────────┼───────────────┼──────────┘
       │           │               │
       │        ┌──▼───────────────▼──┐
       │        │    vb6runtime       │ ◄──── Type system, VB6 semantics
       │        └──┬──────────────────┘       Standard library mappings
       │           │
       ├───────────┘
       │
   ┌───▼───────────────────────────┐
   │        Generated Code         │
   │  (links to vb6runtime)        │
   └───────────────────────────────┘

Used by:
  ┌──────────────┐        ┌──────────────┐
  │  vb6convert  │        │  vb6compile  │
  │ (AST → Code) │        │ (IR → Code)  │
  └──────────────┘        └──────────────┘
```

## Purpose

Both `vb6convert` and `vb6compile` need to generate code in the same target languages. Rather than duplicating code generation logic, this library consolidates:

- Backend trait interfaces for code generation
- Type system mappings from VB6 to target languages
- Code generators for Rust, JavaScript, TypeScript, LLVM IR, etc.
- Formatting and naming convention utilities
- Runtime library function mappings

## Features

### Backends *(planned)*

- **Rust** - Generate idiomatic Rust code *(not implemented)*
- **JavaScript** - Generate modern JavaScript (ES6+) *(not implemented)*
- **TypeScript** - Generate TypeScript with full type annotations *(not implemented)*
- **LLVM** - Generate LLVM IR for native compilation *(not implemented)*

### Integration with VB6 Libraries

*(planned)*

### Utilities

*(planned)*

## Usage

**Note:** The API described below is planned but not yet implemented.

Add this to your `Cargo.toml`:

```toml
[dependencies]
vb6codegen = { path = "../vb6codegen", features = ["rust-backend"] }
```

### Basic Example

```rust
use vb6codegen::{CodegenBackend, RustBackend, CodegenConfig, CaseStyle};

// Create a Rust code generator
let mut backend = RustBackend::new();

// Configure generation
let config = CodegenConfig {
    target: "rust".to_string(),
    generate_comments: true,
    naming: NamingConfig {
        function_case: CaseStyle::Snake,
        variable_case: CaseStyle::Snake,
        type_case: CaseStyle::Pascal,
    },
    ..Default::default()
};

backend.initialize(&config)?;

// Use the backend to generate code
// ... generation logic ...

// Finalize and get results
let generated = backend.finalize()?;
for (path, content) in generated.files {
    println!("Generated: {}", path.display());
}
```

## Feature Flags

All feature flags below are planned but not yet defined in `Cargo.toml`:

- `rust-backend` - Enable Rust code generation *(planned)*
- `javascript-backend` - Enable JavaScript code generation *(planned)*
- `typescript-backend` - Enable TypeScript code generation *(planned)*
- `llvm-backend` - Enable LLVM IR generation *(planned)*
- `all-backends` - Enable all backends *(planned)*
- `serde-support` - Enable serde serialization support *(planned)*

## Type Mappings

*(planned — not yet implemented)*

### Rust

| VB6 Type | Rust Type |
|----------|-----------|
| Byte | `u8` |
| Integer | `i16` |
| Long | `i32` |
| Single | `f32` |
| Double | `f64` |
| String | `String` |
| Boolean | `bool` |
| Variant | `vb6runtime::VBVariant` |

### JavaScript/TypeScript

| VB6 Type | JavaScript Type | TypeScript Type |
|----------|----------------|-----------------|
| Byte, Integer, Long, Single, Double | `number` | `number` |
| String | `string` | `string` |
| Boolean | `boolean` | `boolean` |
| Variant | - | `any` |

## Architecture

**Note:** The architecture described below is planned but not yet implemented.

See [docs/DESIGN.md](docs/DESIGN.md) for detailed architecture information.

## Integration

**Note:** Integration with `vb6convert` and `vb6compile` is planned but not yet implemented.

`vb6convert` uses `vb6codegen` to generate target code after parsing and analyzing VB6 source:

```
VB6 Source → Parse → AST → vb6codegen → Target Code
```

### With vb6compile

`vb6compile` uses `vb6codegen` as its final code generation stage:

```
VB6 Source → Parse → AST → IR → Optimize → vb6codegen → Target Code
```

## Development

**Note:** The commands below are for a future implementation.

```bash
# Build (not yet possible - crate is empty)
# cargo build

# Run tests (not yet possible)
# cargo test
```

## License

MIT
