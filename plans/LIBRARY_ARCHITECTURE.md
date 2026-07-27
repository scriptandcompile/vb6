# VB6 Project Library Architecture

This document describes the complete architecture and dependencies between all libraries in the VB6 project.

## Overview

The VB6 project is organized as a workspace with multiple specialized libraries, each with distinct responsibilities. This modular design promotes code reuse, clear separation of concerns, and allows different tools to share common infrastructure.

## Library Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│                   VB6 Project Workspace                 │
└─────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼────┐      ┌──────▼──────┐     ┌─────▼──────┐
   │ vb6parse│      │ vb6semantic │     │  vb6core   │
   │ (base)  │      │  (analysis) │     │   (IR)     │
   └────┬────┘      └──────┬──────┘     └─────┬──────┘
        │                  │                  │
        │         ┌────────▼────────┐         │
        │         │  vb6runtime     │         │
        │         │ (type system,   │         │
        │         │  VB6 semantics) │         │
        │         └────────┬────────┘         │
        │                  │                  │
        └──────────────┬───┴───┬──────────────┘
                       │       │
                 ┌─────▼───────▼─────┐
                 │    vb6codegen     │
                 │ (code generation) │
                 └─────┬───────┬─────┘
                       │       │
              ┌────────┴───┐   └────────┐
              │            │            │
         ┌────▼─────┐  ┌────▼─────┐  ┌───▼────────┐
         │vb6convert│  │vb6compile│  │vb6interpret│
         │  (tool)  │  │  (tool)  │  │   (tool)   │
         └──────────┘  └──────────┘  └────────────┘
```

## Library Descriptions

### vb6parse (Foundation)

**Purpose**: Parse VB6 source code into an Abstract Syntax Tree (AST).

**Responsibilities**:
- Tokenization and lexical analysis
- Parsing VB6 syntax (modules, classes, forms, projects)
- AST generation
- Source location tracking

**Dependencies**: None (foundation library)

**Used by**: All other libraries and tools

---

### vb6runtime (Runtime Semantics)

**Purpose**: Provide VB6 type system, runtime values, and standard library implementations.

**Responsibilities**:
- VB6 type definitions (`VBType`, `Value`)
- Type conversion rules (VB6-exact semantics)
- Standard library functions (string manipulation, math, conversions)
- Complex VB6 types (Variant, Currency, Date, Arrays with custom bounds)
- Runtime context and state management

**Dependencies**: 
- `vb6parse` (for understanding parsed structures)

**Used by**:
- `vb6semantic` - For type checking
- `vb6codegen` - For type mappings and runtime function calls
- `vb6interpret` - For direct execution
- `vb6compile` - Generated code links to vb6runtime

**Key Types**:
```rust
enum Value {
    Empty, Null, Byte(u8), Integer(i16), Long(i32),
    Single(f32), Double(f64), Currency(i64), Date(f64),
    String(String), Boolean(bool), Variant(Box<Value>),
    Object(ObjectRef), Array(Array), UserDefined(Map), Error(i32)
}

enum VBType {
    Byte, Boolean, Integer, Long, Single, Double,
    Currency, Date, String, Variant, Object, Array, UserDefined
}
```

---

### vb6semantic (Semantic Analysis)

**Purpose**: Perform semantic analysis and type checking on VB6 AST.

**Responsibilities**:
- Symbol table construction
- Scope management
- Type checking using `vb6runtime::VBType`
- Name resolution
- Semantic error detection

**Dependencies**:
- `vb6parse` - For AST types
- `vb6runtime` - For type system

**Used by**:
- `vb6compile` - Before compilation
- `vb6convert` - For analysis (optional)

---

### vb6core (Compiler Infrastructure)

**Purpose**: Intermediate representation (IR) and optimization framework for compilation.

**Responsibilities**:
- IR definition (simplified, strongly-typed representation)
- IR builder utilities (AST → IR lowering)
- Optimization passes (constant folding, dead code elimination, inlining, etc.)
- IR analysis and transformation

**Dependencies**:
- `vb6parse` - For AST types
- `vb6runtime` - For type system in IR

**Used by**:
- `vb6compile` - For optimization and compilation
- `vb6interpret` - For direct IR execution
- `vb6codegen` - Can generate from IR (optional)

**Key IR Types**:
```rust
enum Instruction {
    Label(String), Jump(String), JumpIf(Value, String),
    Call(String, Vec<Value>), Return(Option<Value>),
    DeclareVar(String, VBType), Assign(String, Value),
    Add(Value, Value), Sub(Value, Value), Mul(Value, Value),
    Div(Value, Value), Eq(Value, Value), Lt(Value, Value)
}

