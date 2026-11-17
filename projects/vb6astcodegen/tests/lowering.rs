use vba6_ast_codegen::{
    Ast, Bindings, BuiltinBindings, LoweringError, Node, ResolvedBuiltin,
    emit_ast_with_builtin_lowering, lower_known_builtins, member_calls::ResolvedMemberMetadata,
};
fn runtime() -> BuiltinBindings {
    [9, 12, 0x9b, 0x140]
        .into_iter()
        .map(|id| {
            (
                id,
                ResolvedBuiltin {
                    symbol: 1,
                    signature_result_type: 2,
                    member_metadata: ResolvedMemberMetadata {
                        symbol_kind: Some(4),
                        class: Some(2),
                        descriptor_flags: Some(0),
                        call_descriptor: Some(0x3344),
                        ..Default::default()
                    },
                },
            )
        })
        .collect()
}
#[test]
fn array_and_fixed_string_nodes_become_real_runtime_calls() {
    for kind in [0x3c, 0x3d, 0x40] {
        let mut ast = Ast::default();
        let a = ast.push(Node::integer(6, 7));
        let b = ast.push(Node::integer(6, 2));
        let args = ast.push(Node {
            kind: 0x37,
            left: Some(a),
            right: Some(b),
            ..Default::default()
        });
        let root = ast.push(Node {
            kind,
            type_id: 2,
            right: Some(args),
            ..Default::default()
        });
        let (out, trace) =
            emit_ast_with_builtin_lowering(&ast, root, 1, &Bindings::default(), &runtime())
                .unwrap();
        assert_eq!(out.bytes, [0xf4, 7, 0xf4, 2, 0x0a, 0x44, 0x33, 8, 0]);
        assert_eq!(out.instructions.last().unwrap().logical_opcode, 0x32c);
        assert_eq!(trace.len(), 1);
        assert_eq!(trace[0].source_kind, kind);
        assert_eq!(ast.nodes[root].kind, kind);
    }
}
#[test]
fn array_destructor_materializes_the_native_i4_zero_argument() {
    let mut ast = Ast::default();
    let args = ast.push(Node {
        kind: 0x37,
        ..Default::default()
    });
    let root = ast.push(Node {
        kind: 0x3d,
        type_id: 2,
        right: Some(args),
        ..Default::default()
    });
    let (out, _) =
        emit_ast_with_builtin_lowering(&ast, root, 1, &Bindings::default(), &runtime()).unwrap();
    assert_eq!(out.instructions[0].logical_opcode, 0x3b8);
    assert_eq!(out.instructions[0].operands, [0, 0, 0, 0]);
    assert_eq!(
        out.instructions.last().unwrap().operands,
        [0x44, 0x33, 4, 0]
    );
    assert!(ast.nodes[args].left.is_none());
}
#[test]
fn gosub_cleanup_wraps_the_bound_value_with_native_type23() {
    let mut ast = Ast::default();
    let value = ast.push(Node::integer(6, 7));
    let root = ast.push(Node {
        kind: 0x5b,
        type_id: 2,
        symbol: Some(5),
        ..Default::default()
    });
    let mut bindings = Bindings::default();
    bindings.bound_nodes.insert(5, value);
    let lowered = lower_known_builtins(&ast, root, &bindings, &runtime()).unwrap();
    let wrapper = lowered.ast.nodes[root].right.unwrap();
    assert_eq!(lowered.ast.nodes[wrapper].kind, 0x11);
    assert_eq!(lowered.ast.nodes[wrapper].type_id, 23);
    assert_eq!(lowered.ast.nodes[wrapper].left, Some(value));
    let (out, _) = emit_ast_with_builtin_lowering(&ast, root, 1, &bindings, &runtime()).unwrap();
    assert_eq!(out.bytes, [0xf4, 7, 0x0a, 0x44, 0x33, 4, 0]);
}
#[test]
fn missing_runtime_symbol_is_an_error_instead_of_silent_no_emission() {
    let mut ast = Ast::default();
    let root = ast.push(Node {
        kind: 0x3c,
        type_id: 2,
        ..Default::default()
    });
    assert!(matches!(
        emit_ast_with_builtin_lowering(
            &ast,
            root,
            1,
            &Bindings::default(),
            &BuiltinBindings::new()
        ),
        Err(LoweringError::MissingBuiltin { builtin: 9, .. })
    ));
}
