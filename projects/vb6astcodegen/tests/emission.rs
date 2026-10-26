use vba6_ast_codegen::emitter::select_latebound;
use vba6_ast_codegen::tables::{decode_opcode, encode_opcode};
use vba6_ast_codegen::{Ast, Bindings, EmitError, Literal, Node, emit_ast};

#[test]
fn integer_tree_matches_original_stream() {
    let mut ast = Ast::default();
    let l = ast.push(Node::integer(6, 7));
    let r = ast.push(Node::integer(6, 2));
    let root = ast.push(Node::operator(0x16, 6, l, Some(r)));
    let out = emit_ast(&ast, root, 1, &Bindings::default()).unwrap();
    assert_eq!(out.bytes, [0xf4, 7, 0xf4, 2, 0xa9]);
    assert_eq!(out.native_return, 0);
    assert_eq!(
        out.instructions
            .iter()
            .map(|i| i.logical_opcode)
            .collect::<Vec<_>>(),
        [0x41a, 0x41a, 0x8f]
    );
}
#[test]
fn short_integer_boundaries_and_long_payload_are_exact() {
    for (value, expected) in [
        (-129, vec![0xf3, 0x7f, 0xff]),
        (-128, vec![0xf4, 0x80]),
        (127, vec![0xf4, 0x7f]),
        (128, vec![0xf3, 0x80, 0]),
    ] {
        let mut ast = Ast::default();
        let id = ast.push(Node::integer(6, value));
        assert_eq!(
            emit_ast(&ast, id, 1, &Bindings::default()).unwrap().bytes,
            expected
        );
    }
    let mut ast = Ast::default();
    let id = ast.push(Node::integer(8, 0x12345678));
    let out = emit_ast(&ast, id, 1, &Bindings::default()).unwrap();
    assert_eq!(out.instructions[0].operands, [0x78, 0x56, 0x34, 0x12]);
}
#[test]
fn variant_byte_payload_is_u16_and_slot_precedes_value() {
    let mut ast = Ast::default();
    let id = ast.push(Node {
        kind: 5,
        type_id: 15,
        symbol: Some(7),
        literal: Literal::Variant {
            tag: 17,
            bytes: [0xfe, 1, 2, 3, 4, 5, 6, 7],
            string_reference: None,
        },
        ..Node::default()
    });
    let mut b = Bindings::default();
    b.slots.insert(7, 0x8234);
    let out = emit_ast(&ast, id, 3, &b).unwrap();
    assert_eq!(out.instructions[0].operands, [0x34, 0x82, 0xfe, 0]);
    assert_eq!(out.instructions[1].logical_opcode, 0x18c);
}
#[test]
fn contextual_float_return_and_wrapper_return_are_distinct() {
    let mut ast = Ast::default();
    let leaf = ast.push(Node {
        kind: 3,
        type_id: 10,
        literal: Literal::Raw64(1.25f64.to_le_bytes()),
        ..Node::default()
    });
    let wrapper = ast.push(Node {
        kind: 0x65,
        left: Some(leaf),
        ..Node::default()
    });
    assert_eq!(
        emit_ast(&ast, wrapper, 2, &Bindings::default())
            .unwrap()
            .native_return,
        1
    );
    let root = ast.push(Node::operator(7, 10, leaf, None));
    let out = emit_ast(&ast, root, 1, &Bindings::default()).unwrap();
    assert_eq!(out.native_return, 0);
    assert_eq!(out.instructions.last().unwrap().logical_opcode, 0x189);
}
#[test]
fn reverse_argument_list_preserves_native_order() {
    let mut ast = Ast::default();
    let l = ast.push(Node::integer(6, 7));
    let r = ast.push(Node::integer(6, 2));
    let list = ast.push(Node {
        kind: 0x33,
        left: Some(l),
        right: Some(r),
        ..Node::default()
    });
    assert_eq!(
        emit_ast(&ast, list, 1, &Bindings::default()).unwrap().bytes,
        [0xf4, 2, 0xf4, 7]
    );
}
#[test]
fn unresolved_and_missing_slot_fail_without_partial_output() {
    let mut ast = Ast::default();
    let l = ast.push(Node::integer(6, 7));
    let unsupported = ast.push(Node {
        kind: 0x51,
        flags: 0x600,
        ..Node::default()
    });
    let root = ast.push(Node {
        kind: 0x2f,
        left: Some(l),
        right: Some(unsupported),
        ..Node::default()
    });
    assert!(matches!(
        emit_ast(&ast, root, 1, &Bindings::default()),
        Err(EmitError::Unresolved { kind: 0x51, .. })
    ));
    let missing = ast.push(Node {
        kind: 5,
        type_id: 15,
        literal: Literal::Variant {
            tag: 0,
            bytes: [0; 8],
            string_reference: None,
        },
        ..Node::default()
    });
    assert!(matches!(
        emit_ast(&ast, missing, 1, &Bindings::default()),
        Err(EmitError::MissingBinding(..))
    ));
}
#[test]
fn cycles_and_shared_nodes_are_distinguished() {
    let mut ast = Ast::default();
    let leaf = ast.push(Node::integer(6, 7));
    let root = ast.push(Node::operator(0x16, 6, leaf, Some(leaf)));
    assert_eq!(
        emit_ast(&ast, root, 1, &Bindings::default()).unwrap().bytes,
        [0xf4, 7, 0xf4, 7, 0xa9]
    );
    ast.nodes[root] = Node {
        kind: 0x65,
        left: Some(root),
        ..Node::default()
    };
    assert_eq!(
        emit_ast(&ast, root, 1, &Bindings::default()),
        Err(EmitError::Cycle(root))
    );
}
#[test]
fn special_kind73_is_not_typed_table_access() {
    let mut ast = Ast::default();
    let id = ast.push(Node {
        kind: 0x73,
        literal: Literal::SpecialWord(0x1234),
        ..Node::default()
    });
    let out = emit_ast(&ast, id, 1, &Bindings::default()).unwrap();
    assert_eq!(out.instructions[0].logical_opcode, 0x266);
    assert_eq!(out.instructions[0].operands, [0x34, 0x12]);
}
#[test]
fn short_alias_codec_is_not_falsely_bijective() {
    assert_eq!(encode_opcode(0x19).unwrap(), [0xc5]);
    assert_eq!(decode_opcode(&[0xc5]).unwrap(), (0x1a, 1));
    assert_eq!(encode_opcode(0x166).unwrap(), [0xfc, 0x66]);
    assert_eq!(decode_opcode(&[0xfc, 0x66]).unwrap(), (0x166, 2));
    assert!(decode_opcode(&[0xfc]).is_err());
    assert!(encode_opcode(0x447).is_err());
}
#[test]
fn latebound_original_flag_priority_is_preserved() {
    assert_eq!(
        select_latebound(0x6b, 0x8800, true, false),
        Some((0x3a1, false, true))
    ); // direct wins over Variant/0800
    assert_eq!(
        select_latebound(0x6b, 0x0800, true, true),
        Some((0x43e, false, true))
    );
    assert_eq!(
        select_latebound(0x6c, 0xe000, false, false),
        Some((0x3ac, true, false))
    );
    assert_eq!(
        select_latebound(0x6e, 0xd000, false, false),
        Some((0x40c, true, false))
    ); //1000 wins over layout/direct
}
