//! Observed builtin selections for intermediate AST nodes, not direct P-code mappings.
//! Callee construction uses kind 0x60; call construction uses kind 0x61.
//! Binding, signature resolution and complete argument lowering remain caller-owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinLoweringHint {
    pub ast_kind: u16,
    pub builtin_id: u16,
    pub result_type_template: u32,
    pub original_runtime_name: &'static str,
    pub name_record_address: u32,
    pub name_string_address: u32,
    pub lowering_function: u32,
    pub helper_function: u32,
}
pub const BUILTIN_LOWERING_HINTS: [BuiltinLoweringHint; 4] = [
    BuiltinLoweringHint {
        ast_kind: 0x3c,
        builtin_id: 9,
        result_type_template: 2,
        original_runtime_name: "__vbaAryConstruct2",
        name_record_address: 0x0faf0eac,
        name_string_address: 0x0faf3c04,
        lowering_function: 0x0faec8d3,
        helper_function: 0x0faedced,
    },
    BuiltinLoweringHint {
        ast_kind: 0x3d,
        builtin_id: 12,
        result_type_template: 2,
        original_runtime_name: "__vbaAryDestruct",
        name_record_address: 0x0faf0ee8,
        name_string_address: 0x0faf3bd0,
        lowering_function: 0x0faec8d3,
        helper_function: 0x0faedced,
    },
    BuiltinLoweringHint {
        ast_kind: 0x40,
        builtin_id: 0x9b,
        result_type_template: 2,
        original_runtime_name: "__vbaFixstrConstruct",
        name_record_address: 0x0faf1a14,
        name_string_address: 0x0faf3318,
        lowering_function: 0x0faec8d3,
        helper_function: 0x0faedced,
    },
    BuiltinLoweringHint {
        ast_kind: 0x5b,
        builtin_id: 0x140,
        result_type_template: 2,
        original_runtime_name: "__vbaGosubFree",
        name_record_address: 0x0faf26f8,
        name_string_address: 0x0faf2844,
        lowering_function: 0x0faec8d3,
        helper_function: 0x0fb8ebec,
    },
];
pub fn builtin_lowering_hint(kind: u16) -> Option<&'static BuiltinLoweringHint> {
    BUILTIN_LOWERING_HINTS
        .iter()
        .find(|hint| hint.ast_kind == kind)
}