struct IRModule {
    name: String,
    functions: Vec<IRFunction>,
    globals: Vec<GlobalVar>,
}
```

**Note**: `vb6convert` does NOT use `vb6core` - it works directly from AST.

---

### vb6codegen (Code Generation)

**Purpose**: Shared code generation backends for multiple target languages.

**Responsibilities**:
- Backend trait definitions (`CodegenBackend`, `ExpressionGenerator`, etc.)
- Type mappings from VB6 to target languages
- Code generators:
  - Rust backend
  - JavaScript backend
  - TypeScript backend
  - LLVM backend (optional)
- Code formatting and naming conventions
- Runtime function mappings
- Links generated code to `vb6runtime`

**Dependencies**:
- `vb6runtime` - For type system and runtime function mappings
- `vb6core` - For IR types (optional, for IR-based generation)
- `heck` - For naming conventions
- `inkwell` - For LLVM backend (optional)

**Used by**:
- `vb6convert` - For source-to-source conversion
- `vb6compile` - For final code generation

**Key Features**:
- Feature-gated backends (rust-backend, javascript-backend, typescript-backend, llvm-backend)
- Pluggable architecture for new backends
- Consistent code generation across both convert and compile tools

**Example Generated Rust Code**:
```rust
// Generated from VB6
use vb6runtime::prelude::*;

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn demonstrate_variant() {
    let x: vb6runtime::Value = vb6runtime::Value::String("Hello".to_string());
    let y: vb6runtime::Value = vb6runtime::Value::Integer(42);
}
```

---

## Tool Descriptions

### vb6convert (Conversion Tool)

**Purpose**: Convert VB6 projects to modern languages and frameworks.

**Workflow**:
```
VB6 Source → vb6parse → AST → Analyze → vb6codegen → Target Code
```

**Dependencies**:
- `vb6parse` - Parse VB6 source
- `vb6runtime` - Understand VB6 types and semantics
- `vb6codegen` - Generate target code

**Does NOT use**: `vb6core` (no IR needed for conversion)

**Features**:
- Multiple target languages (Rust, JS, TS, Dart)
- UI framework support (Tauri, Svelte, React, Vue, Flutter)
- Project analysis and conversion feasibility assessment

---

### vb6compile (Compiler)

**Purpose**: Compile VB6 to native executables or optimized target code.

**Workflow**:
```
VB6 Source → vb6parse → AST → vb6semantic → vb6core IR → 
Optimize → vb6codegen → Link with vb6runtime → Executable
```

**Dependencies**:
- `vb6parse` - Parse VB6 source
- `vb6semantic` - Type checking and analysis
- `vb6core` - IR and optimization
- `vb6runtime` - Type system; generated code links to it
- `vb6codegen` - Final code generation

**Features**:
- Multiple optimization levels (O0-O3, Os)
- Multiple backends (Rust, LLVM, JavaScript)
- Cross-compilation support
- Incremental compilation

---

### vb6interpret (Interpreter)

**Purpose**: Direct execution of VB6 code without compilation.

**Workflow**:
```
VB6 Source → vb6parse → AST → vb6core IR → Execute with vb6runtime
```

**Dependencies**:
- `vb6parse` - Parse VB6 source
- `vb6core` - IR for execution
- `vb6runtime` - Runtime values and execution context

---

## Data Flow Examples

### Example 1: Converting VB6 to Rust

```
┌──────────────┐
│  example.bas │  ' Dim x As Integer = 42
└──────┬───────┘  ' Function Add(a, b) ...
       │
       ▼
  [vb6parse]
       │
       ▼
    ┌─────┐
    │ AST │
    └──┬──┘
       │
       ▼
  [vb6convert analyzer]
  - Uses vb6runtime::VBType
  - Determines target (Rust)
       │
       ▼
  [vb6codegen::RustBackend]
  - Maps Integer → i32
  - Generates Rust code
  - Links to vb6runtime for complex types
       │
       ▼
