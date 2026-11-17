# vb6pcode

Standalone Microsoft VB6 P-Code catalog and decoder. The crate embeds the six
256-slot tables, operand formats, instruction lengths, stack/x87 and memory
metadata, semantic categories, and the M1 slot classifications. It does not
depend on VB6R's VM, compiler, runtime, or the upstream repository.

The catalog represents Microsoft's P-Code namespace. Future MX instructions
may use only the explicitly versioned extension namespace in the modern
program container; they must not alter these six Microsoft dispatch tables.
See the repository's
[MX syntax specification](../../docs/integration/MX-SYNTAX.md).

The [`program` module](src/program.rs) defines a versioned, platform-independent
container with Microsoft table/slot identities, normalized operands, procedure
metadata, source maps, and stack validation. The current compiler emits
`ILdRf`, `LitI4`, implemented Long arithmetic/bitwise/comparison operators,
and the typed Long/Boolean result-store and return opcodes; see the
[opcode evidence and limits](../../docs/integration/HIR-PCODE.md).

The [`operator_selector` module](src/operator_selector.rs) separately ports
the recovered FAC2B97/FAC2A60 type-directed selector, and
[`logical_opcode_map`](src/logical_opcode_map.rs) resolves its logical IDs
through FAC2090 to Microsoft dispatch identities. The selector still expects
native AST/type IDs; connecting those to repository HIR remains future work.

See [catalog provenance and generation](docs/CATALOG.md) and
[the changelog](CHANGELOG.md).

Run the crate checks offline with:

```sh
cargo fmt --package vb6pcode --check
cargo test --offline -p vb6pcode
cargo clippy --offline -p vb6pcode --all-targets -- -D warnings
```
