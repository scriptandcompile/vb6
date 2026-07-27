# vb6core

Core compiler infrastructure and intermediate representation for VB6 compiler and interpreter.

## Overview

`vb6core` provides the foundational compiler infrastructure shared between `vb6interpret` and `vb6compile`. It defines the intermediate representation (IR), IR building utilities, and optimization passes. For runtime execution (values, types, standard library), see `vb6runtime`.

## Architecture

```
┌─────────────────┐         ┌─────────────────┐
│  vb6interpret  │         │   vb6compile   │
└────────┬────────┘         └────────┬────────┘
         │                           │
         ├───────────┬───────────────┤
         │           │               │
    ┌────▼────┐  ┌───▼──────┐  ┌────▼────────┐
    │vb6runtime│  │ vb6core  │  │ vb6semantic │
    └────┬────┘  └───┬──────┘  └────┬────────┘
         │           │               │
         └───────────┴───────────────┘
                     │
              ┌──────▼──────┐
              │   vb6parse  │
              └─────────────┘
```

**Separation of Concerns**:
- **vb6core**: IR, compilation infrastructure, optimizations
- **vb6runtime**: Value system, type conversions, standard library, runtime context
- **vb6semantic**: Symbol tables, type checking, semantic analysis

## Core Components

### 1. Intermediate Representation (IR)

A simplified, strongly-typed IR that represents VB6 programs:

```rust
pub enum Instruction {
    // Control flow
    Label(String),
    Jump(String),
    JumpIf(Value, String),
    Call(String, Vec<Value>),
    Return(Option<Value>),
    
    // Variable operations
    DeclareVar(String, VBType),
    Assign(String, Value),
    Load(String),
    
    // Arithmetic
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    
    // Comparisons
    Eq(Value, Value),
    Lt(Value, Value),
**IR Design Goals**:
- Simple enough for interpreter to execute directly
- Rich enough for compiler optimizations
- Preserves VB6 semantics
- Easy to analyze and transform

### 2. IR Builder

Utilities for constructing IR from AST:rations
}
```

**IR Design Goals**:
- Simple enough for interpreter to execute directly
- Rich enough for compiler optimizations
- Preserves VB6 semantics
- Easy to analyze and transform

### 2. IR Builder

Utilities for constructing IR from AST:

```rust
pub struct IRBuilder {
    /// Current function being built
    current_function: Option<String>,
    
    /// Instructions accumulated
    instructions: Vec<Instruction>,
    
    /// Label counter
    label_counter: usize,
}

impl IRBuilder {
    pub fn new() -> Self;
    
    /// Build IR from parsed AST
    pub fn build_from_ast(ast: &AST) -> Result<IRModule>;
    
    /// Add instruction
    pub fn add_instruction(&mut self, instr: Instruction);
    
    /// Generate unique label
    pub fn new_label(&mut self) -> Label;
    
    /// Start new function
    pub fn begin_function(&mut self, name: String);
    
    /// End current function
    pub fn end_function(&mut self) -> IRFunction;
}
```

### 3. IR Optimizer

Optimization passes for IR:

```rust
pub trait OptimizationPass {
    fn optimize(&mut self, module: &mut IRModule) -> Result<bool>;
}

// Available optimization passes
pub struct ConstantFolding;
pub struct DeadCodeElimination;
pub struct CommonSubexpressionElimination;
pub struct InlineFunctions;
```

### 4. IR Module

Container for IR code:

```rust
pub struct IRModule {
    pub name: String,
    pub functions: Vec<IRFunction>,
    pub globals: Vec<GlobalVar>,
}

pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<VBType>,
    pub instructions: Vec<Instruction>,
    pub locals: Vec<LocalVar>,
}
```

## Features

### IR Generation
- Convert AST to simplified IR
- Preserve VB6 semantics
- Handle control flow (If, Select, For, While, GoTo)
- Function calls and returns

### IR Optimization
- Constant folding
- Dead code elimination
- Common subexpression elimination
- Function inlining
- Jump optimization

### IR Analysis
- Control flow analysis
- Data flow analysis
- Variable liveness
- Type inference

## Dependencies

- **vb6parse**: For AST input
- **vb6semantic**: For type information and symbol tables
- **vb6runtime**: For type definitions (VBType)
- **thiserror**: Error handling
- **serde**: IR serialization

## Usage

### Building IR from AST

```rust
use vb6core::ir::IRBuilder;

let ast = parse_vb6_code(source)?;
let ir_module = IRBuilder::build_from_ast(&ast)?;
```

### Optimizing IR

```rust
use vb6core::ir::optimizer::*;

let mut module = build_ir()?;

// Apply optimization passes
let mut changed = true;
while changed {
    changed = false;
    changed |= ConstantFolding.optimize(&mut module)?;
    changed |= DeadCodeElimination.optimize(&mut module)?;
}
```

### Analyzing IR

```rust
use vb6core::ir::analysis::*;

// Control flow analysis
let cfg = ControlFlowGraph::from_function(&function);
let dominators = cfg.compute_dominators();

// Data flow analysis
let live_vars = compute_liveness(&function);
```

## Design Principles

1. **Language Agnostic**: IR should support multiple frontends/backends
2. **Simplicity**: Easy to understand and manipulate
3. **Analyzability**: Support static analysis and optimization
4. **VB6 Semantics**: Preserve VB6 behavior through compilation
5. **Testability**: IR transformations are testable

## Implementation Status

- [ ] IR instruction definitions
- [ ] IR builder from AST
- [ ] IR serialization/deserialization
- [ ] Constant folding optimization
- [ ] Dead code elimination
- [ ] Control flow graph construction
- [ ] Data flow analysis
- [ ] SSA form conversion
- [ ] Type inference on IR
- [ ] IR validation

## Dependencies

- `vb6parse`: For AST structures
- `vb6semantic`: For type information and symbol tables
- `vb6runtime`: For VBType definitions
- `serde`: For IR serialization
- `petgraph`: For control flow graph analysis
- `thiserror`: Error handling

## Testing

```bash
# Run unit tests
cargo test -p vb6core

# Run with all features
cargo test -p vb6core --all-features

# Run benchmarks
cargo bench -p vb6core
```

## Performance Goals

- IR generation: Fast conversion from AST
- Optimization passes: Linear time in most cases
- Analysis: Efficient for large functions
- Memory: Minimal IR size overhead

## Relationship with Other Crates

- **vb6parse** → **vb6semantic** → **vb6core** (AST → IR)
- **vb6core** → **vb6runtime** (IR references runtime types)
- **vb6interpret** uses IR directly for execution
- **vb6compile** uses IR for code generation

For runtime execution concerns (values, types, standard library), see [vb6runtime](../vb6runtime/).

## Future Enhancements

- [ ] SSA form IR variant
- [ ] Advanced optimization passes (loop optimizations, etc.)
- [ ] IR-level debug information
- [ ] Incremental compilation support
- [ ] Cross-module optimization
- [ ] IR-level profiling hooks

## License

MIT License - see LICENSE file for details.
