# Changelog

## Unreleased

- Added `VBType::Decimal` as a distinct type from `Currency` and corrected
  `VarType` 14 decoding/encoding, including array element codes.
- Unified `TypeInfo::can_assign_to` with the existing `VBType` assignability
  rules.
- Added static `ProcedureSignature`, `ProcedureParameter`,
  `ParameterPassingMode`, and `VBType::Procedure` metadata for typed callable
  values. Existing `VBType::Function` and `VBType::Sub` remain available;
  parser and execution support are not included.
- Added `ArrayBound` validity/dynamic-bound helpers and `TypeInfo` helpers for
  ByRef metadata, array rank, and dimensions.
- Added unit tests for Decimal identity/mappings/assignment, procedure
  signatures, bounds and ByRef/array metadata.

### Validation

`cargo test --locked -p vb6core` passed (16 tests). `cargo clippy --locked
-p vb6core -p vb6runtime -p vb6interpret --all-targets`, Rust 2024 formatting
checks for the changed files, and `git diff --check` passed. Cargo emitted
existing workspace warnings for unsupported `lints.cargo` manifest keys.
