# Changelog

## Unreleased

- Add a typed HIR API for VB6 module procedures, parameter signatures,
  declarations, literals, names, arithmetic expressions, and assignments.
  HIR retains CST byte spans and stable scope/name symbol identities; semantic
  errors and statements not yet lowered prevent a compilable program result.
- Lower MX `Return <expr>` into typed HIR, validate it against the declared
  Function result type using VB6 assignment rules, and represent permitted
  widening and runtime `Variant` conversions explicitly. Reject expression
  returns in Subs.
- Lower MX `AndAlso`/`OrElse` into a distinct lazy HIR expression with Boolean
  result type and explicit VB6 Boolean conversions. Preserve `Null`/`Empty`
  operand types for runtime conversion and reject statically unsupported object
  operands.
- Lower structured `If`, `While`, `Do`, and counted `For` control flow, and bind
  typed calls with arity and argument compatibility checks.
- Resolve VB6 symbol names case-insensitively during scope lookup and duplicate
  declaration checks.
- Document the current VB6 HIR lowering boundary and reserved, unimplemented
  MX HIR forms.
- Add opt-in `As Auto` inference for supported numeric, string, Boolean, and
  `Nothing` literal initializers. Missing/unsupported initializers, parameters,
  and use without the MX option produce explicit diagnostics; legacy untyped
  declarations remain `Variant`.
- Add partial `Option Strict On` HIR checks for known typed assignments.
  VB6 widening assignments and assignments to `Variant` are accepted; narrowing
  and `Variant`-to-typed assignments are rejected. Unknown types, array
  assignments, and call/member expressions remain explicitly unsupported.
- Preserve distinct HIR types for `Nothing`, `Null`, and `Empty`.
- Lower MX `??` to a lazy HIR node. Infer the fallback type for `Nothing`,
  preserve `Null` and `Empty` as distinct result types, retain dynamic `Variant`
  results, and select a widening-compatible object type or `Variant` fallback.
  The HIR node is not yet connected to an HIR executor.
- Lower MX `Continue For/Do` to typed HIR and reject a continue statement when
  no matching active loop exists. Preserve `Do` versus `While` loop identity so
  `Continue Do` can skip a nested `While` for an outer `Do` without targeting a
  `While` itself.

### Validation

`cargo test --offline -p vb6semantic --test mx_s04_s08` passed (6 tests);
`cargo test --offline -p vb6semantic --test mx_s01_return` passed (4 tests);
`cargo test --offline -p vb6semantic --test mx_s03_short_circuit` passed (3 tests);
`cargo test --offline -p vb6semantic` passed (unit, integration, and doc tests).
`cargo test --offline -p vb6semantic --test mx_s10_coalesce` passed (3 tests);
the full semantic suite passed again with coalescing coverage. Parser expression
tests passed (41 tests), and `cargo test --offline -p vb6interpret` passed.
`cargo clippy --offline --lib -p vb6semantic --no-deps` completed successfully;
it reports 21 existing lint warnings in the semantic crate.
`cargo test --offline -p vb6semantic --test mx_s02_continue` passed (5 tests);
the complete semantic and interpreter suites passed after the loop HIR change.
