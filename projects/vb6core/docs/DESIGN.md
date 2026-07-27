# vb6core Design Document

## Overview

`vb6core` is the foundational library that provides shared functionality between the VB6 interpreter (`vb6interpret`) and compiler (`vb6compile`). It defines the intermediate representation, runtime value system, type conversions, standard library, and execution environment.

## Goals

1. **Code Reuse**: Maximize shared code between interpreter and compiler
2. **VB6 Compatibility**: Implement exact VB6 semantics, including edge cases
3. **Performance**: Efficient runtime for both interpreted and compiled code
4. **Extensibility**: Easy to add new features and optimizations
5. **Testing**: Comprehensive test coverage with validation against VB6

## Architecture

### Module Structure

```
vb6core/
├── src/
│   ├── lib.rs                  # Public API
│   ├── value.rs                # Value type system
│   ├── types.rs                # VB6 type definitions
│   ├── conversion.rs           # Type conversion rules
│   ├── ir/
│   │   ├── mod.rs              # IR module
│   │   ├── instruction.rs      # IR instructions
│   │   ├── builder.rs          # IR builder utilities
│   │   └── optimizer.rs        # IR optimization passes
│   ├── runtime/
│   │   ├── mod.rs
│   │   ├── context.rs          # Runtime execution context
│   │   ├── stack.rs            # Call stack management
│   │   ├── error.rs            # Error handling
│   │   └── control_flow.rs     # On Error, GoTo, etc.
│   ├── stdlib/
│   │   ├── mod.rs
│   │   ├── string.rs           # String functions
│   │   ├── math.rs             # Math functions
│   │   ├── date.rs             # Date/Time functions
│   │   ├── conversion.rs       # CInt, CLng, etc.
│   │   ├── format.rs           # Format functions
│   │   ├── file.rs             # File I/O
│   │   ├── array.rs            # Array functions
│   │   └── interaction.rs      # MsgBox, InputBox, etc.
│   ├── object/
│   │   ├── mod.rs
│   │   ├── form.rs             # Form runtime
│   │   ├── control.rs          # Control base
│   │   ├── collection.rs       # Collections
│   │   └── com.rs              # COM interop (future)
│   ├── array.rs                # Array implementation
│   └── variant.rs              # Variant type implementation
├── tests/
│   ├── value_tests.rs
│   ├── conversion_tests.rs
│   ├── stdlib_tests.rs
│   └── integration_tests.rs
└── benches/
    ├── value_ops.rs
    ├── conversions.rs
    └── stdlib.rs
```

## Core Components

### 1. Value System (`value.rs`)

The `Value` enum represents all VB6 runtime values:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Empty (uninitialized Variant)
    Empty,
    
    /// Null (database null)
    Null,
    
    /// 8-bit unsigned integer (0-255)
    Byte(u8),
    
    /// 16-bit signed integer (-32,768 to 32,767)
    Integer(i16),
    
    /// 32-bit signed integer
    Long(i32),
    
    /// Single-precision float
    Single(f32),
    
    /// Double-precision float
    Double(f64),
    
    /// Currency (64-bit int, 4 decimal places)
    Currency(i64),
    
    /// Date/Time (OLE automation date)
    Date(f64),
    
    /// Variable-length string
    String(String),
    
    /// Boolean (True = -1, False = 0)
    Boolean(bool),
    
    /// Variant container
    Variant(Box<Value>),
    
    /// Object reference
    Object(ObjectRef),
    
    /// Array (multi-dimensional)
    Array(Array),
    
    /// User-defined type
    UserDefined {
        type_name: String,
        fields: HashMap<String, Value>,
    },
    
    /// Error value
    Error(i32),
}

impl Value {
    /// Get the type of this value
    pub fn value_type(&self) -> VBType;
    
    /// Check if value is numeric
    pub fn is_numeric(&self) -> bool;
    
