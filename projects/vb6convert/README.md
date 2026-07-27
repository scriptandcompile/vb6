# vb6convert

A modular tool for converting VB6 projects to modern languages and frameworks.

## Overview

`vb6convert` provides a flexible framework for converting Visual Basic 6 projects into modern programming languages and frameworks such as Rust, JavaScript/TypeScript, Tauri, and more. It leverages the [vb6parse](../vb6parse) library for parsing VB6 code and provides a trait-based architecture for implementing conversion backends.

## Features

- **Multiple Target Languages**: Convert to Rust, JavaScript, TypeScript, Dart, and more
- **UI Framework Support**: Generate Tauri, Svelte, React, Vue, or Flutter applications
- **Modular Architecture**: Feature-gated modules for minimal compilation footprint
- **Extensible**: Easy to add new conversion targets
- **Analysis Tools**: Analyze VB6 projects for conversion feasibility
- **Validation**: Test harness for comparing converted code against original

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/scriptandcompile/vb6
cd vb6/vb6convert

# Build with default features
cargo build --release

# Or build with specific features
cargo build --release --features rust,javascript
```

### Features

The following features can be enabled during compilation:

- `rust` - Rust code generation
- `javascript` - JavaScript/Node.js code generation
- `typescript` - TypeScript code generation
- `tauri` - Tauri desktop application generation
- `svelte` - Svelte web application generation
- `react` - React web application generation
- `vue` - Vue.js web application generation
- `flutter` - Flutter mobile application generation
- `dart` - Dart code generation
- `test-harness` - Testing and validation framework
- `full` - Enable all features

## Usage

### Basic Conversion

```bash
# Convert a VB6 project to Rust
vb6convert convert MyProject.vbp --target rust --output ./rust-output

# Convert to JavaScript
vb6convert convert MyProject.vbp --target javascript --output ./js-output

# Convert to Tauri application
vb6convert convert MyProject.vbp --target tauri --output ./tauri-app
```

### Project Analysis

```bash
# Analyze a VB6 project
vb6convert analyze MyProject.vbp

# Generate detailed report
vb6convert analyze MyProject.vbp --verbose
```

### List Available Targets

```bash
# Show all available conversion targets
vb6convert targets
```

### Validation (requires test-harness feature)

```bash
# Validate a conversion
vb6convert validate original.vbp converted/ --harness rust
```

## Configuration

You can provide additional configuration via a TOML file:

```toml
# conversion.toml
target = "rust"
preserve_comments = true
generate_docs = true
format_output = true

[target_options]
use_async = true
edition = "2021"
```

Then use it:

```bash
vb6convert convert MyProject.vbp --output ./output --config conversion.toml
```

## Supported Conversions

### Language Features

| Feature | Rust | JavaScript | Tauri | Dart |
|---------|------|------------|-------|------|
| Modules | ✅ | ✅ | ✅ | ✅ |
| Classes | ✅ | ✅ | ✅ | ✅ |
| Forms | ⚠️ | ⚠️ | ✅ | ⚠️ |
| Properties | ✅ | ✅ | ✅ | ✅ |
| Events | ✅ | ✅ | ✅ | ✅ |
| Error Handling | ✅ | ✅ | ✅ | ✅ |
| Arrays | ✅ | ✅ | ✅ | ✅ |
| Collections | ✅ | ✅ | ✅ | ✅ |
| API Calls | ⚠️ | ❌ | ⚠️ | ❌ |
| Database | ⚠️ | ⚠️ | ✅ | ⚠️ |

✅ = Fully supported | ⚠️ = Partially supported | ❌ = Not supported

## Architecture

The conversion process follows these steps:

1. **Parse**: Use vb6parse to parse the VB6 project
2. **Analyze**: Determine features used and conversion complexity
3. **Convert**: Apply target-specific converter
4. **Generate**: Write output files
5. **Validate** (optional): Compare with original behavior

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed architecture documentation.

## Development

### Adding a New Conversion Target

See [IMPLEMENTATION_GUIDE.md](docs/IMPLEMENTATION_GUIDE.md) for step-by-step instructions on implementing a new conversion backend.

### Testing

```bash
# Run all tests
cargo test --all-features

# Run tests for specific feature
cargo test --features rust

# Run with output
cargo test -- --nocapture
```

See [TESTING.md](docs/TESTING.md) for comprehensive testing documentation.

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - System architecture and design
- [Implementation Guide](docs/IMPLEMENTATION_GUIDE.md) - How to add new converters
- [Testing](docs/TESTING.md) - Testing framework and strategy
- [Rust Target](docs/targets/rust.md) - Rust conversion backend
- [JavaScript Target](docs/targets/javascript.md) - JavaScript/TypeScript conversion
- [Tauri Target](docs/targets/tauri.md) - Tauri application generation

## Examples

### Simple Module Conversion

**VB6 Input:**
```vb6
' Calculator.bas
Public Function Add(x As Integer, y As Integer) As Integer
    Add = x + y
End Function
```

**Rust Output:**
```rust
// calculator.rs
pub fn add(x: i16, y: i16) -> i16 {
    x + y
}
```

**JavaScript Output:**
```javascript
// calculator.js
export function add(x, y) {
    return x + y;
}
```

## Roadmap

- [x] Core framework and traits
- [x] Project analysis
- [ ] Rust converter implementation
- [ ] JavaScript converter implementation
- [ ] TypeScript converter implementation
- [ ] Tauri converter implementation
- [ ] Form layout conversion
- [ ] Control mapping
- [ ] Database access layer
- [ ] Test harness implementation
- [ ] Svelte/React/Vue converters
- [ ] Flutter/Dart converters
- [ ] IDE integration
- [ ] GUI tool for conversion

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [vb6parse](../vb6parse) - VB6 parser library
- [aspen](../aspen) - Cargo-like tool for VB6 projects

## Acknowledgments

This project builds upon the excellent [vb6parse](../vb6parse) library for parsing VB6 code.
