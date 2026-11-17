# VB6R M3 migration status

## Scope

The original VB6R repository at `/Users/lvx/Dev/vb6-r` was inspected read-only.
Runtime code was adapted to the fork's existing `VBVariant` and multidimensional
`ArrayValue`; M3's `VbValue` and one-dimensional `VbArray` were not substituted.
No dependency was added, and Cargo manifests and lockfiles were left unchanged.

## Implemented in this pass

- `VBType::Decimal` is distinct from `Currency`; `VarType` 14 now maps to
  Decimal, including array VarType mapping. Assignability only accepts the
  same Decimal type or widening to Variant; Decimal and Currency do not alias.
- `VBVariant::Decimal` stores an `i128` coefficient and decimal scale.
  Formatting, Decimal-to-integer and Decimal-to-Currency conversion are
  checked; ties round to even. Decimal arithmetic and full VB6 Decimal bounds
  are not implemented.
- `VBCurrency` has checked add/subtract/negate and exact-rescaling multiply and
  divide; interpreter arithmetic routes Currency pairs through those checked
  operations. Inexact rescaling is rejected rather than rounded by an
  unverified policy.
- `ArrayValue::set` coerces to the declared element type before mutation.
  `ArrayValue::redim` supports preserving the final dimension of a
  multidimensional array, while retaining the existing multidimensional
  representation.
- `ArrayBound` now exposes dynamic/valid-bound queries, and `TypeInfo` exposes
  additive builders/accessors for ByRef, array rank, and dimensions.
- `ProcedureParameter`, `ProcedureSignature`, and `VBType::Procedure` add
  typed callable metadata while preserving the older `VBType::Function` and
  `VBType::Sub` APIs. These are descriptors only; parser and execution wiring
  remain future work.
- `ValueStore`, `ValueSlot`, and `ByRefHandle` provide stable slots for
  `VBVariant` values and invalidate aliases on release.
- `vb6runtime::VbString` preserves UTF-16 code units, embedded NULs, and
  unpaired surrogates; it supports UTF-16 slicing and concatenation. It remains
  a standalone runtime type: `VBVariant::String(String)` and consumers of that
  public variant are unchanged, so invalid UTF-16 is not yet storable in a
  `VBVariant`.
- The approved future design was copied to [`MX-SYNTAX.md`](./MX-SYNTAX.md).
  No MX syntax, callback executor, suspended frame, or `Async/Await` behavior
  was implemented. The specification continues to distinguish `Nothing`,
  `Null`, and `Empty`.

## Deliberately deferred

- `VBObject::clone_box` is the fork's existing object-copy contract. Replacing
  it with shared object identity would change the public API and runtime
  behavior; no independent M3 identity table is wired into `VBVariant`.
- M3's opaque object table stores no object payload and is not a substitute for
  the fork's `VBObject` model. `ValueStore` provides primitive ByRef slot
  semantics, but VM/interpreter argument-frame wiring remains a separate task.
- `VBType::Function` still carries only a return type. Procedure parameter
  signatures, closure captures/lifetimes, generics, task callback dispatch,
  deterministic cleanup, and spread lowering need coordinated parser,
  semantic, interpreter, and runtime designs. This migration does not invent
  those implementations.
- UDT layout and cleanup and enum member declarations/validation are not
  provided by M3's runtime types. Existing `UserType(String)` and `Enum(String)`
  identities remain unchanged.
- No SAFEARRAY or COM behavior is added. `ReDim Preserve` is implemented only
  for changing the upper bound of the last dimension, matching the supported
  preserve shape in the existing multidimensional storage.

## Changelog

| M3 source symbol | Fork symbol/API | Change |
|---|---|---|
| Static Decimal VarType code | `VBType::Decimal`, `VBType::from_var_type`, `var_type` | Added distinct static identity and tests; corrected former Currency alias |
| `VbDecimal` | `VBDecimal`, `VBVariant::Decimal`, `from_decimal`, `as_vbdecimal` | Adapted coefficient/scale representation; added checked conversions and decimal formatting; arithmetic remains absent |
| `Currency::checked_add/sub/neg/mul/div` | `VBCurrency::checked_*`, interpreter `arith` | Adapted to `VBError`; exact rescaling only for multiply/divide |
| `VbString` UTF-16 operations | `vb6runtime::VbString` in `vbstring.rs` | Adapted and exported as an independent runtime string type; not yet a `VBVariant` storage alternative |
| `VbArray` typed writes and `redim` | `ArrayValue::set`, `ArrayValue::redim` | Adapted type coercion to existing `VBVariant`; extended Preserve to final dimension without replacing multidimensional arrays |
| `ValueSlot`, `ByRefHandle`, `ValueStore` | `vb6runtime::reference` | Adapted storage from `VbValue` to `VBVariant`; not wired to VM call frames |
| Static array bounds and parameter signatures | `ArrayBound`, `TypeInfo`, `ProcedureSignature`, `VBType::Procedure` | Added rank/bound helpers and additive callable metadata; no parser, inference, generic or closure implementation |
| `VbObjectRef`, `VbObjectTable` | None | Not imported: opaque IDs without payload do not integrate with `VBObject::clone_box` identity semantics |
| `NumericOperation` and conversion helpers | Existing `VBVariant` conversions and interpreter operators | Existing ties-to-even/range checks retained; Currency arithmetic now checked; no second numeric value hierarchy added |

## Validation status

**PARTIALLY VERIFIED.** The retry of `cargo fetch --locked` succeeded without
changing manifests or the lockfile. `cargo test --locked -p vb6core` passed
(16 tests), and `cargo test --locked -p vb6interpret` passed (207 tests).
Focused runtime filters passed for Currency (9), Decimal (7), arrays (141),
ByRef (2), and UTF-16 (1). The complete runtime suite compiled and ran, with
1,991 passed and 18 failures in resource-fixture/state/settings tests; several
`.res` fixture files are missing in this checkout, so that suite is not
entirely green.

`cargo clippy --locked -p vb6core -p vb6runtime -p vb6interpret --all-targets`,
Rust 2024 formatting checks for all changed Rust files, and `git diff --check`
passed. The full workspace test command is blocked while compiling the
`vb6parse` `mod` test because many referenced `test-data` files are absent.
Cargo also reports pre-existing warnings that this stable toolchain does not
support workspace `lints.cargo` manifest keys. No manifest or lockfile was
changed. No commits were created because the complete runtime/workspace test
sets are not green.
