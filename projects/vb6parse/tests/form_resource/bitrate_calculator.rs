use vb6parse::files::resource::FormResourceFile;
use vb6parse::files::resource::ResourceEntry;

#[test]
fn bitrate_calculator_about_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Bitrate-calculator/Windows/Source-code/frmAbout.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn bitrate_calculator_main_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Bitrate-calculator/Windows/Source-code/frmMain.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}
