use vb6parse::files::resource::FormResourceFile;

#[test]
fn environment_avi_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/Avi.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_colordialog_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/ColorDialog.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_fileselectordialog_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/FileSelectorDialog.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_fontdialog1_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/FontDialog1.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_frmabout_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/frmAbout.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_guim2000_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/GuiM2000.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_help_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/help.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_mform1_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/mForm1.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_neomsgbox_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/NeoMsgBox.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_small_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/SMALL.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_test_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/TEST.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_textp0_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/TextP0.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}

#[test]
fn environment_tweakprive_frx() {
    let result = FormResourceFile::from_file("../../test-data/Environment/tweakprive.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert!(!entries.is_empty());
}
