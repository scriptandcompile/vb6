//! Known runtime-builtin lowering. This is not the entire binder or FAEC8D3 optimizer.
//! The runtime symbol/signature is a resolved input, not a fabricated pool index.
use crate::{
    Ast, Bindings, Emission, EmitError, Node, NodeId, emit_ast,
    lowering_hints::builtin_lowering_hint, member_calls::ResolvedMemberMetadata,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy)]
pub struct ResolvedBuiltin {
    /// Native symbol returned by FAEDF32, represented by the caller's handle.
    pub symbol: u32,
    /// Native result type of the builtin signature, before FAEE137 adjustment.
    pub signature_result_type: u16,
    pub member_metadata: ResolvedMemberMetadata,
}
pub type BuiltinBindings = BTreeMap<u16, ResolvedBuiltin>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweringError {
    MissingNode(NodeId),
    MissingChild(NodeId),
    MissingBuiltin {
        node: NodeId,
        builtin: u16,
    },
    MissingBoundSymbol(NodeId),
    UnresolvedTypeAdaptation {
        node: NodeId,
        source_type: u16,
        destination_type: u16,
    },
    Cycle(NodeId),
    RecursionLimit,
    Emission(EmitError),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringTrace {
    pub node: NodeId,
    pub source_kind: u16,
    pub builtin_id: u16,
    pub original_function: u32,
}
#[derive(Debug, Clone)]
pub struct LoweredAst {
    pub ast: Ast,
    pub bindings: Bindings,
    pub trace: Vec<LoweringTrace>,
}

/// Lower reachable 3C/3D/40/5B nodes, preserving caller input and shared-node IDs.
/// Existing bound/lowered nodes are retained. Other intermediate tags need the binder.
pub fn lower_known_builtins(
    ast: &Ast,
    root: NodeId,
    bindings: &Bindings,
    builtins: &BuiltinBindings,
) -> Result<LoweredAst, LoweringError> {
    let mut out = LoweredAst {
        ast: ast.clone(),
        bindings: bindings.clone(),
        trace: vec![],
    };
    walk(
        &mut out,
        root,
        builtins,
        &mut BTreeSet::new(),
        &mut BTreeSet::new(),
    )?;
    Ok(out)
}
pub fn emit_ast_with_builtin_lowering(
    ast: &Ast,
    root: NodeId,
    mode: i32,
    bindings: &Bindings,
    builtins: &BuiltinBindings,
) -> Result<(Emission, Vec<LoweringTrace>), LoweringError> {
    let lowered = lower_known_builtins(ast, root, bindings, builtins)?;
    let emitted =
        emit_ast(&lowered.ast, root, mode, &lowered.bindings).map_err(LoweringError::Emission)?;
    Ok((emitted, lowered.trace))
}
fn walk(
    out: &mut LoweredAst,
    id: NodeId,
    builtins: &BuiltinBindings,
    active: &mut BTreeSet<NodeId>,
    done: &mut BTreeSet<NodeId>,
) -> Result<(), LoweringError> {
    if done.contains(&id) {
        return Ok(());
    }
    if !active.insert(id) {
        return Err(LoweringError::Cycle(id));
    }
    if active.len() > 256 {
        return Err(LoweringError::RecursionLimit);
    }
    let old = out
        .ast
        .nodes
        .get(id)
        .cloned()
        .ok_or(LoweringError::MissingNode(id))?;
    for child in [old.left, old.right].into_iter().flatten() {
        walk(out, child, builtins, active, done)?;
    }
    if let Some(hint) = builtin_lowering_hint(old.kind) {
        let bound =
            builtins
                .get(&hint.builtin_id)
                .copied()
                .ok_or(LoweringError::MissingBuiltin {
                    node: id,
                    builtin: hint.builtin_id,
                })?;
        let desired = if old.kind == 0x5b { 2 } else { old.type_id };
        let source = bound.signature_result_type;
        // FAEE137: four equivalent native-type pairs retain the source type.
        let result_type = if matches!((source, desired), (12, 11) | (11, 12) | (3, 6) | (6, 3)) {
            source
        } else if source == 23 && desired != 2 && desired != 23 {
            return Err(LoweringError::UnresolvedTypeAdaptation {
                node: id,
                source_type: source,
                destination_type: desired,
            });
        } else {
            desired
        };
        let mut args = old.right;
        if old.kind == 0x3d {
            // FAEC8D3: missing first argument becomes native type8 literal0.
            let cell = args.ok_or(LoweringError::MissingChild(id))?;
            if out
                .ast
                .nodes
                .get(cell)
                .ok_or(LoweringError::MissingNode(cell))?
                .left
                .is_none()
            {
                let zero = out.ast.push(Node::integer(8, 0));
                out.ast.nodes[cell].left = Some(zero);
            }
        } else if old.kind == 0x5b {
            let target = old
                .symbol
                .and_then(|h| out.bindings.bound_nodes.get(&h).copied())
                .ok_or(LoweringError::MissingBoundSymbol(id))?;
            walk(out, target, builtins, active, done)?;
            // FB8EBEC -> FAB50CA: copied type descriptor becomes native type23.
            let wrapper = out.ast.push(Node {
                kind: 0x11,
                type_id: 23,
                left: Some(target),
                ..Node::default()
            });
            if out.ast.nodes[target].kind == 0x31 {
                let mut cursor = target;
                let mut seen = BTreeSet::new();
                loop {
                    if !seen.insert(cursor) {
                        return Err(LoweringError::Cycle(cursor));
                    }
                    out.ast.nodes[cursor].type_id = 23;
                    let next = out.ast.nodes[cursor]
                        .right
                        .ok_or(LoweringError::MissingChild(cursor))?;
                    if out
                        .ast
                        .nodes
                        .get(next)
                        .ok_or(LoweringError::MissingNode(next))?
                        .kind
                        != 0x31
                    {
                        out.ast.nodes[wrapper].left = Some(next);
                        out.ast.nodes[cursor].right = Some(wrapper);
                        break;
                    }
                    cursor = next;
                }
                args = Some(target);
            } else {
                args = Some(wrapper);
            }
        }
        out.ast.nodes[id] = Node {
            kind: 0x61,
            type_id: result_type,
            right: args,
            symbol: Some(bound.symbol),
            ..Node::default()
        };
        let mut metadata = bound.member_metadata;
        // Both values refer to the new, lowered call and are computed by codegen.
        metadata.argument_bytes = None;
        metadata.result_type_index = None;
        out.bindings.member_metadata.insert(id, metadata);
        out.trace.push(LoweringTrace {
            node: id,
            source_kind: old.kind,
            builtin_id: hint.builtin_id,
            original_function: hint.lowering_function,
        });
    }
    active.remove(&id);
    done.insert(id);
    Ok(())
}
