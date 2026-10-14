# Typed HIR status

`SemanticAnalyzer::analyze_module_hir` and `analyze_class_hir` reuse the analyzer's binding and
diagnostics, then returns `HirAnalysis`. `program` is present only when the
module has no blocking diagnostics. Nodes preserve byte spans in the `vb6parse`
CST, a file/line/column location, resolved `TypeInfo` where known, and a
`SymbolId` (`scope_id` plus a case-folded VB6 name) for bound declarations.
The CST remains the source of truth; HIR is a compiler-facing semantic view,
not an alternate parser or a dependency on `vb6interpret`.

Current HIR lowering covers module-level variable declarations, Sub/Function/
Property signatures (including `ByVal`, default `ByRef`, `Optional`, and
`ParamArray`), local `Dim` declarations, assignments, named procedure calls
with argument arity/type checks, names, literals, parenthesized values, and
basic binary expressions. Assignment compatibility is
checked through `TypeChecker`; supported implicit assignments and operand promotions are
explicit `Conversion` nodes. The current type checker is intentionally not a
complete VB6 language type system; a value that cannot be typed confidently is
not silently promoted to a made-up compiler type.

The HIR lowering supports structured `If`, `While`, `Do`, and numeric counted
`For ... Next` loops. `For Each` and `Select Case` still produce a blocking
diagnostic: the parser's current CST does not expose their operands in a form
this pass can reliably type-check. The HIR has reserved shapes for
lambdas,
spread, `Try/Catch/Finally`, short-circuit operators, and task continuations.
These MX forms are not enabled by this lowering pass. Their source forms, as well as
member access, array semantics, object construction, and other VB6
statements not listed above, produce diagnostics and no compilable HIR. This
keeps partial lowering from being mistaken for compiler-ready output.

The MX syntax design in
[`docs/integration/MX-SYNTAX.md`](../../../docs/integration/MX-SYNTAX.md) is forward-looking.
MX syntax is not enabled here: lambda/closure capture, generics, spread,
`AndAlso`/`OrElse`, `Try/Catch/Finally`, and `VBTask.Then/Catch` require separate
semantic contracts and are not treated as VB6 constructs. Legacy VB6 `And` and
`Or` are not rewritten to short-circuit operators.
