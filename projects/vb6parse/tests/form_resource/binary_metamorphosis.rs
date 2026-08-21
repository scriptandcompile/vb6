use vb6parse::files::resource::FormResourceFile;
use vb6parse::files::resource::ResourceEntry;

#[test]
fn binary_metamorphosis_v1_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Binary-metamorphosis/Binary metamorphosis V1.0/src/Bin_To_VB.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    // These .frx files contain actual data
    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn binary_metamorphosis_v2_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Binary-metamorphosis/Binary metamorphosis V2.0/src/Bin_To_VB.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    // These .frx files contain actual data
    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn binary_metamorphosis_v3_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Binary-metamorphosis/Binary metamorphosis V3.0/src/Bin_To_VB.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    // These .frx files contain actual data
    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}
