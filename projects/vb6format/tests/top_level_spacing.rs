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

#[test]
fn blank_lines_around_top_level_separates_declare_from_option_and_sub() {
    let source = "\
Option Explicit
Public Declare PtrSafe Function MessageBoxA Lib \"user32\" (ByVal hWnd As Long, ByVal lpText As String, ByVal lpCaption As String, ByVal uType As Long) As Long
Sub Foo()
End Sub
";
    let expected = "\
Option Explicit

Public Declare PtrSafe Function MessageBoxA Lib \"user32\" (ByVal hWnd As Long, ByVal lpText As String, ByVal lpCaption As String, ByVal uType As Long) As Long

Sub Foo()
End Sub
";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}

#[test]
fn blank_lines_around_top_level_does_not_force_spacing_between_consecutive_declares() {
    let source = "\
Public Declare Function A Lib \"x\" () As Long
Public Declare Function B Lib \"x\" () As Long
Sub Foo()
End Sub
";
    let expected = "\
Public Declare Function A Lib \"x\" () As Long
Public Declare Function B Lib \"x\" () As Long

Sub Foo()
End Sub
";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}

#[test]
fn blank_lines_around_top_level_preserves_existing_blank_between_declares() {
    let source = "\
Public Declare Function A Lib \"x\" () As Long

Public Declare Function B Lib \"x\" () As Long
Sub Foo()
End Sub
";
    let expected = "\
Public Declare Function A Lib \"x\" () As Long

Public Declare Function B Lib \"x\" () As Long

Sub Foo()
End Sub
";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}

#[test]
fn blank_lines_around_top_level_attaches_comments_and_splits_before_sub_block() {
    let source = "\
Public Declare Function A Lib \"x\" () As Long
' comment for B
Public Declare Function B Lib \"x\" () As Long
' comment for Foo
Sub Foo()
End Sub
";
    let expected = "\
Public Declare Function A Lib \"x\" () As Long
' comment for B
Public Declare Function B Lib \"x\" () As Long

' comment for Foo
Sub Foo()
End Sub
";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}
