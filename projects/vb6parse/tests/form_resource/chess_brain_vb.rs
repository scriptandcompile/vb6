use vb6parse::files::resource::FormResourceFile;

#[test]
fn chess_brain_vb_debugmain_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Forms/DebugMain.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn chess_brain_vb_main_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Forms/main.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}
