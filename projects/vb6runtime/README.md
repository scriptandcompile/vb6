# vb6runtime

VB6 runtime library providing value system, type conversions, and standard library implementations.

## Overview

`vb6runtime` provides the runtime execution infrastructure for VB6 programs. It is used by both `vb6interpret` (for direct execution) and `vb6compile` (for linking compiled output with runtime support). This crate contains everything needed to execute VB6 code at runtime, but not the compilation/IR infrastructure.

## Architecture

```
┌─────────────────┐         ┌─────────────────┐
│  vb6interpret   │         │    vb6compile   │
└────────┬────────┘         └────────┬────────┘
         │                           │
         └───────────┬───────────────┘
                     │
              ┌──────▼───────┐
              │  vb6runtime  │
              └──────┬───────┘
                     │
              ┌──────▼───────┐
              │  vb6semantic │
              └──────────────┘
                     │
              ┌──────▼───────┐
              │   vb6parse   │
              └──────────────┘
```

**Note**: `vb6core` contains the foundational VB6 type definitions (`VBType`, `VBError`), while `vb6runtime` provides the execution infrastructure on top of them.

## Core Components

### 1. Value System

Runtime representation of all VB6 values:

```rust
pub enum VBVariant {
    Empty,
    Null,
    Nothing,
    Byte(u8),
    Integer(i16),
    Long(i32),
    Single(f32),
    Double(f64),
    Currency(i64),      // Fixed-point decimal
    Date(f64),          // OLE Date
    String(String),
    Boolean(bool),      // True = -1, False = 0
    Object(Box<dyn VBObject>),
    Array(ArrayValue),
    Error(VBError),
}
```

### 2. Shared Type System

The runtime crate uses the shared type model from `vb6core` instead of maintaining a second copy of the VB6 type definitions. Runtime code consumes `VBType`, `TypeInfo`, `ArrayBound`, `VBError`, and `VBResult` from `vb6core`, while `vb6runtime` adds the dynamic value layer (`VBVariant`, `ArrayValue`, and conversion behavior):

```rust
use vb6core::error::{VBError, VBResult};
use vb6core::types::{ArrayBound, TypeInfo, VBType};
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

This model keeps `VBVariant` focused on data representation while letting the runtime distinguish:

- omitted optional arguments
- `ByRef` versus `ByVal` semantics
- variable-length argument lists for `ParamArray`
- VB6-specific cases such as `Empty`, `Null`, and `Missing`

In practice, a call flow evaluates each argument expression into a `VBVariant`, wraps it in a call context, binds it to the formal parameter list, and then dispatches to the appropriate procedure implementation.

### 5. Standard Library

Full implementations of VB6 built-in functions are in `vb6runtime::library::`.

See [vb6runtime library docs](https://scriptandcompile.github.io/vb6/vb6runtime/library/) for the complete function reference.

### 6. Runtime State

Process-global state (environment snapshot, RNG seed, settings store) lives in `vb6runtime::state::`.

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
use vb6runtime::VBVariant;

let num = VBVariant::from_integer(42);
let text = VBVariant::from_string("Hello");
```

### Type Conversion

VB6 runtime provides `TryFrom<&VBVariant>` implementations on typed wrappers for VB6-exact conversion:

```rust
use vb6runtime::VBVariant;

let value = VBVariant::from_string("42");
let long_val: vb6runtime::VBLong = value.try_into().unwrap();
assert_eq!(long_val.as_i32(), 42);
```

### Library Functions

VB6 library functions are in `vb6runtime::library::`:

```rust
use vb6runtime::library::string;

let result = string::left("Hello World", 5);  // "Hello"
```

## Known Limitations

- FRX resource file handling is limited: binary blobs are loaded but not all are mapped to control properties.
- The runtime does not yet support VB6 class modules or COM object integration.

## Dependencies

- `thiserror` - Error handling
- `serde` - Serialization support
- `num-traits` - Numeric operations
- `jiff` - Date/time operations
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

✅ **Active Development** — Core value system, type conversions, and 50+ library functions are implemented. See [DESIGN.md](docs/DESIGN.md) for detailed design documentation.

## See Also

- [Parameter Classification](docs/parameter-classification.md) — How library function parameters are classified for conversion

## License

MIT License - See [LICENSE](../LICENSE) file for details.
