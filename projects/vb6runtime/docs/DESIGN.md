# vb6runtime Design Document

## Overview

`vb6runtime` provides the runtime execution infrastructure for VB6 programs. It contains the value system, type conversions, standard library implementations, and runtime context needed to execute VB6 code. This crate is separate from `vb6core` which handles compilation and IR concerns.

## Goals

1. **VB6 Compatibility**: Exact VB6 runtime semantics, including all quirks and edge cases
2. **Code Reuse**: Shared runtime between interpreter and compiled output
3. **Performance**: Efficient value operations and function calls
4. **Testability**: Comprehensive tests validated against real VB6 behavior
5. **Portability**: Runtime works on all platforms (Windows, Linux, macOS)

## Architecture

### Module Structure

```
vb6runtime/
├── src/
│   ├── lib.rs                  # Public API
│   ├── value.rs                # Value type system
│   ├── types.rs                # VB6 type definitions
│   ├── conversion.rs           # Type conversion rules
│   ├── runtime/
│   │   ├── mod.rs
│   │   ├── context.rs          # Runtime execution context
│   │   ├── stack.rs            # Call stack management
│   │   ├── error.rs            # Error handling (On Error)
│   │   └── files.rs            # File handle management
│   ├── stdlib/
│   │   ├── mod.rs
│   │   ├── string.rs           # String functions (Left$, Right$, etc.)
│   │   ├── math.rs             # Math functions (Sin, Cos, Sqr, etc.)
│   │   ├── datetime.rs         # Date/Time functions (Now, DateAdd, etc.)
│   │   ├── conversion.rs       # Conversion functions (CInt, CDbl, etc.)
│   │   ├── format.rs           # Format functions
│   │   ├── file.rs             # File I/O functions (Dir, EOF, etc.)
│   │   ├── array.rs            # Array functions (UBound, Join, etc.)
│   │   └── interaction.rs      # MsgBox, InputBox, Shell, etc.
│   ├── array.rs                # Array implementation
│   ├── variant.rs              # Variant type implementation
│   └── object.rs               # Object system (forms, controls, etc.)
├── tests/
│   ├── value_tests.rs
│   ├── conversion_tests.rs
│   ├── stdlib/
│   │   ├── string_tests.rs
│   │   ├── math_tests.rs
│   │   └── datetime_tests.rs
│   └── integration_tests.rs
└── benches/
    ├── value_ops.rs
    ├── conversions.rs
    └── stdlib_functions.rs
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
    /// Get the VB6 type of this value
    pub fn value_type(&self) -> VBType;
    
    /// Check if value is numeric
    pub fn is_numeric(&self) -> bool;
    
    /// Check if value is empty or null
    pub fn is_empty_or_null(&self) -> bool;
    
    /// Convert to boolean (VB6 rules: 0 = False, non-zero = True)
    pub fn to_boolean(&self) -> Result<bool>;
    
    /// Convert to string (VB6 rules)
    pub fn to_string(&self) -> Result<String>;
    
    /// Convert to number
    pub fn to_number<T: FromValue>(&self) -> Result<T>;
    
    /// Compare values (VB6 comparison rules)
    pub fn compare(&self, other: &Value) -> Result<Ordering>;
    
    /// Arithmetic operations
    pub fn add(&self, other: &Value) -> Result<Value>;
    pub fn subtract(&self, other: &Value) -> Result<Value>;
    pub fn multiply(&self, other: &Value) -> Result<Value>;
    pub fn divide(&self, other: &Value) -> Result<Value>;
    pub fn integer_divide(&self, other: &Value) -> Result<Value>;
    pub fn modulo(&self, other: &Value) -> Result<Value>;
    pub fn power(&self, other: &Value) -> Result<Value>;
    pub fn negate(&self) -> Result<Value>;
    
    /// String concatenation
    pub fn concat(&self, other: &Value) -> Result<Value>;
    
    /// Logical operations
    pub fn logical_and(&self, other: &Value) -> Result<Value>;
    pub fn logical_or(&self, other: &Value) -> Result<Value>;
    pub fn logical_not(&self) -> Result<Value>;
    pub fn logical_xor(&self, other: &Value) -> Result<Value>;
}
```

