# vb6core

Shared type system and error model for the VB6 compiler and interpreter.

## Overview

`vb6core` owns the foundational types that both `vb6semantic` (static analysis)
and `vb6runtime` (execution) depend on, so the whole toolchain agrees on one
type system:

- **[`VBType`]** — the single source of truth for VB6 static types: primitives
  (`Byte`…`Date`), complex types (`Class`, `UserType`, `Enum`, `Array`), value
  states (`Empty`, `Null`, `Nothing`, `Error`), procedure types (`Sub`,
  `Function`), and `Unknown`. Includes VB6-exact `VarType` codes and widening
  rules.
- **[`TypeInfo`]** — wraps a `VBType` with the metadata static analysis needs:
  `ByRef` references, array bounds, and class names.
- **[`VBError`]** — runtime errors mirroring the VB6 `Err` object (`number`,
  `description`, `source`, `help_file`, `help_context`) plus the standard
  built-in error numbers and descriptions.

Keeping these types here ensures semantic analysis, code generation, and the
interpreter never drift apart (they previously lived in duplicated form in
`vb6semantic` and `vb6runtime`).

## Architecture

```
┌──────────────────┐          ┌──────────────────┐
│  vb6interpret    │          │   vb6compile     │
└────────┬─────────┘          └────────┬─────────┘
         │                             │
    ┌────▼─────────┐             ┌─────▼─────────┐
    │ vb6runtime   │             │ vb6semantic   │
    └────┬─────────┘             └─────┬─────────┘
         │                             │
         └─────────────┬───────────────┘
                  ┌────▼──────┐
                  │ vb6core   │   shared types + errors
                  └───────────┘
```

## Usage

```rust
use vb6core::{VBType, TypeInfo};

// Type codes and widening
assert_eq!(VBType::Long.var_type(), 3);
assert!(VBType::Integer.can_assign_to(&VBType::Double));
assert_eq!(VBType::from_var_type(8194), Some(VBType::Array(Box::new(VBType::Integer))));

// Static metadata around a type
let mut info = TypeInfo::new(VBType::Integer);
info.is_array = true;
assert_eq!(info.to_string(), "Integer()");

// Runtime errors mirroring Err
use vb6core::VBError;
let err = VBError::type_mismatch();
assert_eq!(err.number, 13);
assert_eq!(err.to_string(), "Type mismatch (Error 13)");
```

## Dependencies

- `serde` — serialization support for types shared across crates
- `inkwell` (optional, `llvm` feature) — reserved for the LLVM backend

`vb6core` is a leaf crate: it depends on no other `vb6` project. `vb6semantic`
and `vb6runtime` both depend on it.

## Testing

```bash
cargo test -p vb6core
```

## License

MIT License - see LICENSE file for details.
