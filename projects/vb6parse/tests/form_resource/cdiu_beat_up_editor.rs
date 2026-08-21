use vb6parse::files::resource::FormResourceFile;
use vb6parse::files::resource::ResourceEntry;

#[test]
fn cdiu_beat_up_editor_chatbox_frx() {
    let result = FormResourceFile::from_file("../../test-data/CdiuBeatUpEditor/ChatBox.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn cdiu_beat_up_editor_openroom_frx() {
    let result = FormResourceFile::from_file("../../test-data/CdiuBeatUpEditor/OpenRoom.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn cdiu_beat_up_editor_systemread_frx() {
    let result = FormResourceFile::from_file("../../test-data/CdiuBeatUpEditor/systemRead.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn cdiu_beat_up_editor_test_frx() {
    let result = FormResourceFile::from_file("../../test-data/CdiuBeatUpEditor/test.frx")
        .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}
