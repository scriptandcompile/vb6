# vb6codegen

Shared code generation library for VB6 conversion and compilation projects.

## Overview

`vb6codegen` provides a unified code generation framework used by both `vb6convert` (source-to-source converter) and `vb6compile` (ahead-of-time compiler). It includes backend implementations for multiple target languages and platforms.

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

### Backends

- **Rust** - Generate idiomatic Rust code
- **JavaScript** - Generate modern JavaScript (ES6+)
- **TypeScript** - Generate TypeScript with full type annotations
- **LLVM** - Generate LLVM IR for native compilation (optional)

### Integration with VB6 Libraries

**vb6runtime** - Core dependency for VB6 type system and semantics:
- Uses `vb6runtime::VBType` for type definitions
- Generated code links to `vb6runtime` for complex types (Variant, Currency, Date, Arrays)
- Maps VB6 standard library to `vb6runtime` implementations

**vb6core** - Optional dependency for IR-based generation:
- Can accept `vb6core::ir::Module` as input (used by vb6compile)
- Understands IR instruction set
- Not used by vb6convert (which works from AST)

### Utilities

- Type mapping and conversion
- Code formatting and indentation
- Naming convention transformations (snake_case, camelCase, PascalCase, etc.)
- Runtime library function mappings
- Standard library equivalents

## Usage

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

- `rust-backend` - Enable Rust code generation (default)
- `javascript-backend` - Enable JavaScript code generation
- `typescript-backend` - Enable TypeScript code generation
- `llvm-backend` - Enable LLVM IR generation (requires LLVM)
- `all-backends` - Enable all backends
- `serde-support` - Enable serde serialization support

## Type Mappings

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
| Variant | `vb6runtime::Variant` |

### JavaScript/TypeScript

| VB6 Type | JavaScript Type | TypeScript Type |
|----------|----------------|-----------------|
| Byte, Integer, Long, Single, Double | `number` | `number` |
| String | `string` | `string` |
| Boolean | `boolean` | `boolean` |
| Variant | - | `any` |

## Architecture

See [docs/DESIGN.md](docs/DESIGN.md) for detailed architecture information.

## Integration

### With vb6convert

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

```bash
# Build with default features
cargo build

# Build with all backends
cargo build --features all-backends

# Run tests
cargo test

# Run examples
cargo run --example rust_codegen --features rust-backend
```

## License

MIT