┌────────────────┐
│  example.rs    │  let x: i32 = 42;
└────────────────┘  fn add(a: i32, b: i32) -> i32 { ... }
```

### Example 2: Compiling VB6 to Optimized Native Code

```
┌──────────────┐
│  program.vbp │
└──────┬───────┘
       │
       ▼
  [vb6parse]
       │
       ▼
    ┌─────┐
    │ AST │
    └──┬──┘
       │
       ▼
  [vb6semantic]
  - Type check with vb6runtime::VBType
       │
       ▼
  [vb6core: AST → IR]
       │
       ▼
    ┌────┐
    │ IR │  (vb6core::ir::Module)
    └──┬─┘
       │
       ▼
  [vb6core: Optimize]
  - Constant folding
  - Inlining
  - Dead code elimination
       │
       ▼
┌─────────────┐
│Optimized IR │
└──────┬──────┘
       │
       ▼
  [vb6codegen::LLVMBackend]
  - Generate LLVM IR
  - Link to vb6runtime
       │
       ▼
┌──────────────┐
│  executable  │ (with vb6runtime linked in)
└──────────────┘
```

## Key Design Principles

### 1. Separation of Concerns
- **Parsing** (vb6parse) is separate from **semantics** (vb6semantic, vb6runtime)
- **IR/optimization** (vb6core) is separate from **code generation** (vb6codegen)
- **Conversion** (vb6convert) and **compilation** (vb6compile) share infrastructure but have different workflows

### 2. Shared Infrastructure
- `vb6codegen` eliminates duplication between convert and compile tools
- Both tools generate code that uses `vb6runtime` for consistent VB6 semantics
- Type system defined once in `vb6runtime`, used everywhere

### 3. Modularity
- Each library can be used independently
- Feature gates allow minimal compilation
- Clear dependency hierarchy prevents cycles

### 4. VB6 Correctness
- `vb6runtime` is the single source of truth for VB6 semantics
- Type conversions, Variant behavior, array bounds, etc. all use vb6runtime
- Generated code links to vb6runtime to ensure exact VB6 behavior

## When to Use Each Library

| Library | Use When... |
|---------|-------------|
| `vb6parse` | You need to parse VB6 source code |
| `vb6runtime` | You need VB6 types, values, or standard library |
| `vb6semantic` | You need to type check or analyze VB6 code |
| `vb6core` | You need IR for optimization or interpretation |
| `vb6codegen` | You need to generate code in a target language |
| `vb6convert` | You want to convert VB6 to modern languages |
| `vb6compile` | You want to compile VB6 to optimized executables |
| `vb6interpret` | You want to directly execute VB6 code |

## Dependency Summary

```
vb6parse: (none)
vb6runtime: vb6parse
vb6semantic: vb6parse, vb6runtime
vb6core: vb6parse, vb6runtime
vb6codegen: vb6runtime, vb6core (optional)
vb6convert: vb6parse, vb6runtime, vb6codegen
vb6compile: vb6parse, vb6semantic, vb6core, vb6runtime, vb6codegen
vb6interpret: vb6parse, vb6core, vb6runtime
```

## Future Considerations

- **Plugin System**: Allow custom backends in vb6codegen
- **Language Server**: Use vb6parse + vb6semantic for IDE integration
- **Debugger**: Use vb6interpret for debugging support
- **REPL**: Use vb6interpret for interactive VB6 execution
- **Additional Backends**: Python, C++, Go, WebAssembly

---

*Last Updated: February 18, 2026*
