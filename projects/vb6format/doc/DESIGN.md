# vb6format design overview

This document describes the current structure of vb6format as implemented in this repository.

## What vb6format is (and is not)

vb6format is currently a library crate with two public entry points:

- src/lib.rs: fmt_source(source, settings)
- src/lib.rs: fmt_cst(cst, settings)

It does not provide its own CLI in this crate. The aspen command integrates it from projects/aspen/src/fmt.rs.

## High-level architecture

The formatter is a CST walker with a pass pipeline:

1. Parse source into a ConcreteSyntaxTree using vb6parse (for fmt_source).
2. Walk the CST depth-first in src/cst_formatter.rs.
3. For each node/token, run a sequence of formatting passes in src/passes/mod.rs.
4. Each pass can mutate a shared formatting context and token buffer.
5. Emit the final String output.

This is token-stream rewriting with structural context from CST traversal, not an AST rewrite engine.

## Core modules and responsibilities

- src/lib.rs
	- Public API surface.
	- Converts parse result to CST and starts formatting.

- src/cst_formatter.rs
	- Owns the formatter runtime state: CST, output String, Context, and PassManager.
	- Recursively walks nodes.
	- Emits tokens through the pass pipeline.

- src/context.rs
	- Shared mutable state passed through all hooks.
	- Tracks indentation, line ending style, blank-line state, and directive state.

- src/settings.rs
	- User-configurable knobs:
		- indent_size
		- keyword_case
		- blank_lines_around_directives
		- blank_lines_inside_directives
		- blank_lines_around_top_level

- src/passes/mod.rs
	- Defines the pass interface (FormatPass), token buffer contract, and pass ordering.

- src/passes/*.rs
	- Concrete formatting passes.

## Walk model and formatting data flow

The walker in src/cst_formatter.rs does:

- on_node_enter for every non-token node
- recursive traversal of children
- on_node_exit for every non-token node
- on_token for every token node

Special case:

- When the current node kind is StatementList, the walker increments indent_level before visiting children and decrements after.

Token emission model:

- Each token starts as TokenBuffer { prefix: "", text: original token text, emit: true }.
- Passes can:
	- modify text (for casing/normalization),
	- add prefix (typically indentation or extra line breaks),
	- suppress emission (emit = false).
- If emit remains true, formatter appends prefix then text to output.

## Current pass pipeline and why order matters

Passes run in this fixed order (src/passes/mod.rs):

1. LineEndingPass
2. CompilerDirectivePass
3. KeywordCasePass
4. LayoutPass
5. TopLevelSpacingPass
6. DeduplicateBlankLinesPass

The order is part of behavior. Reordering can change output.

### 1) LineEndingPass

- Detects CRLF vs LF once from the root text.
- Rewrites Newline token text to the detected style.
- Updates per-line state (pending_indent, line_has_content, blank tracking).

### 2) CompilerDirectivePass

- Tracks compiler directive depth.
- Inserts requested blank lines around and/or inside #If blocks, depending on settings.
- Uses Context.pending_blank and Context.directive_phase to defer insertion until the next real token.

### 3) KeywordCasePass

- Rewrites keyword token text based on settings.keyword_case.
- Supported values are currently string-based: upper, lower, camel, first.

### 4) LayoutPass

- Collapses whitespace tokens to a single space unless indentation is pending.
- Applies indentation prefix when pending_indent is set.
- Suppresses parser error tokens (ErrorExpectedTokens, ErrorMissingTokens) from output.
- Marks line_has_content and clears pending indentation at the first emitted content on a line.

### 5) TopLevelSpacingPass

- Optional blank-line insertion between top-level constructs.
- Computes insertion points once from normalized root text via line-based heuristics.
- Treats leading comments as attached to the following top-level construct.
- Inserts line breaks as token prefixes at line starts.

### 6) DeduplicateBlankLinesPass

- Final sanitation pass over prefix and text chunks.
- Caps consecutive line endings to at most two.
- Prevents over-insertion when earlier passes emit extra blank lines.

## How tests define expected behavior

tests/common/mod.rs enforces two key properties:

- expected output correctness
- idempotence (formatting formatted output again must not change it)

Behavior coverage today includes:

- indentation and nested blocks: tests/indent.rs, tests/control_flow.rs, tests/type_enum.rs
- compiler directives: tests/compiler_directive.rs
- top-level blank lines: tests/top_level_spacing.rs
- keyword casing: tests/keyword.rs
- continuation lines: tests/continuation.rs
- comments: tests/comments.rs
- line ending preservation: tests/line_ending.rs
- blank-line deduplication: tests/deduplicate_blank_lines.rs
- single-line If stability: tests/single_line_if.rs

## Adding a new formatting pass

Use this workflow.

1. Create a new pass module under src/passes, for example src/passes/alignment.rs.
2. Add mod alignment; and use alignment::AlignmentPass; in src/passes/mod.rs.
3. Implement FormatPass for the new type.
4. Register it in PassManager::new in a deliberate order relative to existing passes.
5. If the pass needs options, extend FmtSettings in src/settings.rs (with a default).
6. Add tests in tests/<feature>.rs using common::assert_fmt or common::assert_fmt_with.
7. Ensure idempotence by formatting twice in tests (already built into helper assertions).

Implementation pattern for stateful passes:

- FormatPass methods receive &self, not &mut self.
- If pass-local mutable state is required across hooks/tokens, use interior mutability (Cell or RefCell), as done by LineEndingPass, TopLevelSpacingPass, and DeduplicateBlankLinesPass.

Order guidance:

- Passes that normalize raw token text (line endings, lexical casing) should generally run early.
- Passes that add spacing/newlines should run before deduplication.
- DeduplicateBlankLinesPass should stay last unless you intentionally redesign dedup behavior.

## Tricky and potentially confusing parts

### 1) Indentation is tied to StatementList traversal

Indent level changes come from CST structure, not keyword string matching in passes. If parser structure for a construct is different than expected, indentation behavior can surprise you.

### 2) Pass ordering is a hidden contract

There is no dependency graph between passes. The vector order in PassManager is the contract. Small reorderings can cause subtle changes.

### 3) TopLevelSpacingPass is line-heuristic based

It scans raw text lines and recognizes top-level constructs by first/second words. This is practical, but it is not full CST semantic classification and may need updates when new VB6 declaration forms are added.

### 4) Some settings are stringly typed

keyword_case is currently a free-form String with known values. Unknown values fall back to original token text. This is flexible but easy to misconfigure.

### 5) Error tokens are dropped by LayoutPass

ErrorExpectedTokens and ErrorMissingTokens are suppressed. That helps produce cleaner output for malformed input, but it also means output can lose parser error markers.

### 6) rewrite.rs is not part of active formatting flow

src/rewrite.rs currently defines helper enums/types but is not wired into the pass pipeline. When extending architecture, do not assume rewrite.rs drives formatting behavior today.

### 7) Parse failure surface is intentionally minimal

fmt_source returns an error when CST creation fails, but currently drops parse-failure details from vb6parse in the public error path.

## Practical change checklist

Before opening a formatter PR:

1. Verify output for representative VB6 snippets (including comments and directives).
2. Run tests for vb6format and keep idempotence passing.
3. Check interaction with line endings and blank-line deduplication.
4. Audit pass order impact explicitly.
5. If adding settings, document defaults and expected values in README.md.

Following this model keeps vb6format maintainable while expanding rule coverage.
