# HIR → Microsoft P-Code: experimental native-AST path

The current compiler tranche accepts one `Function` with two non-optional Long
parameters and one assignment to its implicit result. Long expressions can
contain parameter references, decimal integer literals widened to Long,
parentheses, unary `+`, unary `-`, `Not`, and binary `+`, `-`, `*`, `\`, `Mod`,
`^`, `And`, `Or`, and `Xor`. Boolean-returning functions can return comparisons
between Long expressions (`=`, `<>`, `<`, `<=`, `>`, `>=`), for example:

```vb
Function Add(a As Long, b As Long) As Long
    Add = Not (a * 2) Xor (b Mod 7)
End Function
```

```vb
Function IsGreater(a As Long, b As Long) As Boolean
    IsGreater = a > b
End Function
```

Each parameter reference emits `ILdRf`; each literal emits `LitI4`; operators
emit the type-specific integer opcode in left-to-right postorder. Default
Integer literals are accepted only when the HIR contains a widening conversion
and the value fits `i16`. The Function result store and matching return opcode
follow the expression (`FStR4`/`ExitProc` for Long and
`FStI2`/`ExitProcI2` for Boolean). Microsoft dispatch-table identities are
emitted in the modern `vb6pcode::program` container. No MX opcode is emitted;
the container reserves a separately versioned extension namespace.

## Emitted opcodes

Most identities below are unprefixed Primary-table slots; the recovered native
AST selector selects Lead0 `0x13` for `XorI4`. Lead0 `0x11`, `0x12`, and `0x13` are
catalogued aliases of the same handler. The old `vb6-r` disassembly report records the addition sample as
`6C 0C 00 6C 10 00 AA 71 78 FF 14 00` and documents handlers in the original
`MSVBVM60.DLL` whose SHA-256 is
`e043c59c66ba2dc549bf5ac104c70279ec05931d531de2971458eab91871f7b6`.
The compiler does not copy that sample's x86 frame displacements into its
program; it uses typed symbolic frame slots instead.

| Order | Microsoft identity | Encoded form in Microsoft P-Code | Stack effect | Lowered meaning and evidence |
|---:|---|---|---|---|
| 1 | Primary `0x6C` `ILdRf` | `%a`, 3 bytes | 0 → 1 DWORD | Load parameter slot `a`. Catalog: [`opcodes.csv`](../../projects/vb6pcode/data/opcodes.csv), row 205. M4 handler report: [`WS-A-HANDLERS.md`](../../../vb6-r/research/m5/pcode/WS-A-HANDLERS.md), lines 24–40. |
| 2 | Primary `0x6C` `ILdRf` | `%a`, 3 bytes | 0 → 1 DWORD | Load parameter slot `b`; same handler contract and sources as above. |
| 3 | Primary `0xF5` `LitI4` | `%4`, 5 bytes | 0 → 1 DWORD | Push a signed 32-bit Long literal. Catalog: [`opcodes.csv`](../../projects/vb6pcode/data/opcodes.csv), row 342. |
| 4 | Primary `0xAA` `AddI4` | no operand, 1 byte | 2 DWORDs → 1 DWORD | Signed 32-bit addition with overflow path. Catalog: [`opcodes.csv`](../../projects/vb6pcode/data/opcodes.csv), row 267. M4 handler report: [`WS-A-HANDLERS.md`](../../../vb6-r/research/m5/pcode/WS-A-HANDLERS.md), lines 24–40. |
| 5 | Primary `0xAE` `SubI4` | none, 1 byte | 2 → 1 DWORD | Signed 32-bit subtraction with overflow path. Catalog row 271. |
| 6 | Primary `0xB2` `MulI4` | none, 1 byte | 2 → 1 DWORD | Signed 32-bit multiplication with overflow path. Catalog row 275. |
| 7 | Primary `0xB8` `UMiI4` | none, 1 byte | 1 → 1 DWORD | Signed 32-bit negation; may overflow on `INT_MIN`. Catalog row 281. |
| 8 | Primary `0xC0` `IDvI4` | none, 1 byte | 2 → 1 DWORD | Signed Long integer division. Catalog row 289. |
| 9 | Primary `0xC2` `ModI4` | none, 1 byte | 2 → 1 DWORD | Long remainder; division-by-zero path. Catalog row 291. |
| 10 | Primary `0xC3` `NotI4` | none, 1 byte | 1 → 1 DWORD | Bitwise complement of the Long stack value. Catalog row 292. |
| 11 | Primary `0xC4` `AndI4` | none, 1 byte | 2 → 1 DWORD | Bitwise Long AND. Catalog row 293. |
| 12 | Primary `0xC5` `OrI4` | none, 1 byte | 2 → 1 DWORD | Bitwise Long OR. Catalog row 294. |
| 13 | Lead0 `0x13` `XorI4` | prefix + slot, 2 bytes | 2 → 1 DWORD | Bitwise Long XOR; selected by FAC2A60 → FAC2090; alias of the same handler as Lead0 `0x11`/`0x12`. Catalog row 372. |
| 14 | Primary `0xC7` `EqI4` | none, 1 byte | 2 DWORDs → 1 Boolean | Long equality; produces VB Boolean false/true. Catalog row 296. |
| 15 | Primary `0xCC` `NeI4` | none, 1 byte | 2 DWORDs → 1 Boolean | Long inequality. Catalog row 301. |
| 16 | Primary `0xD1` `LtI4` | none, 1 byte | 2 DWORDs → 1 Boolean | Signed Long less-than. Catalog row 306. |
| 17 | Primary `0xD6` `LeI4` | none, 1 byte | 2 DWORDs → 1 Boolean | Signed Long less-than-or-equal. Catalog row 311. |
| 18 | Primary `0xDB` `GtI4` | none, 1 byte | 2 DWORDs → 1 Boolean | Signed Long greater-than. Catalog row 316. |
| 19 | Primary `0xE0` `GeI4` | none, 1 byte | 2 DWORDs → 1 Boolean | Signed Long greater-than-or-equal. Catalog row 321. |
| 20 | Primary `0x70` `FStI2` | `%a`, 3 bytes | 1 Boolean → 0 | Store the Boolean Function result as a two-byte value. Catalog row 209. |
| 21 | Primary `0x15` `ExitProcI2` | none, 1 byte | 0 → 0 | Return a Boolean/Integer-sized result. Catalog row 118. |
| 22 | Primary `0x71` `FStR4` | `%a`, 3 bytes | 1 DWORD → 0 | Store the Long Function result in the symbolic result slot. Catalog: [`opcodes.csv`](../../projects/vb6pcode/data/opcodes.csv), row 210. M4 handler report: [`WS-A-HANDLERS.md`](../../../vb6-r/research/m5/pcode/WS-A-HANDLERS.md), lines 24–40. |
| 23 | Primary `0x14` `ExitProc` | no operand, 1 byte | 0 → 0 | Return from the Long Function after its result slot has been written. Catalog: [`opcodes.csv`](../../projects/vb6pcode/data/opcodes.csv), row 117. M4 handler report: [`WS-A-HANDLERS.md`](../../../vb6-r/research/m5/pcode/WS-A-HANDLERS.md), lines 24–40. |

The operand forms and primary/lead-byte distinction are described in the
[Avast P-Code disassembly article](https://www.gendigital.com/blog/insights/research/vb6-p-code-disassembly?utm_source=chatgpt.com).
The compiler's normalized operands are `Parameter(0)`, `Parameter(1)`, and
`FunctionResult`; they are not `%a` x86 displacements or PE/COM addresses.

## Evidence boundaries

- The `vba6-ast-codegen` crate contains the recovered FAC2B97/FAC2A60
  selector tables and FAC2090 logical-opcode map. `vb6compile` now selects
  operators from each HIR expression's native AST kind and resolved type IDs
  through those tables; the separate `vb6compile::a2a` bridge has been removed.
- The native-AST emitter produces a raw byte stream from a fully bound numeric
  AST, while `compile_source` currently emits the repository's typed
  `vb6pcode::program::Program` directly. The emitter and HIR-to-native-AST
  binder are not yet connected as one end-to-end path.
- `vb6pcode::logical_opcode_map` retains the FAC2090 prefix table (1095 logical
  entries) and resolves Primary versus Lead0–Lead4 identities. Its tests cover
  representative direct and prefixed translations; it is not evidence that
  every resulting slot has a source-level lowering.
- The oracle requires operands to have been emitted in mode 2, a resolved
  Variant slot where relevant, and a later FAC0B67 result adaptation. Those
  surrounding steps are not implied by the selected opcode.
- The opcode identities, names, operand formats, byte lengths, and table stack
  metadata are catalog observations. The M4 report cites disassembled handler
  behavior for this exact DLL hash: `ILdRf` reads a DWORD from the frame,
  `AddI4` adds two DWORDs with an overflow path, `FStR4` stores a DWORD to the
  frame, and `ExitProc` performs procedure exit/cleanup.
- In the old M4 implementation, these are metadata-only opcode descriptors;
  the M4 golden test executes profile-specific IR, not these Microsoft
  handlers. The reported opcode execution path is limited to a build-bound
  `F5/LitI4` case.
- The documented byte stream does not independently prove that source-level
  `Function Add(a As Long, b As Long)` maps to those exact frame positions.
- The test here validates the HIR-to-opcode identities, normalized operands,
  source map, and stack contracts. It does not execute P-Code or claim full
  Microsoft runtime compatibility.
- Only the listed Long expression operators and Long-to-Boolean comparisons
  over the two declared parameters and supported decimal integer literals are
  lowered. Floating-point and Variant operators, locals, branches, calls,
  loops, other conversions, and module-level declarations still produce
  diagnostics.

## Imported native-AST emitter

The supplied `rust_codegen` implementation is vendored as the
`vba6-ast-codegen` workspace crate. `vb6compile::native_ast_codegen` exposes
its numeric-AST-to-raw-P-Code emitter and its recovered tables. Its AST already
contains native kind/type IDs and its `Bindings` must be populated by a native
binding/address-preparation phase; it is not directly interchangeable with the
repository HIR or `vb6pcode::program::Program`.

Operator selection in the current `compile_source` path uses the imported
crate's typed selector and converts logical opcodes to Microsoft dispatch-table
identities. This generates a typed P-Code `Program` for the tested Long subset,
including operators whose AST association is inferred and approved. It does
not yet lower the full HIR into the imported crate's numeric AST or use its raw
emitter. Source binding, native address descriptors, call/reference metadata,
and statement/control-flow construction remain incomplete; the raw emitter
continues to reject missing native bindings and metadata explicitly.
## Owner decision: experimental native-AST backend

**Decision date:** 2026-10-14  
**Decision owner:** repository owner  
**Status:** selected for an unvalidated evaluation pass

The owner selected the recovered `vba6-ast-codegen` library as the experimental
backend direction. The intended experiment is to lower the repository's typed
HIR into the library's numeric native AST and use its raw AST-to-P-code emitter,
without requiring corpus validation before evaluating the approach. The typed
selector is now used by the current compiler subset; some native AST types and
all non-expression HIR forms still need a concrete lowering.

This choice is an implementation direction, not a claim that every HIR node
already has a faithful native representation. The current `PCodeBuilder`
returns its generated container without running `vb6pcode::program::validate`,
so successful construction is intentionally not validation evidence. The
imported emitter requires
native binding/reference/cleanup metadata for many nodes, and the separate
statement catalog does not yet implement procedure control-flow layout,
backpatching, or finalization. Until those bridges exist, compilation paths
must report unsupported or unresolved HIR explicitly rather than fabricate
successful output. Validation may be deferred for the owner's evaluation, but
must not be silently treated as evidence of correctness.
