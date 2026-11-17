#!/bin/sh
# Optional differential checks: require the original, locally supplied research sources.
set -eu
cd "$(dirname "$0")/.."
cargo build
mkdir -p target/verification
for name in ir_codegen_checks latebound_stream_checks conversion_stream_checks io_stream_checks member_stream_checks differential_selectors verify_assignment_switch verify_member_selectors verify_member_tails redim_stream_checks getput_stream_checks recovered_branch_checks reference_stream_checks full_reference_stream_checks verify_builtin_names
do
    rustc --edition=2021 -Awarnings "verification/$name.rs" \
        --extern vba6_ast_codegen=target/debug/libvba6_ast_codegen.rlib \
        -L dependency=target/debug/deps -o "target/verification/$name"
    "target/verification/$name"
done
