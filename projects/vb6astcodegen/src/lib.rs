//! Offline reconstruction of numeric VBA6 AST emission. No DLL or Ghidra IR execution.
//! Inputs are already bound, typed, and lowered nodes. See README for unresolved domains.
pub mod addressing;
pub mod assignment;
pub mod ast_catalog;
pub mod cleanup;
pub mod conversions;
pub mod emission_rules;
pub mod emitter;
pub mod lowering;
pub mod lowering_hints;
pub mod member_calls;
pub mod references;
pub mod statement_catalog;
pub mod tables;
pub use emitter::{
    Ast, Bindings, Emission, EmitError, Instruction, Literal, Member, Node, NodeId, emit_ast,
};
pub use lowering::{
    BuiltinBindings, LoweredAst, LoweringError, LoweringTrace, ResolvedBuiltin,
    emit_ast_with_builtin_lowering, lower_known_builtins,
};
