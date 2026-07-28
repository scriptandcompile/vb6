use image::EncodableLayout;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;

#[test]
fn audiostation_mod_args_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modArgs.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}

#[test]
fn audiostation_mod_enums_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modEnums.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}

#[test]
fn audiostation_mod_language_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modLanguage.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}

#[test]
fn audiostation_mod_main_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modMain.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}

#[test]
fn audiostation_mod_mus_player_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modMusPlayer.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}

#[test]
fn audiostation_mod_os_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modOS.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}

#[test]
fn audiostation_mod_sid_player_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modSidPlayer.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}

#[test]
fn audiostation_mod_volume_module_load() {
    let file_path = "../../test-data/audiostation/Source/Modules/modVolume.bas";
    let module_file_bytes = std::fs::read(file_path).expect("Failed to read module file");

    let module_source_file =
        match SourceFile::decode_with_replacement(file_path, module_file_bytes.as_bytes()) {
            Ok(source_file) => source_file,
            Err(e) => {
                e.print();
                panic!("failed to decode module '{file_path}'.");
            }
        };

    let result = ModuleFile::parse(&module_source_file);

    let (module_file_opt, failures) = result.unpack();
    let Some(module_file) = module_file_opt else {
        for failure in &failures {
            failure.eprint();
        }
        panic!("Failed to parse '{file_path}' module file");
    };

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module_file);
}
