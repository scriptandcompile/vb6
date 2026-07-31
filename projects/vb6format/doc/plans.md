# Next Reformatting Feature Proposal

## Recommendation

The next reformatting feature should be **line-width-aware continuation formatting**.

Today `vb6format` already handles the basics well: indentation, keyword casing, directive spacing, top-level spacing, and blank-line deduplication. The biggest missing capability is deciding when a VB6 statement is too long and how to rewrite it into a stable multi-line form.

That makes continuation-aware wrapping the highest-value next step because it improves readability across a wide range of real VB6 code, not just in one narrow syntax area.

## Why this should come next

- It addresses the most obvious remaining formatting gap after the current pass pipeline.
- It creates a foundation for later features like comment wrapping, argument alignment, and expression grouping.
- It is a natural fit for the current CST-pass architecture because wrapping can be introduced as one more formatting pass without rewriting the whole formatter.

## Rustfmt ideas worth borrowing

Rustfmt is useful here because it treats formatting as a width-constrained rewriting problem rather than just a token cleanup problem.

Key ideas to borrow:

- **Shape / width budgeting**: keep track of the available width for the current line and choose a layout that fits.
- **Rewrite decisions instead of direct string edits**: try a compact form first, then fall back to a vertical form when the line is too wide.
- **Structured wrapping helpers**: rustfmt has dedicated logic for comments, chains, and expressions instead of one generic line-break rule.
- **Configuration-driven heuristics**: line width and small-item heuristics let the formatter be opinionated without being rigid.

## Suggested scope for the first version

Start with the VB6 constructs that give the biggest payoff and are easiest to stabilize:

- Long procedure calls and argument lists.
- Long `If ... Then ... Else` conditions.
- Long assignment expressions.
- Long chained member access or property access.
- Existing explicit continuation lines using `_`.

The first version should focus on **stable wrapping** rather than perfect alignment. A readable, deterministic split is more important than sophisticated column alignment at the start.

## Suggested behavior

- Add a `max_line_width` setting with a sensible default.
- Keep short statements on one line.
- When a statement exceeds the width budget, rewrite it into a multi-line form using continuation indentation.
- Prefer a consistent continuation style over per-case special casing.
- Preserve idempotence: formatting the output twice should not change it again.

## Implementation shape

A good first implementation would probably look like this:

1. Add width tracking to formatter state.
2. Introduce a small rewrite helper for "fits on one line" vs "must break" decisions.
3. Add a new pass that detects long expressions or statements and injects continuation breaks.
4. Keep blank-line and directive behavior in the existing passes so the new feature stays focused.
5. Add targeted tests for long calls, long conditions, and idempotence.

## Phased follow-ups

After continuation wrapping works, the next features should probably be:

- Comment reflow for long end-of-line comments and block comments.
- Alignment of wrapped argument lists or continuation blocks when it improves readability.
- Width-sensitive formatting for `With` blocks and nested expressions.
- A small set of preview heuristics, similar to rustfmt’s "small items" behavior, to avoid over-wrapping short constructs.

## Bottom line

If only one new formatting feature gets added next, it should be **width-aware continuation wrapping**. That gives `vb6format` a real layout engine instead of only normalization passes, and it opens the door to the rest of the higher-quality formatting work.
