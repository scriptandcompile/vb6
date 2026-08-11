use vb6interpret::Interpreter;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;

fn trace_lines(source: &str) -> Vec<(usize, Option<String>)> {
    let source_file = SourceFile::from_string("scratch.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    let mut interpreter = Interpreter::new();
    interpreter.set_record_debug_snapshots(true);
    let _ = interpreter.run_module(&module);
    interpreter.capture_final_debug_snapshot();
    interpreter
        .debug_snapshots()
        .iter()
        .map(|s| (s.current_line, s.current_procedure.clone()))
        .collect()
}

#[test]
fn hello_highlights_entry_and_exit() {
    let lines = trace_lines(
        "Attribute VB_Name = \"HelloModule\"\n\n\
         Sub Main()\n    Debug.Print \"hi\"\nEnd Sub\n",
    );
    assert_eq!(
        lines,
        vec![
            (3, Some("Main".to_string())),
            (4, Some("Main".to_string())),
            (5, Some("Main".to_string())),
        ]
    );
}

#[test]
fn module_statements_then_entry() {
    let lines = trace_lines(
        "Attribute VB_Name = \"M\"\n\
         Dim gCount As Integer\n\
         Const BASE As Long = 100\n\
         Sub Main()\n    Debug.Print gCount\nEnd Sub\n",
    );
    assert_eq!(
        lines,
        vec![
            (2, None),
            (3, None),
            (4, Some("Main".to_string())),
            (5, Some("Main".to_string())),
            (6, Some("Main".to_string())),
        ]
    );
}

#[test]
fn branching_highlights_taken_branch_then_end_sub() {
    let lines = trace_lines(
        "Attribute VB_Name = \"B\"\n\n\
         Sub Main()\n    Dim score As Integer\n    score = 84\n    If score >= 80 Then\n        Debug.Print \"pass\"\n    Else\n        Debug.Print \"retry\"\n    End If\nEnd Sub\n",
    );
    assert_eq!(
        lines,
        vec![
            (3, Some("Main".to_string())),
            (4, Some("Main".to_string())),
            (5, Some("Main".to_string())),
            (6, Some("Main".to_string())),
            (7, Some("Main".to_string())),
            (11, Some("Main".to_string())),
        ]
    );
}
