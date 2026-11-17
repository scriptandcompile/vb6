# Changelog

## Unreleased

- Add a standalone Microsoft P-Code catalog crate with six generated
  256-entry dispatch tables, full slot/M1 metadata, opcode descriptors, and a
  dependency-free decoder.
- Import the catalog, M1 matrix, evidence-qualified forms, dispatch map,
  Rust table generator, documentation, and required provenance/license files.
- Add a versioned, platform-independent P-Code program container whose
  instruction identities are Microsoft dispatch-table slots and whose
  operands are normalized Rust values.
- Remove the separate classic VB6R opcode set introduced by `7711b569`.
  Reserve a separately versioned MX extension namespace; no MX opcode is
  emitted in the current compiler tranche.
- Document the verified `Add(Long, Long)` identity/stack subset and its
  evidence boundaries in
  [`docs/integration/HIR-PCODE.md`](../../docs/integration/HIR-PCODE.md).