    /// Check if value is empty or null
    pub fn is_empty_or_null(&self) -> bool;
    
    /// Convert to boolean (VB6 rules)
    pub fn to_boolean(&self) -> Result<bool>;
    
    /// Convert to string (VB6 rules)
    pub fn to_string(&self) -> Result<String>;
    
    /// Convert to number
    pub fn to_number<T: FromValue>(&self) -> Result<T>;
    
    /// Compare values (VB6 comparison rules)
    pub fn compare(&self, other: &Value) -> Result<Ordering>;
}
```

**Key Design Decisions**:
- **Boolean Representation**: VB6 uses -1 for True, 0 for False
- **Currency As Integer**: Store as i64 with 4 implied decimal places
- **Date As Float**: OLE automation date (days since 1899-12-30)
- **Variant Boxing**: Use Box to avoid recursive type definition
- **Object References**: Keep lightweight references, objects stored separately

### 2. Type System (`types.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VBType {
    Byte,
    Boolean,
    Integer,
    Long,
    Single,
    Double,
    Currency,
    Date,
    
    /// String with optional fixed length
    String(Option<usize>),
    
    /// Variant
    Variant,
    
    /// Object with optional class name
    Object(Option<String>),
    
    /// Array with element type and bounds
    Array {
        element_type: Box<VBType>,
        dimensions: usize,
        bounds: Vec<(i32, i32)>,  // (lower, upper) for each dimension
    },
    
    /// User-defined type
    UserDefined(String),
    
    /// Function type (for function pointers/AddressOf)
    Function {
        parameters: Vec<VBType>,
        return_type: Box<VBType>,
    },
}

impl VBType {
    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool;
    
    /// Check if type can be assigned from another
    pub fn can_assign_from(&self, other: &VBType) -> bool;
    
    /// Get default value for this type
    pub fn default_value(&self) -> Value;
    
    /// Size in bytes (for fixed-size allocations)
    pub fn size_bytes(&self) -> Option<usize>;
}
```

### 3. Type Conversion (`conversion.rs`)

VB6 has complex conversion rules that must be implemented exactly:

```rust
/// Convert a value to a target type
pub fn convert(value: &Value, target: &VBType) -> Result<Value> {
    match (value, target) {
        // Numeric widening conversions
        (Value::Integer(i), VBType::Long) => Ok(Value::Long(*i as i32)),
        (Value::Integer(i), VBType::Double) => Ok(Value::Double(*i as f64)),
        
        // String to number conversions
        (Value::String(s), VBType::Integer) => parse_integer(s),
        (Value::String(s), VBType::Double) => parse_double(s),
        
        // Boolean conversions (True = -1)
        (Value::Boolean(true), VBType::Integer) => Ok(Value::Integer(-1)),
        (Value::Boolean(false), VBType::Integer) => Ok(Value::Integer(0)),
        
        // Variant unwrapping
        (Value::Variant(v), target) => convert(v, target),
        
        // ... more conversion rules
    }
}

/// Numeric type promotion for binary operations
pub fn promote_numeric(left: &Value, right: &Value) -> Result<(Value, Value)>;

/// Implicit conversions (VB6 automatic conversions)
pub fn implicit_convert(value: &Value, target: &VBType) -> Result<Value>;

/// Explicit conversions (CInt, CLng, etc.)
pub fn explicit_convert(value: &Value, target: &VBType) -> Result<Value>;
```

**Conversion Rules**:
1. **Widening**: Byte → Integer → Long → Single → Double → Variant
2. **String Parsing**: Recognize numeric strings, dates
3. **Boolean**: -1 for True, 0 for False in numeric context
4. **Null Propagation**: Null propagates through most operations
5. **Error Values**: Error values propagate through expressions

### 4. Intermediate Representation (`ir/`)

A simplified IR that both interpreter and compiler can work with:

```rust
#[derive(Debug, Clone)]
pub enum Instruction {
    // Labels and jumps
    Label(Label),
    Jump(Label),
    JumpIfFalse(Value, Label),
    JumpIfTrue(Value, Label),
    
