# vb6codegen Design Document

## Overview

`vb6codegen` is a shared code generation library that provides backend code generators for multiple target languages and platforms. It is used by both `vb6convert` (source-to-source converter) and `vb6compile` (ahead-of-time compiler) to generate code in target languages like Rust, JavaScript, TypeScript, and others.

## Purpose

Both `vb6convert` and `vb6compile` need to generate code in the same target languages (Rust, JavaScript, LLVM IR, etc.). Rather than duplicating the code generation logic, type mappings, and backend implementations across both projects, these are consolidated into `vb6codegen`.

### What is Shared

- **Backend Traits**: Common interfaces for code generation
- **Type System**: Mappings from VB6 types to target language types
- **Code Generators**: Implementations for Rust, JavaScript, TypeScript, LLVM, etc.
- **Formatting Utilities**: Code formatting, naming conventions, indentation
- **Runtime Mappings**: Standard library function mappings to target equivalents

### What Remains Separate

- **vb6convert**: High-level conversion logic, project analysis, UI framework support
- **vb6compile**: Compilation pipeline, IR optimization, linking, incremental compilation

## Integration with vb6core and vb6runtime

`vb6codegen` integrates with the VB6 ecosystem's core libraries:

### vb6runtime

**Purpose**: Runtime execution infrastructure - value system, type conversions, standard library implementations.

**How vb6codegen uses it**:
- References VB6 type definitions (`VBType`, `Value`) for understanding semantics
- Generates code that links to vb6runtime for complex types (Variant, Object, Array with VB6 semantics)
- Maps VB6 standard library functions to vb6runtime implementations
- Understands VB6-specific type conversion rules

**Example**: When generating Rust code for a VB6 Variant:
```rust
// VB6: Dim x As Variant = "Hello"
// Generated Rust:
let x: vb6runtime::Value = vb6runtime::Value::String("Hello".to_string());
```

### vb6core

**Purpose**: Compiler infrastructure - IR definitions, IR building utilities, optimization framework.

**How vb6codegen uses it** (optional, primarily for vb6compile):
- Can accept IR (`vb6core::ir::Module`) as input for code generation
- Understands IR instruction set for generating target code
- `vb6compile` lowers AST → IR → optimizes IR → passes to vb6codegen

**Note**: `vb6convert` works directly from AST and doesn't use vb6core's IR. The IR pathway is primarily for `vb6compile`.

**Example flow**:
```
vb6compile:  AST → vb6core::IR → optimize → vb6codegen → Target Code
vb6convert:  AST → analyze → vb6codegen → Target Code
```

## Architecture

### Core Traits