**Key Design Decisions**:
- **Boolean Representation**: VB6 uses -1 for True, 0 for False
- **Currency As Integer**: Store as i64 with 4 implied decimal places (10000 per unit)
- **Date As Float**: OLE automation date (days since 1899-12-30)
- **Variant Boxing**: Use Box to avoid recursive type definition
- **Object References**: Keep lightweight references, objects stored in RuntimeContext

### 2. Shared Type System

`vb6runtime` does not define its own copy of the VB6 type model. It consumes the shared definitions from `vb6core`:

```rust
use vb6core::error::{VBError, VBResult};
use vb6core::types::{ArrayBound, TypeInfo, VBType};
```

The runtime crate contributes the dynamic value layer (`Value`, `ArrayValue`, and conversion semantics) on top of that shared type system.

### 3. Type Conversion (`conversion.rs`)

VB6 has complex conversion rules that must be implemented exactly:

```rust
/// Convert a value to a target type
pub fn convert(value: &Value, target: &VBType) -> Result<Value>;

/// Numeric type promotion for binary operations
pub fn promote_numeric(left: &Value, right: &Value) -> Result<(Value, Value)>;

/// Implicit conversions (VB6 automatic conversions)
pub fn implicit_convert(value: &Value, target: &VBType) -> Result<Value>;

/// Explicit conversions (CInt, CLng, etc.)
pub fn explicit_convert(value: &Value, target: &VBType) -> Result<Value>;

/// Parse string to number (handles VB6 string formats)
pub fn parse_numeric(s: &str, target: &VBType) -> Result<Value>;

/// Parse string to date (handles VB6 date formats)
pub fn parse_date(s: &str) -> Result<f64>;
```

**Conversion Rules**:
1. **Widening**: Byte → Integer → Long → Single → Double → Variant
2. **String Parsing**: Recognize numeric strings, dates, booleans
3. **Boolean**: -1 for True, 0 for False in numeric context
4. **Null Propagation**: Null propagates through most operations
5. **Error Values**: Error values propagate through expressions
6. **Currency Precision**: Maintain 4 decimal places
7. **Date Handling**: OLE automation date format

### 4. Runtime Context (`runtime/`)

```rust
pub struct RuntimeContext {
    /// Global variables (module-level)
    globals: HashMap<String, Value>,
    
    /// Call stack
    call_stack: Vec<StackFrame>,
    
    /// File handles (for Open/Close/Print #)
    file_handles: HashMap<i32, FileHandle>,
    
    /// Error state (On Error Resume Next/GoTo)
    error_mode: ErrorMode,
    current_error: Option<RuntimeError>,
    
    /// Random number generator state
    rng: Rng,
    
    /// Forms currently loaded
    forms: HashMap<String, Form>,
    
    /// Timer for DoEvents
    event_timer: EventTimer,
}

impl RuntimeContext {
    pub fn new() -> Self;
    
    /// Get/set global variables
    pub fn get_global(&self, name: &str) -> Result<&Value>;
    pub fn set_global(&mut self, name: &str, value: Value);
    
    /// Call stack operations
    pub fn push_frame(&mut self, frame: StackFrame);
    pub fn pop_frame(&mut self) -> Option<StackFrame>;
    pub fn current_frame(&self) -> Option<&StackFrame>;
    pub fn current_frame_mut(&mut self) -> Option<&mut StackFrame>;
    
    /// File operations
    pub fn open_file(&mut self, handle: i32, file: FileHandle) -> Result<()>;
    pub fn close_file(&mut self, handle: i32) -> Result<()>;
    pub fn get_file(&self, handle: i32) -> Result<&FileHandle>;
    pub fn get_file_mut(&mut self, handle: i32) -> Result<&mut FileHandle>;
    pub fn free_file_number(&self) -> i32;
    
    /// Error handling
    pub fn set_error(&mut self, error: RuntimeError);
    pub fn clear_error(&mut self);
    pub fn get_error(&self) -> Option<&RuntimeError>;
    pub fn set_error_mode(&mut self, mode: ErrorMode);
    
    /// Random numbers
    pub fn randomize(&mut self, seed: Option<i32>);
    pub fn rnd(&mut self) -> f64;
}

pub struct StackFrame {
    pub function_name: String,
    pub locals: HashMap<String, Value>,
    pub line_number: usize,
}

pub enum ErrorMode {
    Propagate,
    ResumeNext,
    GoTo(String),
}

pub struct RuntimeError {
    pub number: i32,
    pub description: String,
    pub source: String,
    pub line: usize,
}
```

