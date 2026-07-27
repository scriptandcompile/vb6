# CST Error Handling Improvements for Malformed VB6 Input

## Problem Statement
Current behavior primarily emits `unknown` tokens with minimal diagnostic detail. This keeps parsing alive, but it is not compiler-grade recovery and does not provide enough actionable feedback for users or downstream tools.

## Goals
- Preserve a full-fidelity CST even when input is malformed.
- Recover aggressively so parsing continues after local failures.
- Emit precise, structured diagnostics with stable codes.
- Minimize cascading/noisy errors from one root fault.
- Keep all changes compatible with existing `ParseResult` patterns.

## How High-Quality Compilers Typically Handle CST Errors
1. **Maintain structure**: Always produce a CST, including malformed regions.
2. **Represent failure in-tree**: Add explicit error nodes, not just unknown leaf tokens.
3. **Recover quickly**: Skip forward to synchronization points and resume parsing.
4. **Insert missing expected tokens**: Represent virtual/missing tokens in error nodes.
5. **Emit rich diagnostics**: Include expected vs found, exact span, and recovery action.
6. **Throttle cascades**: Limit secondary diagnostics that are likely downstream noise.

## Recommended Model for VB6Parse

### 1) Improve lexer-level unknown token handling
Replace the generic `unknown` category with typed invalid token variants where possible:
- `UnknownCharacter`
- `UnterminatedString`
- `InvalidNumberLiteral`
- `InvalidDirective`

These should still preserve source text in CST/token stream while producing first-pass diagnostics.

### 2) Add parser-level error productions
Introduce dedicated CST nodes for malformed constructs:
- `ErrorStatement`
- `ErrorExpression`
- `MissingToken` (e.g., missing `)`)
- `SkippedTokens`
- Block-specific missing closers such as `MissingEndIf`

This allows downstream tools to reason about broken regions without reverse-engineering token noise.

### 3) Introduce panic-mode recovery with VB6 sync sets
When a grammar rule fails, skip tokens until a synchronization point, then continue.

Suggested sync sets:
- **Statement-level**: newline, `:`
- **Conditional blocks**: `Else`, `ElseIf`, `End If`
- **Select blocks**: `Case`, `Case Else`, `End Select`
- **Loop blocks**: `Next`, `Loop`, `Wend`
- **Procedure blocks**: `End Sub`, `End Function`, `End Property`

### 4) Enrich diagnostics payloads
Standardize `ErrorDetails` payload extensions (or mapped adjunct type) to include:
- Stable error code (e.g., `VB6P1001`)
- Severity (`error`, `warning`, possibly `note`)
- Primary span
- Expected token set
- Found token kind/text
- Recovery action taken (e.g., "skipped until End If")
- Optional fix hint

### 5) Suppress low-value cascades
Implement simple anti-cascade heuristics:
- If two diagnostics occur at same offset/kind, emit only one.
- Require meaningful parser progress before allowing additional similar diagnostics.
- Cap repeated diagnostics per statement/block.

## VB6-Specific Recovery Recommendations
- Treat line continuation (`_`) failures as recoverable at line boundaries.
- Treat unterminated strings/comments as recoverable at newline or statement boundary.
- Use block-aware recovery for:
  - `If ... Then ... Else ... End If`
  - `Select Case ... End Select`
  - `For ... Next`
  - `Do ... Loop`
  - `With ... End With`
- Use separate synchronization policy in header/declaration sections (`VERSION`, `Attribute`, `Option`) vs executable code bodies.

## Migration Path (Incremental, Low Risk)

### Phase 0: Baseline and observability
- Add malformed fixture corpus representative of user-reported failures.
- Snapshot current CST + failures behavior as baseline.
- Add basic metrics (failure count, first failure location, parse completion rate).

### Phase 1: Lexer diagnostic quality
- Split generic unknown tokens into typed invalid token variants.
- Emit lexer diagnostics with stable codes and spans.
- Keep parser behavior unchanged for this phase.

### Phase 2: Generic parser error nodes
- Add a generic `ErrorNode`/`ErrorStatement` and `SkippedTokens` representation.
- Add panic-mode recovery at statement boundaries (newline/colon).
- Ensure parser never panics on malformed input.

### Phase 3: Missing-token insertion
- Introduce `MissingToken` representation for common expectations:
  - `)`
  - `Then`
  - block terminators (`End If`, `End Select`)
- Update diagnostics to include expected-vs-found and insertion notes.

### Phase 4: Block-aware recovery
- Add dedicated sync sets for `If`, `Select`, loops, and procedure endings.
- Improve error locality so first diagnostic better matches root fault.

### Phase 5: Cascade suppression and polish
- Add anti-cascade deduping and suppression windows.
- Tune messages and fix hints for common VB6 mistakes.
- Add regression tests for noisy/multi-error files.

## Testing Strategy
- Continue `insta` snapshots for both:
  - CST structure (with error nodes)
  - Diagnostic list content (codes, spans, recovery notes)
- Add invariants for malformed inputs:
  - Parser returns CST and does not panic.
  - `ParseResult` retains both partial result and failures.
  - First diagnostic spans remain stable across refactors (where feasible).
- Extend fuzz checks to assert:
  - No parser panic
  - Bounded diagnostics (avoid unbounded growth)

## Suggested Error Taxonomy (Starter)
- `VB6P1xxx`: Lexical errors
- `VB6P2xxx`: Statement parse errors
- `VB6P3xxx`: Expression parse errors
- `VB6P4xxx`: Block structure/terminator errors
- `VB6P9xxx`: Recovery notes/internal parser recovery events

Example diagnostics:
- `VB6P2003`: Expected `Then` after `If` condition; found `Identifier`.
- `VB6P4001`: Missing `End If` before `End Sub`; inserted virtual terminator.
- `VB6P1002`: Unterminated string literal; recovered at end of line.

## Suggested Implementation Steps (Concrete)
1. Add new token variants in lexer and map them to diagnostic constructors.
2. Add `SyntaxKind` entries for `ErrorStatement`, `ErrorExpression`, `MissingToken`, `SkippedTokens`.
3. Implement `recover_to(sync_set)` helper in parser core.
4. Wire statement parser to call recovery helper on hard failures.
5. Add missing-token insertion for `)` and `Then` first (high-value/low-risk).
6. Introduce stable error-code enum and mapping to printable messages.
7. Add/refresh snapshot fixtures for each new recovery path.
8. Add dedupe/suppression pass for diagnostics.

## Operational Guidance
- Keep error recovery deterministic to preserve snapshot stability.
- Prefer token-stream-driven recovery over raw text heuristics.
- Keep rowan internals hidden from public API, exposing only CST abstractions and diagnostics.
- Continue using `ParseResult` accessors (`unpack`, `failures`, etc.) and avoid `unwrap()` patterns.

## Success Criteria
- Malformed files still produce navigable CST with explicit error regions.
- Diagnostic quality improves from generic "unknown token" to actionable messages.
- Number of cascading false-positive diagnostics drops significantly.
- Fuzz and malformed fixture suites run panic-free.
- Downstream tooling can identify and isolate syntax failure areas using CST error nodes.

## Further Suggestions
- Add a CLI/examples mode to print diagnostics grouped by statement/block context.
- Add optional "strict mode" that elevates selected recoverable issues to hard errors for conversion pipelines.
- Add developer docs page describing recovery strategy and error code catalog.
- Track and publish parser quality metrics over time (first-error accuracy, average errors/file, recovery completion rate).