```rust
/// Main trait for all code generation backends
pub trait CodegenBackend: Send + Sync {
    /// Name of this backend (e.g., "rust", "javascript", "llvm")
    fn name(&self) -> &str;
    
    /// Description of this backend
    fn description(&self) -> &str;
    
    /// Target-specific initialization
    fn initialize(&mut self, config: &CodegenConfig) -> Result<()>;
    
    /// Finalize code generation and return all generated code
    fn finalize(&mut self) -> Result<GeneratedCode>;
}

/// Trait for generating expressions in the target language
pub trait ExpressionGenerator: Send + Sync {
    /// Generate an addition expression
    fn gen_add(&mut self, left: &str, right: &str) -> Result<String>;
    
    /// Generate a subtraction expression
    fn gen_sub(&mut self, left: &str, right: &str) -> Result<String>;
    
    /// Generate a multiplication expression
    fn gen_mul(&mut self, left: &str, right: &str) -> Result<String>;
    
    /// Generate a division expression
    fn gen_div(&mut self, left: &str, right: &str) -> Result<String>;
    
    /// Generate a function call
    fn gen_call(&mut self, function: &str, args: &[String]) -> Result<String>;
    
    /// Generate a variable reference
    fn gen_var_ref(&mut self, name: &str) -> Result<String>;
    
    /// Generate a constant literal
    fn gen_literal(&mut self, value: &LiteralValue) -> Result<String>;
}

/// Trait for generating statements in the target language
pub trait StatementGenerator: Send + Sync {
    /// Generate a variable declaration
    fn gen_var_decl(&mut self, name: &str, typ: &TypeInfo, init: Option<&str>) -> Result<String>;
    
    /// Generate an assignment statement
    fn gen_assign(&mut self, target: &str, value: &str) -> Result<String>;
    
    /// Generate an if statement
    fn gen_if(&mut self, condition: &str, then_block: &[String], else_block: Option<&[String]>) -> Result<String>;
    
    /// Generate a while loop
    fn gen_while(&mut self, condition: &str, body: &[String]) -> Result<String>;
    
    /// Generate a for loop
    fn gen_for(&mut self, var: &str, start: &str, end: &str, body: &[String]) -> Result<String>;
    
    /// Generate a return statement
    fn gen_return(&mut self, value: Option<&str>) -> Result<String>;
}

/// Trait for generating functions and procedures
pub trait FunctionGenerator: Send + Sync {
    /// Generate a function/sub declaration
    fn gen_function(&mut self, func: &FunctionInfo) -> Result<String>;
    
    /// Generate a function parameter
    fn gen_parameter(&mut self, param: &ParameterInfo) -> Result<String>;
    
    /// Generate function body
    fn gen_function_body(&mut self, statements: &[String]) -> Result<String>;
}

/// Trait for generating module-level constructs
pub trait ModuleGenerator: Send + Sync {
    /// Generate a module/namespace
    fn gen_module(&mut self, module: &ModuleInfo) -> Result<String>;
    
    /// Generate imports/uses
    fn gen_imports(&mut self, imports: &[ImportInfo]) -> Result<String>;
    
    /// Generate global variable
    fn gen_global(&mut self, var: &GlobalVarInfo) -> Result<String>;
}

/// Trait for type system operations
pub trait TypeMapper: Send + Sync {
    /// Map a VB6 type to the target language type
    fn map_type(&self, vb6_type: &VB6Type) -> Result<TargetType>;
    
    /// Check if a type conversion requires runtime support
    fn needs_runtime_conversion(&self, from: &VB6Type, to: &VB6Type) -> bool;
    
    /// Generate code for explicit type conversion
    fn gen_type_cast(&mut self, expr: &str, from: &VB6Type, to: &VB6Type) -> Result<String>;
    
    /// Get the default/zero value for a type
    fn default_value(&self, typ: &VB6Type) -> String;
}

/// Trait for runtime library mappings
pub trait RuntimeMapper: Send + Sync {
    /// Map a VB6 standard library function to target equivalent
    fn map_stdlib_function(&self, vb6_func: &str) -> Result<RuntimeMapping>;
    
    /// Check if a function requires runtime support library
    fn needs_runtime_support(&self, vb6_func: &str) -> bool;
}
```

### Type System

`vb6codegen` uses the type system from `vb6runtime` as the source of truth for VB6 semantics:

```rust
// Re-export from vb6runtime
pub use vb6runtime::{VBType, Value};

/// Additional type information specific to code generation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VB6Type {
    // Core types (mirrors vb6runtime::VBType)
    Byte,
    Integer,
    Long,
    LongLong,
    Single,
    Double,
    Currency,
    Decimal,
    String,
    Boolean,
    Date,
    Variant,
    Object {
        class_name: Option<String>,
    },
    Array {
        element_type: Box<VB6Type>,
        dimensions: Option<Vec<ArrayDimension>>,
    },
    UserDefined(String),
}

/// Target language type representation
#[derive(Debug, Clone)]
pub struct TargetType {
    /// Type name in target language
    pub name: String,
    
    /// Whether this requires heap allocation
    pub is_heap_allocated: bool,
    
    /// Whether this is a reference type
    pub is_reference: bool,
    
    /// Additional type information (generics, etc.)
    pub type_params: Vec<String>,
}

/// Array dimension information
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayDimension {
    pub lower_bound: i32,
    pub upper_bound: i32,
}

/// Literal value types
#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i32),
    Long(i64),
    Single(f32),
    Double(f64),
    String(String),
    Boolean(bool),
    Null,
}
```

### Configuration

```rust
/// Configuration for code generation
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Target language/platform
    pub target: String,
    
    /// Indentation settings
    pub indent: IndentConfig,
    
    /// Naming convention
    pub naming: NamingConfig,
    
    /// Whether to generate comments
    pub generate_comments: bool,
    
    /// Whether to generate debug information
    pub generate_debug_info: bool,
    
    /// Custom type mappings
    pub type_overrides: HashMap<String, String>,
    
    /// Runtime library to use
    pub runtime_library: RuntimeLibrary,
}

#[derive(Debug, Clone)]
pub struct IndentConfig {
    /// Use spaces or tabs
    pub use_spaces: bool,
    
    /// Number of spaces/tabs per indent level
    pub indent_size: usize,
}

#[derive(Debug, Clone)]
pub struct NamingConfig {
    /// Naming convention for functions
    pub function_case: CaseStyle,
    
    /// Naming convention for variables
    pub variable_case: CaseStyle,
    
    /// Naming convention for types
    pub type_case: CaseStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum CaseStyle {
    /// snake_case
    Snake,
    /// camelCase
    Camel,
    /// PascalCase
    Pascal,
    /// kebab-case
    Kebab,
    /// SCREAMING_SNAKE_CASE
    ScreamingSnake,
}

#[derive(Debug, Clone)]
pub enum RuntimeLibrary {
    /// Use vb6runtime for runtime support
    VB6Runtime,
    
    /// Use target language's standard library only
    Native,
    
    /// Custom runtime library
    Custom(String),
}
```