    // Function calls
    Call {
        function: String,
        arguments: Vec<Value>,
        result: Option<String>,  // Destination variable
    },
    
    Return(Option<Value>),
    
    // Variable operations
    DeclareLocal(String, VBType),
    LoadLocal(String),
    StoreLocal(String, Value),
    LoadGlobal(String),
    StoreGlobal(String, Value),
    
    // Arithmetic
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    IntDiv(Value, Value),
    Mod(Value, Value),
    Pow(Value, Value),
    Neg(Value),
    
    // String operations
    Concat(Value, Value),
    
    // Comparison
    Eq(Value, Value),
    Ne(Value, Value),
    Lt(Value, Value),
    Le(Value, Value),
    Gt(Value, Value),
    Ge(Value, Value),
    
    // Logical
    And(Value, Value),
    Or(Value, Value),
    Not(Value),
    Xor(Value, Value),
    
    // Array operations
    ArrayAccess(String, Vec<Value>),  // Name, indices
    ArrayStore(String, Vec<Value>, Value),
    ReDim {
        array: String,
        bounds: Vec<(Value, Value)>,
        preserve: bool,
    },
    
    // Object operations
    PropertyGet(Value, String),
    PropertySet(Value, String, Value),
    MethodCall(Value, String, Vec<Value>),
    
    // Error handling
    OnErrorResumeNext,
    OnErrorGoTo(Label),
    OnErrorGoToZero,
    Resume,
    ResumeNext,
    ResumeLabel(Label),
    
    // Debug
    DebugPrint(Vec<Value>),
    Stop,
    
    // Metadata (for debugging)
    SourceLine(String, usize),
}

pub type Label = String;

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<(String, VBType)>,
    pub return_type: Option<VBType>,
    pub locals: Vec<(String, VBType)>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct IRModule {
    pub name: String,
    pub functions: Vec<IRFunction>,
    pub globals: Vec<(String, VBType, Option<Value>)>,
}
```

**IR Design Principles**:
- **High-Level**: Keep VB6 semantics visible (not too low-level)
- **Typed**: Every operation knows its types
- **SSA-Like**: Use value semantics where possible
- **Optimizable**: Easy to analyze and transform
- **Debuggable**: Preserve source line information

### 5. Standard Library (`stdlib/`)

Implementation of all VB6 built-in functions.

#### String Functions (`stdlib/string.rs`)

```rust
pub fn left(s: &str, n: i32) -> Result<String>;
pub fn right(s: &str, n: i32) -> Result<String>;
pub fn mid(s: &str, start: i32, length: Option<i32>) -> Result<String>;
pub fn len(s: &str) -> i32;
pub fn instr(start: Option<i32>, s1: &str, s2: &str, compare: Option<i32>) -> i32;
pub fn lcase(s: &str) -> String;
pub fn ucase(s: &str) -> String;
pub fn trim(s: &str) -> String;
pub fn ltrim(s: &str) -> String;
pub fn rtrim(s: &str) -> String;
pub fn string_fn(n: i32, character: &str) -> Result<String>;
pub fn space(n: i32) -> String;
pub fn str(n: f64) -> String;
pub fn val(s: &str) -> f64;
pub fn asc(s: &str) -> Result<i32>;
pub fn chr(code: i32) -> Result<String>;
// ... 40+ more string functions
```

#### Math Functions (`stdlib/math.rs`)

```rust
pub fn abs<T: Numeric>(n: T) -> T;
pub fn sgn<T: Numeric>(n: T) -> i32;
pub fn sqr(n: f64) -> f64;
pub fn sin(n: f64) -> f64;
pub fn cos(n: f64) -> f64;
pub fn tan(n: f64) -> f64;
pub fn atn(n: f64) -> f64;
pub fn exp(n: f64) -> f64;
pub fn log(n: f64) -> Result<f64>;
pub fn fix(n: f64) -> i32;
pub fn int(n: f64) -> i32;
pub fn round(n: f64, decimals: Option<i32>) -> f64;
pub fn rnd(n: Option<f32>) -> f32;
pub fn randomize(seed: Option<i32>);
// ... more math functions
```

#### Date/Time Functions (`stdlib/date.rs`)

```rust
pub fn now() -> f64;
pub fn date() -> f64;
pub fn time() -> f64;
pub fn timer() -> f32;
pub fn year(date: f64) -> i32;
pub fn month(date: f64) -> i32;
pub fn day(date: f64) -> i32;
pub fn hour(time: f64) -> i32;
pub fn minute(time: f64) -> i32;
pub fn second(time: f64) -> i32;
pub fn weekday(date: f64) -> i32;
pub fn date_add(interval: &str, number: i32, date: f64) -> Result<f64>;
pub fn date_diff(interval: &str, date1: f64, date2: f64) -> Result<i32>;
pub fn date_value(s: &str) -> Result<f64>;
pub fn time_value(s: &str) -> Result<f64>;
// ... more date functions
```

### 6. Runtime Context (`runtime/`)

Manages execution state for both interpreter and generated code:

```rust
pub struct RuntimeContext {
    /// Global variables
    globals: HashMap<String, Value>,
    
