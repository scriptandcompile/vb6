# Changelog

## Unreleased

### Blocked
- MX S06/S07 runtime support remains **BLOCKED**. `RunError` wraps a
  `VBError`, but the interpreter has no catch/unwind contract to preserve a
  primary error across cleanup. `VBObject` has no `Dispose` operation, and
  interpreter expression evaluation does not support member access/calls, so
  there is no existing resource protocol to invoke safely. No default/no-op
  disposal behavior was introduced.

- Added the `VBVariant::Decimal` subtype backed by `VBDecimal` coefficient and
  scale, with formatting and checked Decimal-to-integer/Currency conversions.
  Full Decimal arithmetic and complete VB6 Decimal bounds are not implemented.
- Added checked fixed-point `VBCurrency` arithmetic; multiplication/division
  reject inexact rescaling instead of applying an unverified rounding policy.
- `ArrayValue::set` now coerces values to the declared element type before
  mutation. Added multidimensional `ArrayValue::redim` with Preserve support
  for changing the final dimension only.
- Added `ValueStore`, `ValueSlot`, and `ByRefHandle` for stable Variant slots.
  These primitives are not wired to interpreter/VM call frames.
- Added exported UTF-16 `VbString` storage and operations that preserve NUL
  code units and unpaired surrogates. It remains separate from the existing
  `VBVariant::String(String)` representation.
- Added Decimal handling to `TypeName`, RGB conversion, and interpreter-side
  Variant display so the new Variant tag is covered at those existing API
  boundaries.
- Made the Unix clock error-number retrieval portable across Unix targets and
  imported the request types used by the native macOS interaction backend,
  unblocking compilation on the current host.

### Validation

The runtime compiled as part of the focused package tests and Clippy. Filtered
runtime tests passed for Currency (9), Decimal (7), arrays (141), ByRef (2),
and UTF-16 (1). The complete runtime unit suite ran 2,009 tests: 1,991 passed
and 18 failed in resource-fixture/state/settings tests outside this migration;
some `.res` fixture files are absent in this checkout. The complete runtime
suite is therefore **not green**. Rust 2024 formatting checks for changed
files, `git diff --check`, and Clippy passed.
