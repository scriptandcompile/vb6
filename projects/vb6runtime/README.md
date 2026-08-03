# vb6runtime

VB6 runtime library providing value system, type conversions, and standard library implementations.

## Overview

`vb6runtime` provides the runtime execution infrastructure for VB6 programs. It is used by both `vb6interpret` (for direct execution) and `vb6compile` (for linking compiled output with runtime support). This crate contains everything needed to execute VB6 code at runtime, but not the compilation/IR infrastructure.

## Architecture

```
┌─────────────────┐         ┌─────────────────┐
│  vb6interpret  │         │   vb6compile   │
└────────┬────────┘         └────────┬────────┘
         │                           │
         └───────────┬───────────────┘
                     │
              ┌──────▼──────────┐
              │   vb6runtime   │
              └──────┬──────────┘
                     │
              ┌──────▼──────┐
              │   vb6parse  │
              └─────────────┘
```

**Note**: `vb6core` contains the IR and compilation infrastructure, while `vb6runtime` contains the runtime execution infrastructure.

## Core Components

### 1. Value System

Runtime representation of all VB6 values:

```rust
pub enum Value {
    Empty,
    Null,
    Byte(u8),
    Integer(i16),
    Long(i32),
    Single(f32),
    Double(f64),
    Currency(i64),      // Fixed-point decimal
    Date(f64),          // OLE Date
    String(String),
    Boolean(bool),      // True = -1, False = 0
    Variant(Box<Value>),
    Object(ObjectRef),
    Array(Array),
    UserDefined(HashMap<String, Value>),
    Error(i32),
}
```

### 2. Type System

VB6 type information and rules:

```rust
pub enum VBType {
    Byte,
    Boolean,
    Integer,
    Long,
    Single,
    Double,
    Currency,
    Date,
    String(Option<usize>),  // Fixed-length strings
    Variant,
    Object(Option<String>), // Optional class name
    Array(Box<VBType>, Vec<(i32, i32)>),  // Type + bounds
    UserDefined(String),
}
```

### 3. Type Conversions

VB6-exact conversion rules:

- Widening conversions (Byte → Integer → Long → Double → Variant)
- String to number parsing
- Boolean representations (True = -1, False = 0)
- Variant unwrapping
- Null propagation

### 4. Argument Handling Model

VB6 procedures do not pass arguments as a bare list of values. The runtime should model an invocation as a structured call context so it can preserve semantics for omitted arguments, `ByRef`/`ByVal`, and `ParamArray` handling.

```rust
pub enum ArgumentPresence {
    Present(Value),
    Missing,
}

pub struct RuntimeArgument {
    pub presence: ArgumentPresence,
    pub by_ref: bool,
}

pub struct CallFrame {
    pub callee: String,
    pub args: Vec<RuntimeArgument>,
    pub named: HashMap<String, RuntimeArgument>,
}
```

This model keeps `Value` focused on data representation while letting the runtime distinguish:

- omitted optional arguments
- `ByRef` versus `ByVal` semantics
- variable-length argument lists for `ParamArray`
- VB6-specific cases such as `Empty`, `Null`, and `Missing`

In practice, a call flow should evaluate each argument expression into a `Value`, wrap it in `RuntimeArgument`, bind it to the formal parameter list, and then dispatch to the appropriate procedure implementation.

### 5. Standard Library

Full implementations of VB6 built-in functions:

#### String Functions
- `Left$`, `Right$`, `Mid$` - String extraction
- `Len`, `InStr`, `Replace` - String operations
- `Trim`, `LTrim`, `RTrim` - Whitespace handling
- `UCase`, `LCase` - Case conversion
- `String$`, `Space$` - String generation

#### Math Functions
- `Abs`, `Sgn` - Basic math
- `Sin`, `Cos`, `Tan`, `Atn` - Trigonometry
- `Log`, `Exp` - Logarithms
- `Sqr` - Square root
- `Rnd`, `Randomize` - Random numbers
- `Int`, `Fix`, `Round` - Rounding

#### Conversion Functions
- `CInt`, `CLng`, `CDbl`, `CSng` - Numeric conversions
- `CStr`, `CBool`, `CDate` - Type conversions
- `Val`, `Str$` - String/number conversion
- `Hex$`, `Oct$` - Base conversions
- `Asc`, `Chr$`, `AscW`, `ChrW$` - Character codes

