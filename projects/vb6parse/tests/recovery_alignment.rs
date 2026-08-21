use vb6parse::ConcreteSyntaxTree;
use vb6parse::parsers::cst::NodeRange;

fn overlaps(start_a: u32, end_a: u32, start_b: u32, end_b: u32) -> bool {
    start_a < end_b && start_b < end_a
}

#[test]
fn recovery_events_align_with_error_recovery_nodes() {
    let source = r"
Property Get Name() As String
    Name = m_name
End Sub
Function NextFunction() As String
    NextFunction = Name
End Function
";

    let result = ConcreteSyntaxTree::from_text("test.bas", source);
    let (cst_opt, _failures, recovery_events) = result.unpack_with_recovery();

    let cst = cst_opt.expect("CST should be present even with syntax recovery.");
    let ranges = cst.error_recovery_ranges();

    assert_eq!(
        ranges.len(),
        recovery_events.len(),
        "Each recovery event should produce exactly one ErrorRecovery CST node."
    );

    for event in &recovery_events {
        let event_start = event.span.offset;
        let event_end = event.span.offset.saturating_add(event.span.length.max(1));

        assert!(
            ranges
                .iter()
                .any(|range| overlaps(event_start, event_end, range.start, range.end)),
            "Recovery event at offset {} should overlap an ErrorRecovery node range.",
            event.span.offset
        );
    }
}

#[test]
fn no_recovery_events_for_clean_source() {
    let source = r"
Function Calculate(x As Integer) As Integer
    Calculate = x * 2
End Function
";

    let result = ConcreteSyntaxTree::from_text("clean.bas", source);
    let (cst_opt, _failures, recovery_events) = result.unpack_with_recovery();

    let cst = cst_opt.expect("CST should be present.");
    assert_eq!(cst.error_recovery_ranges(), [] as [NodeRange; 0]);
    assert!(recovery_events.is_empty());
}
