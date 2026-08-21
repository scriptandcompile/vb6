use vb6parse::files::resource::FormResourceFile;
use vb6parse::files::resource::ResourceEntry;

#[test]
fn endless_runner_game_vb6_0_desert_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/desert.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_form10_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/Form10.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_form2_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/Form2.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_form3_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/Form3.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_form5_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/Form5.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_form9_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/Form9.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_frmsplash_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/frmSplash.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_howtoplay_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/howtoplay.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_jp2_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/jp2.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_jump4_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/jump4.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}

#[test]
fn endless_runner_game_vb6_0_jump_king_frx() {
    let result = FormResourceFile::from_file(
        "../../test-data/Endless-runner-Game_VB6.0/Endless runner project files/jump king.frx",
    )
    .expect("Failed to read file");

    assert!(!result.has_failures());
    let resource_file = result.unwrap_or_fail();

    let mut entries: Vec<_> = resource_file.iter_entries().collect();
    entries.sort_by_key(|(offset, _)| *offset);

    assert_ne!(entries, [] as [(usize, &ResourceEntry); 0]);
}
