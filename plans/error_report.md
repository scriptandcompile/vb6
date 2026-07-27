# Parser Error Reporting Gap Report

This report lists parser-related call sites where we should emit explicit `report_error(...)` diagnostics (or equivalent surfaced errors), but currently return/continue silently.

## 1) `parse()` drops parser failures entirely
- Location: `src/parsers/cst/mod.rs:505-507`
- Current behavior: `parse(tokens)` returns only `parser.parse_root().0` and discards `parser.parse_root().1` (all collected parser failures).
- Why this should report: Any `report_error(...)` emitted during CST parsing is lost through this API. Callers in file parsers cannot see parser diagnostics, so malformed syntax can appear as "successful" parse with no actionable error output.
- Affected call paths:
  - `src/files/class/mod.rs:122`
  - `src/files/module/mod.rs:161`
  - `src/files/module/mod.rs:193`
  - `src/files/form/mod.rs:112`

## 2) Direct VERSION parsing silently accepts malformed VERSION statements
- Location: `src/parsers/cst/mod.rs:888-933` (`parse_version_direct`)
- Current behavior:
  - Returns `ParseResult::new(None, Vec::new())` when `VERSION` is missing (`:893`) and also when a malformed version token is present (`_ => None`, parse failures in `major.minor` conversion).
  - Always returns `ParseResult::new(version_result, Vec::new())` (`:933`) with no diagnostics.
- Why this should report: If `VERSION` is present but malformed (wrong token type or invalid numeric format), this is parse-invalid input and should generate `report_error` with span context.

## 3) Control block parser fails silently when `BEGIN` is missing
- Location: `src/parsers/cst/mod.rs:1738-1744` (`parse_properties_block_to_control`)
- Current behavior: If the parser is not at `BEGIN`, it returns `ParseResult::new(None, Vec::new())`.
- Why this should report: Callers expect a control block parse at this point; missing `BEGIN` is structurally invalid and should produce an error rather than silent `None`.

## 4) Control depth overflow skips nested content without diagnostic
- Location: `src/parsers/cst/mod.rs:1802-1816` (`parse_properties_block_to_control`)
- Current behavior: When `stack.len() >= MAX_CONTROL_DEPTH`, parser skips nested control content until matching `End` and continues.
- Why this should report: This is a hard truncation of source structure. It should report a `NestingTooDeep`-style error (similar to statement depth handling at `:2535`) so users know controls were dropped.

## 5) Unknown tokens in direct control parsing are consumed without error
- Locations:
  - `src/parsers/cst/mod.rs:1849-1851` (`parse_properties_block_to_control`)
  - `src/parsers/cst/mod.rs:1948-1950` (`parse_properties_block_to_form_root`)
- Current behavior: Unknown tokens are consumed via `self.consume_advance()` with no diagnostic.
- Why this should report: In direct extraction mode, this can hide malformed property/control syntax. At least a recoverable parse error should be recorded for non-trivia unexpected tokens.

## 6) Form root parser fails silently when `BEGIN` is missing
- Location: `src/parsers/cst/mod.rs:1868-1878` (`parse_properties_block_to_form_root`)
- Current behavior: Returns `ParseResult::new(None, Vec::new())` when not at `BEGIN`.
- Why this should report: Form/control sections are required for this parse path; missing `BEGIN` should surface as a parse error.

## 7) Invalid top-level form control type silently downgraded to default form
- Location: `src/parsers/cst/mod.rs:1967-1988` (`parse_properties_block_to_form_root`)
- Current behavior: If `build_form_root(...)` fails (invalid root kind), parser returns a default empty `Form` without adding any failure.
- Why this should report: Replacing invalid root with fallback hides incorrect input and can mislead downstream tooling. Should emit explicit error before fallback.

## 8) Missing terminating `End` in direct form/control parsing can pass without explicit error
- Location: `src/parsers/cst/mod.rs:1954-1959` and related control parser loop in `:1759-1855`
- Current behavior: `End` is consumed only if present; reaching EOF without `End` does not emit an explicit missing-terminator diagnostic in direct extraction paths.
- Why this should report: Unterminated `Begin ... End` blocks are structural parse errors and should produce clear diagnostics.

## 9) File-level fast paths rely on direct extractors that currently under-report
- Locations:
  - `src/files/form/mod.rs:84-93`
  - `src/files/form/control_only.rs:118-122`
- Current behavior: These call `parse_version_direct()` and `parse_properties_block_to_form_root()` and propagate their failures, but those parsers often produce empty failure vectors for invalid structure.
- Why this should report: Fast-path parsing is user-facing API behavior; it should preserve useful diagnostics instead of silently defaulting/returning `None`.

## Notes
- The only current parser-side `report_error` call found is nesting depth protection in statement parsing: `src/parsers/cst/mod.rs:2535`.
- Several gaps above are likely intentional permissive behavior in early direct extraction code, but they still represent places where adding diagnostics would improve debuggability and correctness visibility.