### 5. Standard Library (`stdlib/`)

Each module implements a category of VB6 built-in functions:

#### String Functions (`stdlib/string.rs`)
```rust
pub fn left(s: &str, length: i32) -> Result<String>;
pub fn right(s: &str, length: i32) -> Result<String>;
pub fn mid(s: &str, start: i32, length: Option<i32>) -> Result<String>;
pub fn len(s: &str) -> i32;
pub fn instr(start: Option<i32>, s1: &str, s2: &str, compare: Option<i32>) -> i32;
pub fn replace(s: &str, find: &str, replace: &str, start: Option<i32>, count: Option<i32>, compare: Option<i32>) -> Result<String>;
pub fn trim(s: &str) -> String;
pub fn ltrim(s: &str) -> String;
pub fn rtrim(s: &str) -> String;
pub fn ucase(s: &str) -> String;
pub fn lcase(s: &str) -> String;
pub fn string_repeat(count: i32, character: &str) -> Result<String>;
pub fn space(count: i32) -> Result<String>;
pub fn strcomp(s1: &str, s2: &str, compare: Option<i32>) -> i32;
```

#### Math Functions (`stdlib/math.rs`)
```rust
pub fn abs<T: Numeric>(value: T) -> T;
pub fn sgn<T: Numeric>(value: T) -> i32;
pub fn sin(angle: f64) -> f64;
pub fn cos(angle: f64) -> f64;
pub fn tan(angle: f64) -> f64;
pub fn atn(value: f64) -> f64;
pub fn log(value: f64) -> Result<f64>;
pub fn exp(value: f64) -> f64;
pub fn sqr(value: f64) -> Result<f64>;
pub fn int(value: f64) -> f64;
pub fn fix(value: f64) -> f64;
pub fn round(value: f64, places: Option<i32>) -> f64;
```

#### Date/Time Functions (`stdlib/datetime.rs`)
```rust
pub fn now() -> f64;
pub fn date() -> f64;
pub fn time() -> f64;
pub fn year(date: f64) -> i32;
pub fn month(date: f64) -> i32;
pub fn day(date: f64) -> i32;
pub fn hour(time: f64) -> i32;
pub fn minute(time: f64) -> i32;
pub fn second(time: f64) -> i32;
pub fn weekday(date: f64, first_day: Option<i32>) -> i32;
pub fn date_add(interval: &str, number: i32, date: f64) -> Result<f64>;
pub fn date_diff(interval: &str, date1: f64, date2: f64) -> Result<i32>;
pub fn date_serial(year: i32, month: i32, day: i32) -> f64;
pub fn time_serial(hour: i32, minute: i32, second: i32) -> f64;
pub fn date_value(s: &str) -> Result<f64>;
pub fn time_value(s: &str) -> Result<f64>;
```

#### Conversion Functions (`stdlib/conversion.rs`)
```rust
pub fn cint(value: &Value) -> Result<i16>;
pub fn clng(value: &Value) -> Result<i32>;
pub fn cdbl(value: &Value) -> Result<f64>;
pub fn csng(value: &Value) -> Result<f32>;
pub fn cstr(value: &Value) -> Result<String>;
pub fn cbool(value: &Value) -> Result<bool>;
pub fn cbyte(value: &Value) -> Result<u8>;
pub fn ccur(value: &Value) -> Result<i64>;
pub fn cdate(value: &Value) -> Result<f64>;
pub fn cvar(value: &Value) -> Result<Value>;
pub fn val(s: &str) -> f64;
pub fn str_value(value: f64) -> String;
pub fn hex(value: i32) -> String;
pub fn oct(value: i32) -> String;
pub fn asc(s: &str) -> Result<i32>;
pub fn chr(code: i32) -> Result<String>;
pub fn ascw(s: &str) -> Result<u16>;
pub fn chrw(code: u16) -> Result<String>;
```

#### Array Functions (`stdlib/array.rs`)
```rust
pub fn lbound(arr: &Array, dimension: Option<i32>) -> Result<i32>;
pub fn ubound(arr: &Array, dimension: Option<i32>) -> Result<i32>;
pub fn array(values: Vec<Value>) -> Array;
pub fn join(arr: &Array, delimiter: Option<&str>) -> Result<String>;
pub fn split(s: &str, delimiter: Option<&str>, limit: Option<i32>) -> Result<Array>;
pub fn filter(arr: &Array, match_str: &str, include: Option<bool>, compare: Option<i32>) -> Result<Array>;
```

