//! Integration tests for linking a `.res` file to the interpreter.
//!
//! These live in their own test binary because the resource binding and the
//! file backend are process-global: every `run_module` either links the staged
//! resource file or clears the binding, so a run in another test would unlink
//! this file's fixture mid-test. Cargo gives each integration test file its own
//! process, which isolates them from the rest of the suite; the lock below
//! serializes them against each other within this process.

use std::sync::Mutex;

use vb6interpret::Interpreter;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;

/// Serializes tests that link a `.res` file: the resource binding and the file
/// backend are process-global, so parallel runs would stomp each other.
static RES_TEST_LOCK: Mutex<()> = Mutex::new(());

/// Builds a `.res` record with an ordinal type and name.
fn res_record(res_type: u16, name: u16, data: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&32u32.to_le_bytes()); // header size
    bytes.extend_from_slice(&0xFFFFu16.to_le_bytes()); // type is an ordinal
    bytes.extend_from_slice(&res_type.to_le_bytes());
    bytes.extend_from_slice(&0xFFFFu16.to_le_bytes()); // name is an ordinal
    bytes.extend_from_slice(&name.to_le_bytes());
    bytes.extend_from_slice(&[0u8; 16]); // fixed header trailer
    bytes.extend_from_slice(data);
    bytes.resize(bytes.len().next_multiple_of(4), 0);
    bytes
}

/// Builds an `RT_STRING` bundle holding `slots` from ID 0 upward.
fn res_string_bundle(slots: &[&str]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for slot in 0..16 {
        let text = slots.get(slot).copied().unwrap_or("");
        let units: Vec<u16> = text.encode_utf16().collect();
        bytes.extend_from_slice(&(units.len() as u16).to_le_bytes());
        for unit in units {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
    }
    bytes
}

/// A `.res` image with one string bundle and one custom data resource.
fn sample_res_image() -> Vec<u8> {
    let mut image = res_record(0, 0, &[]); // leading null record
    image.extend(res_record(
        6,
        1,
        &res_string_bundle(&["zero", "one", "two"]),
    ));
    image.extend(res_record(10, 101, b"custom-payload"));
    image
}

/// Writes `image` into a fresh memory backend and returns its path.
fn stage_res_file(interpreter: &Interpreter, image: &[u8]) -> String {
    interpreter.set_file_backend(Box::new(
        vb6runtime::state::file::memory::MemoryBackend::new(),
    ));
    vb6runtime::state::file::set_root("/");
    vb6runtime::state::file::write_memory_file("/app.res", image).unwrap();
    "/app.res".to_string()
}

#[test]
fn loadresstring_reads_the_linked_resource_file() {
    let _guard = RES_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let mut interpreter = Interpreter::new();
    let path = stage_res_file(&interpreter, &sample_res_image());
    interpreter.set_resource_file(&path);

    let source = "Attribute VB_Name = \"M\"\n\
         Sub Main()\n\
         Debug.Print LoadResString(1)\n\
         Debug.Print LoadResString(2)\n\
         End Sub\n";
    let source_file = SourceFile::from_string("m.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    interpreter.run_module(&module).expect("run failed");

    assert_eq!(
        interpreter.output(),
        &["one".to_string(), "two".to_string()]
    );

    interpreter.clear_resource_file();
    interpreter.reset_file_backend();
}

#[test]
fn loadresdata_reads_the_linked_resource_file() {
    let _guard = RES_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let mut interpreter = Interpreter::new();
    let path = stage_res_file(&interpreter, &sample_res_image());
    interpreter.set_resource_file(&path);

    // UBound of a zero-based 14-byte array is 13.
    let source = "Attribute VB_Name = \"M\"\n\
         Sub Main()\n\
         Dim data\n\
         data = LoadResData(101, 10)\n\
         Debug.Print UBound(data)\n\
         End Sub\n";
    let source_file = SourceFile::from_string("m.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    interpreter.run_module(&module).expect("run failed");

    assert_eq!(interpreter.output(), &["13".to_string()]);

    interpreter.clear_resource_file();
    interpreter.reset_file_backend();
}

#[test]
fn resource_file_survives_clear_and_reapplies_each_run() {
    let _guard = RES_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let mut interpreter = Interpreter::new();
    let path = stage_res_file(&interpreter, &sample_res_image());
    interpreter.set_resource_file(&path);

    let source = "Attribute VB_Name = \"M\"\n\
         Sub Main()\n\
         Debug.Print LoadResString(0)\n\
         End Sub\n";
    let source_file = SourceFile::from_string("m.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();

    interpreter.run_module(&module).expect("first run failed");
    assert_eq!(interpreter.output(), &["zero".to_string()]);

    // clear() must preserve the link, and the second run must re-apply it.
    interpreter.clear();
    assert_eq!(interpreter.resource_file(), Some(path.as_str()));
    interpreter.run_module(&module).expect("second run failed");
    assert_eq!(interpreter.output(), &["zero".to_string()]);

    interpreter.clear_resource_file();
    interpreter.reset_file_backend();
}

#[test]
fn without_a_linked_resource_file_loadres_raises_326() {
    let _guard = RES_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let mut interpreter = Interpreter::new();
    stage_res_file(&interpreter, &sample_res_image());
    // Deliberately do not link the file.
    assert_eq!(interpreter.resource_file(), None);

    let source = "Attribute VB_Name = \"M\"\n\
         Sub Main()\n\
         Debug.Print LoadResString(1)\n\
         End Sub\n";
    let source_file = SourceFile::from_string("m.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    let error = interpreter.run_module(&module).unwrap_err();
    assert!(
        format!("{error}").contains("326") || format!("{error}").contains("Resource"),
        "expected a resource error, got: {error}"
    );

    interpreter.reset_file_backend();
}

#[test]
fn clearing_the_resource_file_unlinks_it_for_later_runs() {
    let _guard = RES_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let mut interpreter = Interpreter::new();
    let path = stage_res_file(&interpreter, &sample_res_image());
    interpreter.set_resource_file(&path);

    let source = "Attribute VB_Name = \"M\"\n\
         Sub Main()\n\
         Debug.Print LoadResString(1)\n\
         End Sub\n";
    let source_file = SourceFile::from_string("m.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    interpreter.run_module(&module).expect("linked run failed");

    interpreter.clear_resource_file();
    assert_eq!(interpreter.resource_file(), None);
    assert!(
        interpreter.run_module(&module).is_err(),
        "unlinked run should fail"
    );

    interpreter.reset_file_backend();
}

#[test]
fn relinking_picks_up_rewritten_resource_contents() {
    let _guard = RES_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let mut interpreter = Interpreter::new();
    let path = stage_res_file(&interpreter, &sample_res_image());
    interpreter.set_resource_file(&path);

    let source = "Attribute VB_Name = \"M\"\n\
         Sub Main()\n\
         Debug.Print LoadResString(1)\n\
         End Sub\n";
    let source_file = SourceFile::from_string("m.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    interpreter.run_module(&module).expect("first run failed");
    assert_eq!(interpreter.output(), &["one".to_string()]);

    // Rewrite the file with different strings; the next run re-links, which
    // drops the cached parse.
    let mut replacement = res_record(0, 0, &[]);
    replacement.extend(res_record(6, 1, &res_string_bundle(&["A", "B"])));
    vb6runtime::state::file::write_memory_file("/app.res", &replacement).unwrap();

    interpreter.run_module(&module).expect("second run failed");
    assert_eq!(interpreter.output(), &["B".to_string()]);

    interpreter.clear_resource_file();
    interpreter.reset_file_backend();
}
