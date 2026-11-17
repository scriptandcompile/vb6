# Upstream data notice

The opcode catalog data is derived from `ATRAPSLLC/visualbasic-rs`, version
`0.5.0`, revision `7885bb7096694c94f94801f2db4cb6e086fbee57`, licensed under
Apache-2.0. Its applicable license text is retained in
[`LICENSE-APACHE`](./LICENSE-APACHE).

The catalog was imported from the VB6R checkout's
`crates/vb6r-vm/data/opcodes.csv`; that asset carries the upstream attribution.
The table generator and decoder in this crate are self-contained and do not
import VM handlers, runtime, compiler, or cross-checkout dependencies.