#### File I/O Functions (`stdlib/file.rs`)
```rust
pub fn dir(ctx: &mut RuntimeContext, path: Option<&str>, attributes: Option<i32>) -> Result<String>;
pub fn eof(ctx: &RuntimeContext, file_number: i32) -> Result<bool>;
pub fn lof(ctx: &RuntimeContext, file_number: i32) -> Result<i64>;
pub fn file_len(path: &str) -> Result<i64>;
pub fn file_date_time(path: &str) -> Result<f64>;
pub fn free_file(ctx: &RuntimeContext) -> i32;
pub fn file_attr(ctx: &RuntimeContext, file_number: i32) -> Result<i32>;
pub fn seek_pos(ctx: &RuntimeContext, file_number: i32) -> Result<i64>;
```

### 6. Array Implementation (`array.rs`)

```rust
pub struct Array {
    /// Element type
    element_type: VBType,
    
    /// Dimensions
    dimensions: Vec<Dimension>,
    
    /// Flattened data storage
    data: Vec<Value>,
}

pub struct Dimension {
    pub lower_bound: i32,
    pub upper_bound: i32,
}

impl Array {
    /// Create new array with dimensions
    pub fn new(element_type: VBType, dimensions: Vec<Dimension>) -> Self;
    
    /// Get element at indices
    pub fn get(&self, indices: &[i32]) -> Result<&Value>;
    
    /// Set element at indices
    pub fn set(&mut self, indices: &[i32], value: Value) -> Result<()>;
    
    /// Get number of dimensions
    pub fn num_dimensions(&self) -> usize;
    
    /// Get dimension info
    pub fn dimension(&self, dim: usize) -> Option<&Dimension>;
    
    /// ReDim - resize array (loses data unless Preserve)
    pub fn redim(&mut self, dimensions: Vec<Dimension>, preserve: bool) -> Result<()>;
}
```

## Implementation Priorities

### Phase 1: Core Infrastructure
1. Value type and basic operations
2. Type system
3. Type conversions
4. Runtime context

### Phase 2: Essential Standard Library
1. String functions
2. Math functions
3. Conversion functions
4. Basic array support

### Phase 3: Advanced Features
1. Date/time functions
2. File I/O
3. Format functions
4. Full array operations

### Phase 4: Advanced Runtime
1. Object system
2. Forms and controls
3. Error handling (On Error)
4. DoEvents and event processing

## Testing Strategy

1. **Unit Tests**: Each function tested against VB6 reference implementation
2. **Property Tests**: Random input validation
3. **Integration Tests**: Complete VB6 programs
4. **Benchmark Tests**: Performance comparison
5. **Edge Cases**: Empty, Null, Error values
6. **Compatibility Tests**: Validated against original VB6

## Dependencies

- **vb6parse**: For any parsing needs (Format strings, etc.)
- **thiserror**: Error handling
- **serde**: Serialization support
- **num-traits**: Numeric operations
- **jiff**: Date/time operations
- **encoding_rs**: String encoding (Windows-1252)

## Integration

### With vb6interpret
```rust
use vb6runtime::{RuntimeContext, Value};

let mut ctx = RuntimeContext::new();
ctx.set_global("x", Value::Integer(10));
// Execute interpreter instructions using ctx
```

### With vb6compile
Compiled code links against vb6runtime and calls functions directly:
```rust
// Generated Rust code
use vb6runtime::stdlib;

pub fn main() {
    let result = vb6runtime::stdlib::string::left("Hello", 3);
    vb6runtime::stdlib::interaction::msgbox(&result.unwrap());
}
```

## Future Enhancements

1. **COM Interop**: CreateObject, GetObject support
2. **Async Operations**: DoEvents, timers
3. **Graphics**: Form rendering, graphics methods
4. **Database**: ADODB support
5. **Optimization**: JIT compilation of hot paths
6. **FFI**: P/Invoke support for Windows APIs

## References

- VB6 Language Reference
- MSDN VB6 Documentation
- VB6 Runtime Library behavior validation
