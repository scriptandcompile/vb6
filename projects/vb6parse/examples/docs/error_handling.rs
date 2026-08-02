use vb6parse::{ConcreteSyntaxTree, SourceFile};

fn main() {
    // Malformed VB6 code: the first procedure ends with End Function instead of End Sub,
    // which forces the parser to recover before the next Public Sub.
    let bad_code = r#"Attribute VB_Name = "BadModule"

Public Sub BrokenFunction()
    x = 5
End Function

Public Sub AnotherFunction()
    MsgBox "This one is fine"
End Sub
"#;

    let source = SourceFile::from_string("BadModule.bas", bad_code);
    let result = ConcreteSyntaxTree::from_source(&source);
    let (cst, failures, recovery_events) = result.unpack_with_recovery();

    if let Some(cst) = cst {
        println!("✓ Parsed CST: {:?}", cst.root_kind());
        println!(
            "  (Despite {} errors and {} recovery events)",
            failures.len(),
            recovery_events.len()
        );
    } else {
        println!("✗ Parsing completely failed");
    }

    if !failures.is_empty() {
        println!("\nParsing Issues:");
        for failure in failures {
            println!(
                "  Line {}-{}: {:?}",
                failure.line_start, failure.line_end, failure.kind
            );

            failure.print();
        }
    }

    if !recovery_events.is_empty() {
        println!("\nRecovery points:");
        for event in recovery_events {
            println!(
                "  {:?} at offset {} (line {})",
                event.strategy, event.span.offset, event.span.line_start
            );
        }
    }
}
