use vb6parse::*;

#[test]
fn audiostation_logger_class_load() {
    let file_path = "../../test-data/audiostation/Source/Classes/clsLogger.cls";
    let class_bytes =
        std::fs::read(file_path).expect(&format!("Failed to read class file: {file_path}"));

    let result = SourceFile::decode_with_replacement(file_path, &class_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file '{file_path}': {e:?}"),
    };

    let (class_file_opt, failures) = ClassFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Class parse had failures");
    }

    let class = class_file_opt.expect("Class should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/class/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(class);
}

#[test]
fn audiostation_sibra_soft_class_load() {
    let file_path = "../../test-data/audiostation/Source/Classes/clsSibraSoft.cls";
    let class_bytes =
        std::fs::read(file_path).expect(&format!("Failed to read class file: {file_path}"));

    let result = SourceFile::decode_with_replacement(file_path, &class_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file '{file_path}': {e:?}"),
    };

    let (class_file_opt, failures) = ClassFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Class parse had failures");
    }

    let class = class_file_opt.expect("Class should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/class/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(class);
}

#[test]
fn audiostation_smart_buffer_class_load() {
    let file_path = "../../test-data/audiostation/Source/Classes/clsSmartBuffer.cls";
    let class_bytes =
        std::fs::read(file_path).expect(&format!("Failed to read class file: {file_path}"));

    let result = SourceFile::decode_with_replacement(file_path, &class_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file '{file_path}': {e:?}"),
    };

    let (class_file_opt, failures) = ClassFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Class parse had failures");
    }

    let class = class_file_opt.expect("Class should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/class/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(class);
}

#[test]
fn audiostation_string_builder_class_load() {
    let file_path = "../../test-data/audiostation/Source/Classes/clsStringBuilder.cls";
    let class_bytes =
        std::fs::read(file_path).expect(&format!("Failed to read class file: {file_path}"));

    let result = SourceFile::decode_with_replacement(file_path, &class_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file '{file_path}': {e:?}"),
    };

    let (class_file_opt, failures) = ClassFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Class parse had failures");
    }

    let class = class_file_opt.expect("Class should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/class/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(class);
}

#[test]
fn audiostation_volume_channel_class_load() {
    let file_path = "../../test-data/audiostation/Source/Classes/mdlVolumeChannel.cls";
    let class_bytes =
        std::fs::read(file_path).expect(&format!("Failed to read class file: {file_path}"));

    let result = SourceFile::decode_with_replacement(file_path, &class_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file '{file_path}': {e:?}"),
    };

    let (class_file_opt, failures) = ClassFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Class parse had failures");
    }

    let class = class_file_opt.expect("Class should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/class/audiostation");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(class);
}
