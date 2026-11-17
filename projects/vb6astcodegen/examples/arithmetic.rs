use vba6_ast_codegen::{Ast, Bindings, Node, emit_ast};
fn main() {
    let mut ast = Ast::default();
    let left = ast.push(Node::integer(6, 7));
    let right = ast.push(Node::integer(6, 2));
    let root = ast.push(Node::operator(0x16, 6, left, Some(right)));
    let output = emit_ast(&ast, root, 1, &Bindings::default()).expect("bound integer AST");
    println!(
        "logical opcodes: {:X?}",
        output
            .instructions
            .iter()
            .map(|i| i.logical_opcode)
            .collect::<Vec<_>>()
    );
    println!("bytecode: {:02X?}", output.bytes);
}
