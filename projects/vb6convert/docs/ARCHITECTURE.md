# vb6convert Architecture

## Overview

`vb6convert` is a modular conversion framework for transforming VB6 projects into modern languages and frameworks. It leverages the `vb6parse` library for parsing and understanding VB6 code, then applies pluggable converters to generate equivalent code in the target language.

## Design Principles

1. **Modularity**: Each target language/framework is implemented as a separate feature-gated module
2. **Extensibility**: New conversion targets can be added without modifying core code
3. **Trait-based**: All converters implement a common set of traits defining the conversion interface
4. **Composability**: Complex targets (like Tauri) compose simpler converters (Rust + HTML + CSS + JS)
5. **Validation**: Built-in testing framework to validate conversions against known-good implementations

## Architecture Layers

### Layer 1: Core Framework
- **Traits** (`traits.rs`): Core trait definitions that all converters implement
- **Types** (`types.rs`): Common types used across the framework
- **Error Handling** (`error.rs`): Unified error types
- **Converter Registry** (`converters.rs`): Factory and registry for converter instances

### Layer 2: Analysis & Planning
- **Project Analyzer** (`analysis.rs`): Analyzes VB6 projects to determine:
  - Complexity score
  - Feature usage
  - Potential conversion issues
  - Recommended target platforms
- **Validation** (`validation.rs`): Validates conversion results

### Layer 3: Conversion Backends (Feature-gated)
Each conversion target is implemented as a feature-gated module:

```
src/
  rust/          # feature = "rust-code"
  javascript/    # feature = "js-code"
  typescript/    # feature = "typescript"
  dart/          # feature = "dart"
  html/          # feature = "html"
  css/           # feature = "css"
  tauri/         # feature = "tauri"
  svelte/        # feature = "svelte"
  react/         # feature = "react"
  vue/           # feature = "vue"
  flutter/       # feature = "flutter"
```

### Layer 4: Testing & Validation
- **Test Harness** (`testing/`): Framework for comparing converted code against reference implementations
- Unit tests for each converter
- Integration tests for complete project conversions

## Conversion Flow

```
┌─────────────────┐
│  VB6 Project    │
│  (.vbp, .frm,   │
│   .bas, .cls)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  vb6parse       │
│  Parse & AST    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Project        │
│  Analyzer       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Converter      │
│  Selection      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Target         │
│  Converter      │
│  (Rust/JS/etc)  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Code           │
│  Generation     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Validation     │
│  (Optional)     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Output Files   │
└─────────────────┘
```

## Trait Hierarchy

### ProjectConverter (Main Interface)
Every target must implement the `ProjectConverter` trait:

```rust
pub trait ProjectConverter {
    fn name(&self) -> &str;
    fn convert_project(&self, project: &Project, config: &ConversionConfig) 
        -> Result<ConversionResult>;
    fn supports_feature(&self, feature: VB6Feature) -> bool;
    fn required_dependencies(&self) -> Vec<Dependency>;
}
```

### Specialized Converters
Depending on the target, converters may also implement:

- **ModuleConverter**: For converting .bas files
- **ClassConverter**: For converting .cls files  
- **FormConverter**: For converting .frm files (UI)
- **ControlConverter**: For converting individual controls
- **ExpressionConverter**: For converting expressions and statements
- **TypeConverter**: For mapping VB6 types to target types

## Feature Gates

Conversion targets are feature-gated to:
1. Reduce compilation time when only specific targets are needed
2. Minimize binary size
3. Allow conditional dependencies
4. Support optional functionality

### Feature Hierarchy
```
full
├── rust
│   ├── rust-code
│   └── rust-ui
│       └── tauri
├── javascript
│   ├── js-code
│   └── js-ui
│       ├── html
│       ├── css
│       ├── svelte
│       ├── react
│       └── vue
├── typescript (implies javascript)
└── flutter
    └── dart
```

## File Organization

```
vb6convert/
├── Cargo.toml              # Package config with feature gates
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # CLI entry point
│   ├── error.rs            # Error types
│   ├── types.rs            # Common types
│   ├── traits.rs           # Core trait definitions
│   ├── converters.rs       # Converter registry
│   ├── analysis.rs         # Project analysis
│   ├── validation.rs       # Validation utilities
│   │
│   ├── rust/               # Rust converter (feature gated)
│   │   ├── mod.rs
│   │   ├── converter.rs
│   │   ├── expressions.rs
│   │   ├── types.rs
│   │   └── ui.rs
│   │
│   ├── javascript/         # JS converter (feature gated)
│   │   ├── mod.rs
│   │   ├── converter.rs
│   │   └── ...
│   │
│   ├── tauri/             # Tauri converter (feature gated)
│   │   ├── mod.rs
│   │   └── ...
│   │
│   └── testing/           # Test harness (feature gated)
│       ├── mod.rs
│       ├── harness.rs
│       └── ...
│
├── docs/                  # Documentation
│   ├── ARCHITECTURE.md    # This file
│   ├── IMPLEMENTATION_GUIDE.md
│   ├── TESTING.md
│   └── targets/           # Per-target documentation
│       ├── rust.md
│       ├── javascript.md
│       └── ...
│
└── tests/                 # Integration tests
    ├── rust_conversion.rs
    └── ...
```

## Extension Points

To add a new conversion target:

1. Create a new feature in `Cargo.toml`
2. Create a new module under `src/`
3. Implement the required traits (at minimum `ProjectConverter`)
4. Register the converter in the `ConverterRegistry`
5. Add documentation in `docs/targets/`
6. Add tests in `tests/`

## Configuration

Conversions can be configured via:
1. Command-line arguments
2. Configuration files (TOML)
3. Programmatic API

Configuration includes:
- Target language/framework
- Output directory structure
- Code style preferences
- Feature mappings
- Dependency versions
- Custom templates

## Dependencies

Core dependencies:
- `vb6parse`: For parsing VB6 code
- `clap`: CLI argument parsing
- `serde`: Serialization/deserialization
- `thiserror`: Error handling
- `anyhow`: General error handling

Target-specific dependencies are only included when their feature is enabled.