### Generated Code

```rust
/// Result of code generation
#[derive(Debug, Clone)]
pub struct GeneratedCode {
    /// Generated files
    pub files: HashMap<PathBuf, String>,
    
    /// Entry point (if applicable)
    pub entry_point: Option<String>,
    
    /// Required dependencies
    pub dependencies: Vec<Dependency>,
    
    /// Additional build instructions
    pub build_info: Option<BuildInfo>,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub build_command: Option<String>,
    pub environment: HashMap<String, String>,
}
```

## Backend Implementations

### Rust Backend

Generates Rust code with the following mappings:

| VB6 Type | Rust Type |
|----------|-----------|
| Byte | `u8` |
| Integer | `i16` |
| Long | `i32` |
| LongLong | `i64` |
| Single | `f32` |
| Double | `f64` |
| String | `String` |
| Boolean | `bool` |
| Variant | `vb6runtime::Value` |
| Object | `vb6runtime::ObjectRef` or `Rc<dyn TraitName>` |
| Array | `vb6runtime::Array<T>` (preserves VB6 semantics) or `Vec<T>` (simple cases) |
| Currency | `vb6runtime::Currency` (fixed-point decimal) |
| Date | `vb6runtime::Date` (OLE date format) |

**Standard Library Mappings:**

VB6 standard library functions map to `vb6runtime` implementations to preserve exact VB6 semantics:

- `Left$(s, n)` → `vb6runtime::string::left(&s, n)`
- `Right$(s, n)` → `vb6runtime::string::right(&s, n)`
- `Mid$(s, start, len)` → `vb6runtime::string::mid(&s, start, len)`
- `MsgBox(msg, ...)` → `vb6runtime::ui::msgbox(msg, ...)`
- `InputBox(prompt, ...)` → `vb6runtime::ui::inputbox(prompt, ...)`
- `CInt(x)` → `vb6runtime::convert::to_integer(&x)`
- `CStr(x)` → `vb6runtime::convert::to_string(&x)`
- `IsNumeric(x)` → `vb6runtime::checks::is_numeric(&x)`

**Why vb6runtime?**
VB6 has subtle type coercion rules, date handling quirks, and string semantics that differ from Rust/JavaScript. Using vb6runtime ensures generated code behaves identically to VB6.

### JavaScript Backend

Generates JavaScript (ES6+) code:

| VB6 Type | JavaScript Type |
|----------|-----------------|
| All numeric | `number` |
| String | `string` |
| Boolean | `boolean` |
| Variant | `any` (TypeScript) |
| Object | `object` |
| Array | `Array<T>` |

**Standard Library Mappings:**
- `Left$()` → `str.substring(0, n)`
- `Right$()` → `str.substring(str.length - n)`
- `Mid$()` → `str.substring(start, end)`
- `MsgBox` → `alert()` or custom implementation
- `InputBox` → `prompt()` or custom implementation

### TypeScript Backend

Extends JavaScript backend with type annotations:

```typescript
// VB6: Dim x As Integer = 42
let x: number = 42;

// VB6: Function Add(a As Integer, b As Integer) As Integer
function add(a: number, b: number): number {
    return a + b;
}
```

### LLVM Backend (feature: llvm)

Generates LLVM IR for compilation to native code:

| VB6 Type | LLVM Type |
|----------|-----------|
| Byte | `i8` |
| Integer | `i16` |
| Long | `i32` |
| LongLong | `i64` |
| Single | `float` |
| Double | `double` |
| String | `i8*` (C string) or struct |
| Variant | struct with tag and union |

## Module Structure

