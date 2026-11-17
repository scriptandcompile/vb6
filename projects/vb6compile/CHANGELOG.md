# Changelog

## Unreleased

- Remove the VB6R-defined classic opcode set and broad lowering introduced in
  commit `7711b569`.
- Keep `compile_source`, `compile_hir`, and `PCodeBuilder`, but constrain code
  generation to the verified HIR shape `Function Add(a As Long, b As Long) As
  Long: Add = a + b: End Function`.
- Emit the Microsoft Primary-table identities `ILdRf` (`0x6C`), `AddI4`
  (`0xAA`), `FStR4` (`0x71`), and `ExitProc` (`0x14`) with symbolic modern
  frame-slot operands, source maps, procedure metadata, and validated typed
  stack contracts.
- Diagnose all other HIR forms rather than substituting a VB6R opcode. No VM
  execution, byte-stream serializer, native backend, or MX lowering is claimed.
- Document disassembly/catalog evidence and semantic limits in
  [`docs/integration/HIR-PCODE.md`](../../docs/integration/HIR-PCODE.md).

### Validation

`cargo test --offline -p vb6compile --test hir_to_pcode` passes the exact
Microsoft-identity golden test and the negative unsupported-shape test.
Execution compatibility remains unverified because this change does not add a
VM.
