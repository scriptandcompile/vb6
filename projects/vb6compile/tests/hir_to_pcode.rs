use vb6compile::compile_source;
use vb6pcode::{
    DispatchTable,
    program::{FrameSlot, OpcodeIdentity, Operand, PROGRAM_FORMAT_VERSION},
};

#[test]
fn compiles_long_add_to_the_observed_microsoft_opcode_sequence() {
    let source = "Attribute VB_Name = \"AddModule\"\n\
                  Public Function Add(a As Long, b As Long) As Long\n\
                      Add = a + b\n\
                  End Function\n";
    let program = compile_source("AddModule.bas", source).unwrap();

    assert_eq!(program.format_version, PROGRAM_FORMAT_VERSION);
    assert_eq!(program.procedures.len(), 1);
    assert!(program.constants.is_empty());

    let procedure = &program.procedures[0];
    let identities: Vec<_> = procedure
        .instructions
        .iter()
        .map(|instruction| match instruction.opcode {
            OpcodeIdentity::Microsoft(id) => (id.table, id.opcode),
            OpcodeIdentity::MxExtension { .. } => panic!("MX opcode emitted for classic VB6"),
        })
        .collect();
    assert_eq!(
        identities,
        vec![
            (DispatchTable::Primary, 0x6c), // ILdRf
            (DispatchTable::Primary, 0x6c), // ILdRf
            (DispatchTable::Primary, 0xaa), // AddI4
            (DispatchTable::Primary, 0x71), // FStR4
            (DispatchTable::Primary, 0x14), // ExitProc
        ]
    );
    let mnemonics: Vec<_> = procedure
        .instructions
        .iter()
        .map(|instruction| match instruction.opcode {
            OpcodeIdentity::Microsoft(id) => id.catalog_entry().mnemonic,
            OpcodeIdentity::MxExtension { .. } => panic!("MX opcode emitted for classic VB6"),
        })
        .collect();
    assert_eq!(mnemonics, ["ILdRf", "ILdRf", "AddI4", "FStR4", "ExitProc"]);
    assert_eq!(
        procedure.instructions[0].operand,
        Operand::FrameSlot(FrameSlot::Parameter(0))
    );
    assert_eq!(
        procedure.instructions[1].operand,
        Operand::FrameSlot(FrameSlot::Parameter(1))
    );
    assert_eq!(
        procedure.instructions[3].operand,
        Operand::FrameSlot(FrameSlot::FunctionResult)
    );
    assert_eq!(procedure.instructions[2].stack.pops.len(), 2);
    assert_eq!(procedure.instructions[2].stack.pushes.len(), 1);
    assert_eq!(procedure.max_stack, 2);
    assert_eq!(procedure.source_map.len(), procedure.instructions.len());
    assert!(procedure.source_map.iter().all(|entry| entry.line > 0));
    let mut validated = program.clone();
    assert!(vb6pcode::program::validate(&mut validated).is_ok());
}

#[test]
fn compiles_nested_long_additions_recursively() {
    let source = "Attribute VB_Name = \"NestedAddModule\"\n\
                  Public Function Add(a As Long, b As Long) As Long\n\
                      Add = a + (b + a)\n\
                  End Function\n";
    let program = compile_source("NestedAddModule.bas", source).unwrap();
    let procedure = &program.procedures[0];

    let opcodes: Vec<_> = procedure
        .instructions
        .iter()
        .map(|instruction| match instruction.opcode {
            OpcodeIdentity::Microsoft(id) => id.opcode,
            OpcodeIdentity::MxExtension { .. } => panic!("MX opcode emitted for classic VB6"),
        })
        .collect();
    assert_eq!(opcodes, [0x6c, 0x6c, 0x6c, 0xaa, 0xaa, 0x71, 0x14]);
    assert_eq!(
        procedure.instructions[0].operand,
        Operand::FrameSlot(FrameSlot::Parameter(0))
    );
    assert_eq!(
        procedure.instructions[1].operand,
        Operand::FrameSlot(FrameSlot::Parameter(1))
    );
    assert_eq!(
        procedure.instructions[2].operand,
        Operand::FrameSlot(FrameSlot::Parameter(0))
    );
    assert_eq!(procedure.max_stack, 3);
    assert_eq!(procedure.source_map.len(), procedure.instructions.len());
}