```
vb6codegen/
├── Cargo.toml
├── README.md
├── docs/
│   ├── DESIGN.md           # This file
│   └── BACKENDS.md         # Backend-specific documentation
├── src/
│   ├── lib.rs              # Main library interface
│   ├── traits.rs           # Core trait definitions
│   ├── types.rs            # Type system
│   ├── config.rs           # Configuration types
│   ├── error.rs            # Error types
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── formatting.rs   # Code formatting utilities
│   │   ├── naming.rs       # Naming convention utilities
│   │   └── indentation.rs  # Indentation helpers
│   ├── backend/
│   │   ├── mod.rs          # Backend registry
│   │   ├── rust/           # Rust code generator
│   │   │   ├── mod.rs
│   │   │   ├── types.rs
│   │   │   ├── expressions.rs
│   │   │   ├── statements.rs
│   │   │   ├── functions.rs
│   │   │   └── runtime.rs
│   │   ├── javascript/     # JavaScript code generator
│   │   │   ├── mod.rs
│   │   │   ├── types.rs
│   │   │   ├── expressions.rs
│   │   │   ├── statements.rs
│   │   │   └── runtime.rs
│   │   ├── typescript/     # TypeScript code generator
│   │   │   └── ...
│   │   └── llvm/           # LLVM IR generator (optional)
│   │       └── ...
│   └── runtime/
│       ├── mod.rs
│       └── mappings.rs     # Runtime function mappings
└── tests/
    ├── rust_codegen_tests.rs
    ├── javascript_codegen_tests.rs
    └── type_mapping_tests.rs
```

## Feature Gates

The following Cargo features control which backends are compiled:

- `rust-backend` - Rust code generation (default)
- `javascript-backend` - JavaScript code generation
- `typescript-backend` - TypeScript code generation (implies `javascript-backend`)
- `llvm-backend` - LLVM IR generation (requires `inkwell` dependency)
- `all-backends` - Enable all backends

## Usage Examples

### For vb6convert

```rust
use vb6codegen::{CodegenBackend, RustBackend, CodegenConfig};
use vb6runtime::VBType;

// Create a Rust backend
let mut backend = RustBackend::new();

// Configure code generation
let config = CodegenConfig {
    target: "rust".to_string(),
    generate_comments: true,
    runtime_library: RuntimeLibrary::VB6Runtime, // Link to vb6runtime
    naming: NamingConfig {
        function_case: CaseStyle::Snake,
        variable_case: CaseStyle::Snake,
        type_case: CaseStyle::Pascal,
    },
    ..Default::default()
};

backend.initialize(&config)?;

// Generate code for VB6 constructs (from AST)
// Backend automatically uses vb6runtime types for Variant, complex conversions, etc.
// ... conversion logic ...

let generated = backend.finalize()?;
// Generated code includes: use vb6runtime::prelude::*;
```

### For vb6compile

```rust
use vb6codegen::{CodegenBackend, RustBackend, LLVMBackend, CodegenConfig};
use vb6core::ir::Module as IRModule;

// vb6compile works through IR
let ir_module = compile_to_ir(ast)?; // From vb6core

// Create backend based on compiler options
let mut backend: Box<dyn CodegenBackend> = match opt_level {
    OptLevel::O3 => Box::new(LLVMBackend::new()),
    _ => Box::new(RustBackend::new()),
};

let config = CodegenConfig {
    generate_debug_info: debug_mode,
    runtime_library: RuntimeLibrary::VB6Runtime, // Link to vb6runtime
    ..Default::default()
};

backend.initialize(&config)?;

// Generate code from vb6core IR
backend.generate_from_ir(&ir_module)?;

let generated = backend.finalize()?;
// Generated code links to vb6runtime for runtime support
```

## Integration Points

### vb6convert Integration

`vb6convert` uses `vb6codegen` to generate code after AST analysis:

1. Parse VB6 project → AST
2. Analyze and validate AST
3. Select target backend from `vb6codegen`
4. Walk AST and call backend methods to generate code
5. Finalize and write output files

### vb6compile Integration

`vb6compile` uses `vb6codegen` as the final code generation step:

1. Parse VB6 project → AST
2. Semantic analysis → Typed AST
3. Lower to IR
4. Optimize IR
5. Select backend from `vb6codegen`
6. Generate code from optimized IR
7. Link/package as needed

## Dependencies

### Core Dependencies
- `vb6runtime` - VB6 type system and runtime semantics (required)
- `vb6core` - IR definitions (optional, for IR-based generation)
- `thiserror` - Error handling
- `heck` - Case conversion utilities
- `indoc` - Indented string literals

### Optional Dependencies
- `serde` - Serialization support
- `inkwell` - LLVM bindings (for llvm-backend feature)

## Testing Strategy

### Unit Tests
- Type mapping correctness
- Expression generation
- Statement generation
- Name mangling and formatting

### Integration Tests
- Complete function generation
- Module generation
- Cross-backend consistency

### Golden Tests
- Compare generated code against known-good outputs
- Ensure backward compatibility

## Future Enhancements

- [ ] C backend for maximum portability
- [ ] C++ backend
- [ ] Python backend
- [ ] WebAssembly backend
- [ ] Custom backend plugin system
- [ ] Optimization hints for backends
- [ ] Source map generation

## License

MIT