    /// Call stack
    call_stack: Vec<StackFrame>,
    
    /// File handles (for file I/O)
    file_handles: HashMap<i32, FileHandle>,
    
    /// Forms (lazy-loaded)
    forms: HashMap<String, Box<dyn Form>>,
    
    /// COM objects (future)
    com_objects: ObjectRegistry,
    
    /// Random number generator state
    rng: Option<Rng>,
    
    /// Error handling state
    error_state: ErrorState,
    
    /// Debug mode
    debug: bool,
}

pub struct StackFrame {
    pub function_name: String,
    pub locals: HashMap<String, Value>,
    pub return_address: Option<usize>,
    pub source_file: String,
    pub line_number: usize,
}

pub struct ErrorState {
    pub mode: ErrorMode,
    pub last_error: Option<RuntimeError>,
    pub error_handler: Option<Label>,
}

pub enum ErrorMode {
    Propagate,
    ResumeNext,
    GoTo(Label),
}

impl RuntimeContext {
    pub fn new() -> Self;
    
    // Variable access
    pub fn get_global(&self, name: &str) -> Result<&Value>;
    pub fn set_global(&mut self, name: &str, value: Value);
    pub fn get_local(&self, name: &str) -> Result<&Value>;
    pub fn set_local(&mut self, name: &str, value: Value);
    
    // Stack management
    pub fn push_frame(&mut self, frame: StackFrame);
    pub fn pop_frame(&mut self) -> Result<StackFrame>;
    pub fn current_frame(&self) -> Option<&StackFrame>;
    
    // Error handling
    pub fn set_error(&mut self, error: RuntimeError);
    pub fn clear_error(&mut self);
    pub fn handle_error(&mut self) -> ErrorAction;
    
    // File I/O
    pub fn open_file(&mut self, file_number: i32, path: &str, mode: FileMode) -> Result<()>;
    pub fn close_file(&mut self, file_number: i32) -> Result<()>;
    pub fn get_file_handle(&mut self, file_number: i32) -> Result<&mut FileHandle>;
}
```

### 7. Array Implementation (`array.rs`)

VB6 arrays have unique semantics that must be preserved:

```rust
pub struct Array {
    /// Element type
    element_type: VBType,
    
    /// Dimensions with bounds
    dimensions: Vec<(i32, i32)>,  // (lower, upper)
    
    /// Flat storage
    data: Vec<Value>,
}

impl Array {
    /// Create array with given bounds
    pub fn new(element_type: VBType, dimensions: Vec<(i32, i32)>) -> Self;
    