#[test]
fn widens_an_integer_literal_for_long_addition() {
    let source = "Attribute VB_Name = \"LiteralAddModule\"\n\
                  Public Function Add(a As Long, b As Long) As Long\n\
                      Add = a + 2\n\
                  End Function\n";
    let program = compile_source("LiteralAddModule.bas", source).unwrap();
    let procedure = &program.procedures[0];

    let opcodes: Vec<_> = procedure
        .instructions
        .iter()
        .map(|instruction| match instruction.opcode {
            OpcodeIdentity::Microsoft(id) => id.opcode,
            OpcodeIdentity::MxExtension { .. } => panic!("MX opcode emitted for classic VB6"),
        })
        .collect();
    assert_eq!(opcodes, [0x6c, 0xf5, 0xaa, 0x71, 0x14]);
    assert_eq!(procedure.instructions[1].operand, Operand::ImmediateI32(2));
    assert_eq!(procedure.max_stack, 2);
}

#[test]
fn lowers_the_supported_long_operator_family_in_one_batch() {
    let cases = [
        ("-", DispatchTable::Primary, 0xae, "SubI4"),
        ("*", DispatchTable::Primary, 0xb2, "MulI4"),
        ("\\", DispatchTable::Primary, 0xc0, "IDvI4"),
        ("Mod", DispatchTable::Primary, 0xc2, "ModI4"),
        ("And", DispatchTable::Primary, 0xc4, "AndI4"),
        ("Or", DispatchTable::Primary, 0xc5, "OrI4"),
        ("Xor", DispatchTable::Lead0, 0x13, "XorI4"),
        ("unary -", DispatchTable::Primary, 0xb8, "UMiI4"),
        ("Not", DispatchTable::Primary, 0xc3, "NotI4"),
    ];
    for (operator, table, opcode, mnemonic) in cases {
        let expression = match operator {
            "unary -" => "-a".to_string(),
            "Not" => "Not a".to_string(),
            binary => format!("a {binary} b"),
        };
        let source = format!(
            "Attribute VB_Name = \"OperatorModule\"\n\
             Public Function Add(a As Long, b As Long) As Long\n\
                 Add = {expression}\n\
             End Function\n"
        );
        let program = compile_source("OperatorModule.bas", &source).unwrap();
        let procedure = &program.procedures[0];
        let operation = &procedure.instructions[procedure.instructions.len() - 3];
        let vb6pcode::program::OpcodeIdentity::Microsoft(id) = operation.opcode else {
            panic!("MX opcode emitted for classic VB6");
        };
        assert_eq!((id.table, id.opcode), (table, opcode), "{operator}");
        assert_eq!(id.catalog_entry().mnemonic, mnemonic, "{operator}");
        assert_eq!(procedure.instructions.last().unwrap().stack.pops.len(), 0);
    }
}

#[test]
fn compiles_power_operator_through_inferred_native_ast_mapping() {
    let source = "Attribute VB_Name = \"UnsupportedAdd\"\n\
                  Public Function Add(a As Long, b As Long) As Long\n\
                      Add = a ^ b\n\
                  End Function\n";
    let program = compile_source("UnsupportedAdd.bas", source).unwrap();
    let procedure = &program.procedures[0];
    assert_eq!(procedure.instructions.len(), 5);
    assert!(matches!(
        procedure.instructions[2].opcode,
        OpcodeIdentity::Microsoft(_)
    ));
}

#[test]
fn lowers_long_comparisons_to_boolean_functions() {
    let cases = [
        ("=", 0xc7, "EqI4"),
        ("<>", 0xcc, "NeI4"),
        ("<", 0xd1, "LtI4"),
        ("<=", 0xd6, "LeI4"),
        (">", 0xdb, "GtI4"),
        (">=", 0xe0, "GeI4"),
    ];
    for (operator, opcode, mnemonic) in cases {
        let source = format!(
            "Attribute VB_Name = \"CompareModule\"\n\
             Public Function IsMatch(a As Long, b As Long) As Boolean\n\
                 IsMatch = a {operator} b\n\
             End Function\n"
        );
        let program = compile_source("CompareModule.bas", &source).unwrap();
        let procedure = &program.procedures[0];
        assert_eq!(
            procedure.instructions[2].opcode,
            OpcodeIdentity::Microsoft(vb6pcode::program::MicrosoftOpcodeId::new(
                DispatchTable::Primary,
                opcode,
            )),
        );
        let OpcodeIdentity::Microsoft(id) = procedure.instructions[2].opcode else {
            panic!("MX opcode emitted for classic VB6");
        };
        assert_eq!(id.catalog_entry().mnemonic, mnemonic);
        assert_eq!(
            procedure.instructions[3].opcode,
            OpcodeIdentity::Microsoft(vb6pcode::program::MicrosoftOpcodeId::new(
                DispatchTable::Primary,
                0x70,
            )),
        );
        assert_eq!(
            procedure.instructions[4].opcode,
            OpcodeIdentity::Microsoft(vb6pcode::program::MicrosoftOpcodeId::new(
                DispatchTable::Primary,
                0x15,
            )),
        );
        assert_eq!(procedure.max_stack, 2);
    }
}