#### Date/Time Functions
- `Now`, `Date`, `Time` - Current date/time
- `Year`, `Month`, `Day`, `Hour`, `Minute`, `Second` - Date parts
- `DateAdd`, `DateDiff` - Date arithmetic
- `DateSerial`, `TimeSerial` - Date construction
- `DateValue`, `TimeValue` - Date parsing

#### Array Functions
- `LBound`, `UBound` - Array bounds
- `Array` - Create array from values
- `Join`, `Split` - Array/string conversion
- `Filter` - Array filtering

#### Format Functions
- `Format$` - General formatting
- `FormatNumber`, `FormatCurrency` - Numeric formatting
- `FormatPercent`, `FormatDateTime` - Specialized formatting

#### File I/O Functions
- `Dir`, `EOF`, `LOF` - File properties
- `FileLen`, `FileDateTime` - File information
- `FreeFile` - Get free file handle
- `Input$`, `Line Input` - File reading
- `Open`, `Close`, `Reset` - File operations
- `Get`, `Put` - Binary I/O
- `Print #`, `Write #` - Text output

#### Interaction Functions
- `MsgBox` - Message boxes
- `InputBox` - Input dialogs
- `Shell` - Execute programs
- `Environ` - Environment variables
- `Command$` - Command line arguments

### 5. Runtime Context

Execution state management:

```rust
pub struct RuntimeContext {
    /// Global variables
    pub globals: HashMap<String, Value>,
    
    /// Call stack
    pub call_stack: Vec<StackFrame>,
    
    /// File handles
    pub file_handles: HashMap<i32, FileHandle>,
    
    /// Error state (On Error Resume Next/GoTo)
    pub error_state: Option<RuntimeError>,
    
    /// Random number generator state
    pub rng: RandomState,
}
```

### 6. Error Handling

VB6-compatible error system:

```rust
pub enum ErrorMode {
    Propagate,          // Default: propagate to caller
    ResumeNext,         // On Error Resume Next
    GoTo(String),       // On Error GoTo label
}

pub struct RuntimeError {
    pub number: i32,
    pub description: String,
    pub source: String,
    pub line: usize,
}
```

## Features

### Value Operations
- Arithmetic with proper type promotion
- String concatenation
- Comparison operators
- Variant coercion

### Array Support
- Dynamic arrays with ReDim
- Preserve semantics
- Multi-dimensional arrays
- Custom bounds (arrays starting at any index)

### Type Safety
- VB6-exact conversion rules
- Null and Empty handling
- Error value propagation
- Variant type checking

## Usage

### Creating Values

```rust
use vb6runtime::value::Value;

let num = Value::Integer(42);
let text = Value::String("Hello".to_string());
let var = Value::Variant(Box::new(Value::Long(100)));
```

### Type Conversion

```rust
use vb6runtime::conversion::convert;

let value = Value::String("42".to_string());
let number = convert(&value, &VBType::Integer)?;  // Value::Integer(42)
```

### Calling Built-in Functions

```rust
use vb6runtime::stdlib;

let result = stdlib::string::left("Hello World", 5)?;  // "Hello"
let length = stdlib::string::len("Test")?;              // 4
let now = stdlib::datetime::now()?;                     // Current date/time
```

### Runtime Context

```rust
use vb6runtime::runtime::RuntimeContext;

let mut ctx = RuntimeContext::new();
ctx.set_global("MyVar", Value::Integer(10));
let value = ctx.get_global("MyVar")?;
```

## Dependencies

- `thiserror` - Error handling
- `serde` - Serialization support
- `num-traits` - Numeric operations
- `chrono` - Date/time operations
- `encoding_rs` - String encoding

## Testing

```bash
# Run all tests
cargo test -p vb6runtime

# Run with output
cargo test -p vb6runtime -- --nocapture

# Run benchmarks
cargo bench -p vb6runtime
```

## Status

🚧 **In Design Phase** - Core architecture defined, implementation pending

See [DESIGN.md](docs/DESIGN.md) for detailed design documentation.

## License

MIT License - See [LICENSE](../LICENSE) file for details.
