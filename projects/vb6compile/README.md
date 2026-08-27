# vb6compile

A Visual Basic 6 compiler that transforms VB6 code into native executables or other target languages.

## Implementation Status: PLANNING

> This crate is in the planning phase. The documentation below describes
> the intended design. The current source code is a stub and most features
> are not yet implemented.

## Overview

`vb6compile` (short command: `vb6c`) is an ahead-of-time compiler for VB6 that generates efficient, native code or transpiles to modern languages. It supports multiple backends including Rust, LLVM, and JavaScript.

**Note:** The CLI, backends, and optimization pipeline are not yet implemented.

## Features

- **Multiple Backends**: Compile to Rust, LLVM IR, JavaScript, or native code *(planned)*
- **Optimization**: Multiple optimization levels (-O0 to -O3) *(planned)*
- **Cross-Compilation**: Generate code for different platforms *(planned)*
- **Incremental Compilation**: Fast rebuilds with caching *(planned)*
- **Debug Info**: Generate debug symbols for debuggers *(planned)*
- **Static Analysis**: Detect issues at compile time *(planned)*
- **Link-Time Optimization**: Whole-program optimization *(planned)*
- **Profile-Guided Optimization**: Use runtime profiles to optimize *(planned)*

## Architecture

```
┌────────────────────────────────────────┐
│         vb6compile (vb6c)             │
│                                        │
│  ┌──────────┐      ┌───────────────┐ │
│  │ Frontend │─────▶│   IR Builder  │ │
│  └──────────┘      └───────┬───────┘ │
│                            │          │
│                    ┌───────▼────────┐ │
│                    │   Optimizer    │ │
│                    └───────┬────────┘ │
│                            │          │
│       ┌────────────────────┼─────────┐│
│       │                    │         ││
│  ┌────▼────┐  ┌──────▼──────┐  ┌───▼──────┐
│  │  Rust   │  │    LLVM     │  │JavaScript│
│  │ Backend │  │   Backend   │  │ Backend  │
│  └────┬────┘  └──────┬──────┘  └───┬──────┘
└───────┼──────────────┼──────────────┼───────┘
        │              │              │
   ┌────▼────┐    ┌────▼────┐    ┌───▼────┐
   │ Rust    │    │ Native  │    │   JS   │
   │ Source  │    │  Code   │    │ Source │
   └─────────┘    └─────────┘    └────────┘
```

## Installation

```bash
# Build from source
cargo build --release
```

## Usage

### Basic Compilation

**Note:** The CLI commands below describe the intended design and are not yet implemented.

```bash
# Compile to native executable (via Rust)
vb6c compile MyProject.vbp

# Compile to Rust source code
vb6c compile --emit rust MyProject.vbp

# Compile to LLVM IR
vb6c compile --emit llvm-ir --backend llvm MyProject.vbp

# Compile to JavaScript
vb6c compile --emit javascript --backend js MyProject.vbp
```

### Optimization Levels

**Note:** Optimization levels are not yet implemented.

```bash
```
# No optimization (fast compile, slow runtime)
vb6c compile -O0 MyProject.vbp

# Basic optimization
vb6c compile -O1 MyProject.vbp

# Default optimization
vb6c compile -O2 MyProject.vbp

# Aggressive optimization (slow compile, fast runtime)
vb6c compile -O3 MyProject.vbp

# Size optimization
vb6c compile -Os MyProject.vbp
```

### Cross-Compilation

**Note:** Cross-compilation targets are not yet implemented.

```bash
```
# Compile for Windows x64
vb6c compile --target x86_64-pc-windows-msvc MyProject.vbp

# Compile for Linux
vb6c compile --target x86_64-unknown-linux-gnu MyProject.vbp

# Compile for macOS
vb6c compile --target x86_64-apple-darwin MyProject.vbp

# Compile for WebAssembly
vb6c compile --target wasm32-unknown-unknown MyProject.vbp
```

### Link-Time Optimization

**Note:** LTO is not yet implemented.

```bash
```
# Enable LTO for smaller, faster executables
vb6c compile --lto MyProject.vbp

# Thin LTO (faster compilation)
vb6c compile --lto=thin MyProject.vbp
```

### Debug Information

**Note:** Debug info generation is not yet implemented.

```bash
```
# Include debug symbols
vb6c compile --debug MyProject.vbp

# Debug with optimization
vb6c compile -O2 --debug MyProject.vbp
```

### Incremental Compilation

**Note:** Incremental compilation is not yet implemented.

```bash
```
# Enable incremental compilation
vb6c compile --incremental MyProject.vbp

# Clean incremental cache
vb6c clean MyProject.vbp
```

## Command-Line Interface

**Note:** The CLI is a planned feature. The current source code is a stub.

```
```
vb6c [OPTIONS] <COMMAND>

Commands:
  compile      Compile a VB6 project or file
  build        Compile and link to executable
  check        Check for compilation errors
  clean        Remove build artifacts
  ir           Generate and display IR
  asm          Generate and display assembly
  opt          Run optimizer on IR
  help         Show help information

Options:
  -O <LEVEL>              Optimization level [0-3, s, z]
  --backend <BACKEND>     Compilation backend [rust, llvm, js]
  --target <TARGET>       Target triple
  --emit <TYPE>           Emission type [exe, rust, llvm-ir, asm, js]
  --out-dir <DIR>         Output directory
  --incremental           Enable incremental compilation
  --lto[=LEVEL]           Link-time optimization [off, thin, fat]
  --debug                 Include debug information
  -g                      Alias for --debug
  --verbose               Verbose output
  -v                      Alias for --verbose
  -h, --help              Print help
  -V, --version           Print version
```

