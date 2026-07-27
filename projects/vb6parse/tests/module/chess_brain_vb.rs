use vb6parse::*;

#[test]
fn chess_brain_vb_board_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Board.bas");

    let result = SourceFile::decode_with_replacement("Board.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Board.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_book_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Book.bas");

    let result = SourceFile::decode_with_replacement("Book.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Book.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_chessbrainvb_module_load() {
    let module_bytes = include_bytes!(
        "../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/ChessBrainVB.bas"
    );

    let result = SourceFile::decode_with_replacement("ChessBrainVB.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'ChessBrainVB.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_cmdoutput_module_load() {
    let module_bytes = include_bytes!(
        "../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/CmdOutput.bas"
    );

    let result = SourceFile::decode_with_replacement("CmdOutput.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'CmdOutput.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_const_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Const.bas");

    let result = SourceFile::decode_with_replacement("Const.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Const.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_debug_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Debug.bas");

    let result = SourceFile::decode_with_replacement("Debug.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Debug.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_epd_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/EPD.bas");

    let result = SourceFile::decode_with_replacement("EPD.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'EPD.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
#[ignore = "Failing due to a stack overflow in the parser."]
fn chess_brain_vb_eval_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Eval.bas");

    let result = SourceFile::decode_with_replacement("Eval.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Eval.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_hash_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Hash.bas");

    let result = SourceFile::decode_with_replacement("Hash.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Hash.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_io_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/IO.bas");

    let result = SourceFile::decode_with_replacement("IO.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'IO.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_process_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Process.bas");

    let result = SourceFile::decode_with_replacement("Process.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Process.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_search_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Search.bas");

    let result = SourceFile::decode_with_replacement("Search.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Search.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}

#[test]
fn chess_brain_vb_time_module_load() {
    let module_bytes =
        include_bytes!("../../../../test-data/ChessBrainVB/ChessbrainVB_V4_10/Modules/Time.bas");

    let result = SourceFile::decode_with_replacement("Time.bas", module_bytes);

    let source_file = match result {
        Ok(source_file) => source_file,
        Err(e) => panic!("Failed to decode source file 'Time.bas': {e:?}"),
    };

    let (module_file_opt, failures) = ModuleFile::parse(&source_file).unpack();

    if !failures.is_empty() {
        for failure in failures {
            failure.print();
        }

        panic!("Module parse had failures");
    }

    let module = module_file_opt.expect("Module should be present.");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/module/chess_brain_vb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(module);
}
