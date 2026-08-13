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

/// Trace lines plus the sub-line cursor range `[start, end)` of each snapshot.
fn trace_with_cursors(source: &str) -> Vec<(usize, Option<(u32, u32)>)> {
    let source_file = SourceFile::from_string("scratch.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    let mut interpreter = Interpreter::new();
    interpreter.set_record_debug_snapshots(true);
    let _ = interpreter.run_module(&module);
    interpreter.capture_final_debug_snapshot();
    interpreter
        .debug_snapshots()
        .iter()
        .map(|s| (s.current_line, s.cursor_range))
        .collect()
}

fn byte_range(source: &str, text: &str) -> Option<(u32, u32)> {
    let start = source.find(text)? as u32;
    Some((start, start + text.len() as u32))
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

#[test]
fn for_loop_steps_each_element_then_next() {
    let source = "Attribute VB_Name = \"ForModule\"\n\n\
Sub Main()\n    For i = 1 To 3 Step 1\n        Debug.Print i\n    Next i\nEnd Sub\n";
    // The CST (and thus cursor byte offsets) is relative to the module body,
    // with the `Attribute` header stripped.
    let body = source.strip_prefix("Attribute VB_Name = \"ForModule\"\n").unwrap();

    let start_cursor = byte_range(body, "i = 1");
    let step_cursor = byte_range(body, "Step 1");
    let to_cursor = byte_range(body, "To 3");
    let next_cursor = byte_range(body, "Next i");

    let trace = trace_with_cursors(source);
    assert_eq!(
        trace,
        vec![
            (3, None),
            // First iteration: counter assignment, then step.
            (4, start_cursor),
            (4, step_cursor),
            (5, None),
            (6, next_cursor),
            // Second iteration: end check, then step.
            (4, to_cursor),
            (4, step_cursor),
            (5, None),
            (6, next_cursor),
            // Third iteration: end check, then step.
            (4, to_cursor),
            (4, step_cursor),
            (5, None),
            (6, next_cursor),
            // Fourth check fails and the loop exits.
            (4, to_cursor),
            (7, None),
        ]
    );
}

#[test]
fn for_loop_without_step_skips_the_step_highlight() {
    let source = "Attribute VB_Name = \"PlainFor\"\n\n\
Sub Main()\n    For i = 1 To 2\n        Debug.Print i\n    Next\nEnd Sub\n";
    let body = source.strip_prefix("Attribute VB_Name = \"PlainFor\"\n").unwrap();

    let start_cursor = byte_range(body, "i = 1");
    let to_cursor = byte_range(body, "To 2");
    let next_cursor = byte_range(body, "Next");

    assert_eq!(
        trace_with_cursors(source),
        vec![
            (3, None),
            (4, start_cursor),
            (5, None),
            (6, next_cursor),
            (4, to_cursor),
            (5, None),
            (6, next_cursor),
            (4, to_cursor),
            (7, None),
        ]
    );
}