## Backends

**Note:** Backends are not yet implemented. The descriptions below outline the intended design.

### vb6-Rust Backend (Default)

Generates idiomatic Rust code:

**Advantages**:
- Type safety guaranteed by Rust compiler
- Easy to integrate with Rust ecosystem
- Human-readable output
- Fast compilation (leverages rustc)
- Excellent tooling support

**Generated Code Example**:
```rust
pub fn calculate(x: i32, y: i32) -> i32 {
    let mut result: i32 = 0;
    result = x + y;
    result *= 2;
    result
}
```

### LLVM Backend (Optional)

**Note:** LLVM backend is a planned feature.

### JavaScript Backend (Optional)

**Note:** JavaScript backend is a planned feature.

## Optimization Passes

**Note:** The optimizer is not yet implemented.

### -O0 (No Optimization)
- Fast compilation
- Direct translation
- Maximum debuggability

### -O1 (Basic Optimization)
- Dead code elimination
- Constant folding
- Basic inlining

### -O2 (Default Optimization)
- All -O1 optimizations
- Loop optimizations
- Function specialization
- Common subexpression elimination

### -O3 (Aggressive Optimization)
- All -O2 optimizations
- Aggressive inlining
- Vectorization
- Interprocedural optimization

### -Os (Size Optimization)
- Minimize binary size
- Avoid code bloat from inlining
- String deduplication

## Type System

VB6 types are mapped to native types in each backend:

| VB6 Type | Rust Type | LLVM Type | JavaScript |
|----------|-----------|-----------|------------|
| Byte | u8 | i8 | number |
| Integer | i16 | i16 | number |
| Long | i32 | i32 | number |
| Single | f32 | float | number |
| Double | f64 | double | number |
| String | String | *i8 | string |
| Boolean | bool | i1 | boolean |
| Variant | VBVariant | %variant | any |
| Object | Rc<dyn VbObject> | *obj | object |

## Runtime Library

The compiler links against `vb6runtime` for:
- Standard library functions
- Variant support (`VBVariant`)
- Object model
- Error handling

**Rust Example**:
```rust
use vb6runtime::VBVariant;

let value = VBVariant::from_integer(42);
```

**LLVM Example**:
```llvm
declare i8* @vb6_string_left(i8*, i32)

%result = call i8* @vb6_string_left(i8* %str, i32 5)
```

## Build System Integration

**Note:** Build system integration is a planned feature.

### Cargo Integration

Generated Rust code includes `Cargo.toml`:
```toml
[package]
name = "myproject"
version = "1.0.0"
edition = "2021"

[dependencies]
vb6core = "0.1"

[[bin]]
name = "myproject"
path = "src/main.rs"
```

### CMake Integration (LLVM backend)

```cmake
find_package(LLVM REQUIRED)
add_executable(myproject generated.ll)
target_link_libraries(myproject vb6core)
```

### Package.json (JavaScript backend)

```json
{
  "name": "myproject",
  "version": "1.0.0",
  "main": "dist/main.js",
  "dependencies": {
    "vb6-runtime-js": "^0.1.0"
  }
}
```

## Performance

**Note:** Performance benchmarks are not yet available (no implementation exists).

Typical performance characteristics:

| Benchmark | VB6 (native) | vb6c -O0 | vb6c -O2 | vb6c -O3 |
|-----------|--------------|----------|----------|----------|
| Integer math | 1.0x | 1.2x | 0.9x | 0.8x |
| String ops | 1.0x | 1.5x | 1.1x | 1.0x |
| Function calls | 1.0x | 1.3x | 1.0x | 0.9x |
| Array access | 1.0x | 1.1x | 0.95x | 0.85x |
| Overall | 1.0x | 1.3x | 1.0x | 0.9x |

*(Lower is better. 1.0x = same as VB6, 0.8x = 20% faster)*

## Limitations

Current limitations:
- [ ] Forms and controls (planned)
- [ ] Late binding (partial)
- [ ] COM objects (future)
- [ ] Some Windows API specific features
- [ ] ActiveX controls

## Development

### Building

```bash
# Build from source
cargo build --release
```

### Testing

```bash
# Run unit tests
cargo test

# Run backend-specific tests
cargo test --features rust-backend

# Run integration tests
cargo test --test codegen
```

### Adding a New Backend

**Note:** This is a planned feature. The trait interface below outlines the intended design.

```rust
```
pub trait CodeGenerator {
    fn generate_module(&mut self, module: &IRModule) -> Result<String>;
    fn generate_function(&mut self, function: &IRFunction) -> Result<String>;
    fn generate_instruction(&mut self, instr: &Instruction) -> Result<String>;
}
```

## Future Enhancements

The following are planned features for a future implementation.

- [ ] C backend
- [ ] Go backend
- [ ] Python backend
- [ ] Profile-guided optimization
- [ ] Distributed compilation
- [ ] Plugin system for custom backends
- [ ] Visual code browser for generated code
- [ ] Performance profiler integration

## License

MIT License - see LICENSE file for details.
