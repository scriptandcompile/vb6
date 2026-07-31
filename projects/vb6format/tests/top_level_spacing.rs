use vb6format::FmtSettings;

mod common;

#[test]
fn blank_lines_around_top_level_only_between_constructs() {
    let source = "\
Option Explicit
Sub Foo()
End Sub
Sub Bar()
End Sub
";
    let expected = "\
Option Explicit
Sub Foo()
End Sub

Sub Bar()
End Sub
";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}

#[test]
fn blank_lines_around_top_level_keeps_comments_with_next_construct() {
    let source = "\
Option Explicit
Sub Foo()
End Sub
' docs
Sub Bar()
End Sub
";
    let expected = "\
Option Explicit
Sub Foo()
End Sub

' docs
Sub Bar()
End Sub
";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}

#[test]
fn blank_lines_around_top_level_preserves_existing_spacing() {
    let source = "\
Option Explicit
Sub Foo()
End Sub

Sub Bar()
End Sub
";
    let expected = "\
Option Explicit
Sub Foo()
End Sub

Sub Bar()
End Sub
";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}