    /// Get element at indices
    pub fn get(&self, indices: &[i32]) -> Result<&Value>;
    
    /// Set element at indices
    pub fn set(&mut self, indices: &[i32], value: Value) -> Result<()>;
    
    /// Redimension array
    pub fn redim(&mut self, new_dimensions: Vec<(i32, i32)>, preserve: bool) -> Result<()>;
    
    /// Get lower bound of dimension
    pub fn lbound(&self, dimension: usize) -> Result<i32>;
    
    /// Get upper bound of dimension
    pub fn ubound(&self, dimension: usize) -> Result<i32>;
    
    /// Convert multi-dimensional index to flat index
    fn indices_to_offset(&self, indices: &[i32]) -> Result<usize>;
}
```

**Array Design Notes**:
- **Custom Bounds**: Arrays can start at any index (e.g., `Dim A(5 To 10)`)
- **ReDim Preserve**: Only last dimension can change when preserving
- **Zero-Based Warning**: VB6 defaults to 1-based unless `Option Base 0`

## Integration Points

### With vb6parse

```rust
use vb6parse::language::*;
use vb6core::ir::*;

pub fn ast_to_ir(module: &Module) -> Result<IRModule> {
    // Convert parsed AST to IR
}
```

### With vb6semantic

```rust
use vb6semantic::*;
use vb6core::types::VBType;

pub fn semantic_type_to_vb_type(sem_type: &TypeInfo) -> VBType {
    // Convert semantic analysis types to runtime types
}
```

### With vb6interpret

```rust
use vb6core::runtime::RuntimeContext;
use vb6core::ir::IRModule;

pub fn execute(module: &IRModule, ctx: &mut RuntimeContext) -> Result<()> {
    // Execute IR in interpreter
}
```

### With vb6compile

```rust
use vb6core::ir::IRModule;

pub fn compile_to_rust(module: &IRModule) -> Result<String> {
    // Generate Rust code from IR
}

pub fn compile_to_llvm(module: &IRModule) -> Result<LLVMModule> {
    // Generate LLVM IR from VB6 IR
}
```

## Testing Strategy

### Unit Tests
- Every function in stdlib has comprehensive tests
- Type conversion edge cases
- Array bounds and operations
- Error handling scenarios

### Integration Tests
- Complete VB6 programs executed
- Compared against actual VB6 output
- Test harness integration (see separate doc)

### Property-Based Tests
- Conversion round-trips
- Arithmetic properties
- String operation correctness

### Benchmark Tests
- String operations performance
- Numeric computation speed
- Function call overhead
- Array access patterns

## Performance Considerations

### Optimization Opportunities

1. **Value Representation**:
   - Consider using enum discriminant packing
   - Pool allocations for String values
   - Intern common strings

2. **Conversion Caching**:
   - Cache type conversion results
   - Memoize format strings

3. **Array Access**:
   - Inline bounds checking in hot loops
   - SIMD for array operations when possible

4. **Function Calls**:
   - Inline built-in functions when possible
   - Direct dispatch table for stdlib

5. **Variant Operations**:
   - Quick path for common variant types
   - Avoid boxing when possible

## Future Enhancements

1. **JIT Compilation**: Compile hot functions at runtime
2. **Parallel Execution**: Safe parallelization of independent operations
3. **Memory Management**: Garbage collection for object references
4. **COM Interop**: Full support for calling COM objects
5. **GPU Acceleration**: Use GPU for numeric arrays
6. **Profile-Guided Optimization**: Optimize based on runtime profiles

## Dependencies

- `vb6parse`: ^0.5.0
- `vb6semantic`: ^0.1.0
- `num-traits`: ^0.2
- `num-complex`: ^0.4
- `serde`: ^1.0
- `thiserror`: ^1.0
- `anyhow`: ^1.0
- `inkwell`: ^0.4 (optional, for LLVM backend)

## License

MIT
