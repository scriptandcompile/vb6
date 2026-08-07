//! Integration tests for the `vb6interpret` tree-walking interpreter.

use vb6interpret::run_source;

/// Run a module body and return the captured `Debug.Print` output.
fn run(body: &str) -> Vec<String> {
    let source = format!("Attribute VB_Name = \"M\"\nSub Main()\n{}\nEnd Sub\n", body);
    run_source(&source).expect("interpretation failed")
}

#[test]
fn arithmetic_and_concat() {
    let out = run("    Debug.Print 2 + 3 * 4\n\
         Debug.Print 10 \\ 3\n\
         Debug.Print 10 Mod 3\n\
         Debug.Print 2 ^ 10\n\
         Debug.Print \"a\" & \"b\" & 1\n");
    assert_eq!(out, vec!["14", "3", "1", "1024", "ab1"]);
}

#[test]
fn integer_types() {
    let out = run("    Dim i As Integer\n\
         Dim l As Long\n\
         i = 32767\n\
         l = i + 1\n\
         Debug.Print i\n\
         Debug.Print l\n");
    assert_eq!(out, vec!["32767", "32768"]);
}

#[test]
fn string_functions() {
    let out = run("    Dim s As String\n\
         s = \"  Hello World  \"\n\
         Debug.Print Trim(s)\n\
         Debug.Print UCase(s)\n\
         Debug.Print Left(s, 5)\n\
         Debug.Print Right(s, 5)\n\
         Debug.Print Mid(s, 8, 5)\n\
         Debug.Print Len(s)\n\
         Debug.Print InStr(\"hello\", \"ll\")\n\
         Debug.Print Chr(65) & Chr(66)\n\
         Debug.Print Space(3) & \"!\"\n");
    assert_eq!(
        out,
        vec![
            "Hello World",
            "  HELLO WORLD  ",
            "  Hel",
            "rld  ",
            " Worl",
            "15",
            "3",
            "AB",
            "   !",
        ]
    );
}

#[test]
fn string_function_suffix_variants() {
    let out = run("    Debug.Print Chr$(65) & Chr$(66)\n\
         Debug.Print ChrB$(65) & ChrB$(66)\n\
         Debug.Print ChrW$(65) & ChrW$(66)\n\
         Debug.Print Trim$(\"  hi  \")\n");
    assert_eq!(out, vec!["AB", "AB", "AB", "hi"]);
}

#[test]
fn if_elseif_else_block() {
    let out = run("    Dim n As Integer\n\
         n = 7\n\
         If n < 5 Then\n\
         Debug.Print \"low\"\n\
         ElseIf n < 10 Then\n\
         Debug.Print \"mid\"\n\
         Else\n\
         Debug.Print \"high\"\n\
         End If\n\
         If n < 5 Then Debug.Print \"one\" Else Debug.Print \"two\"\n");
    assert_eq!(out, vec!["mid", "two"]);
}

#[test]
fn select_case() {
    let out = run("    Dim a As Long\n\
         a = 4\n\
         Select Case a\n\
         Case 1, 2\n\
         Debug.Print \"low\"\n\
         Case 3 To 5\n\
         Debug.Print \"three-to-five\"\n\
         Case Is > 100\n\
         Debug.Print \"big\"\n\
         Case Else\n\
         Debug.Print \"other\"\n\
         End Select\n");
    assert_eq!(out, vec!["three-to-five"]);
}

#[test]
fn for_loop_with_step() {
    let out = run("    Dim total As Long\n\
         Dim i As Integer\n\
         For i = 1 To 10 Step 2\n\
         total = total + i\n\
         Next i\n\
         Debug.Print total\n\
         Dim j As Integer\n\
         For j = 5 To 1 Step -1\n\
         Debug.Print j\n\
         Next j\n");
    assert_eq!(out, vec!["25", "5", "4", "3", "2", "1"]);
}

#[test]
fn do_and_while_loops() {
    let out = run("    Dim x As Long\n\
         x = 1\n\
         Do While x < 1000\n\
         x = x * 2\n\
         Loop\n\
         Debug.Print x\n\
         x = 1\n\
         Do\n\
         x = x + 1\n\
         Loop Until x >= 5\n\
         Debug.Print x\n\
         Dim n As Integer\n\
         n = 0\n\
         While n < 4\n\
         n = n + 1\n\
         Wend\n\
         Debug.Print n\n");
    assert_eq!(out, vec!["1024", "5", "4"]);
}

#[test]
fn recursion_factorial() {
    let source = "Attribute VB_Name = \"M\"\n\
Function Factorial(n As Integer) As Long\n\
    If n <= 1 Then\n\
        Factorial = 1\n\
    Else\n\
        Factorial = n * Factorial(n - 1)\n\
    End If\n\
End Function\n\
Sub Main()\n\
    Debug.Print Factorial(6)\n\
    Debug.Print Factorial(0)\n\
End Sub\n";
    let out = run_source(source).expect("interpretation failed");
    assert_eq!(out, vec!["720", "1"]);
}

#[test]
fn arrays_and_dim_const() {
    let source = "Attribute VB_Name = \"M\"\n\
Const MAX As Integer = 3\n\
Sub Main()\n\
    Dim a(1 To MAX) As Integer\n\
    Dim total As Long\n\
    a(1) = 10\n\
    a(2) = 20\n\
    a(3) = 30\n\
    For i = 1 To MAX\n\
        total = total + a(i)\n\
    Next i\n\
    Debug.Print total\n\
End Sub\n";
    let out = run_source(source).expect("interpretation failed");
    assert_eq!(out, vec!["60"]);
}

#[test]
fn function_returns_default_when_unset() {
    let source = "Attribute VB_Name = \"M\"\n\
Function Unset() As Integer\n\
End Function\n\
Sub Main()\n\
    Debug.Print Unset()\n\
End Sub\n";
    let out = run_source(source).expect("interpretation failed");
    assert_eq!(out, vec!["0"]);
}

#[test]
fn global_const_and_module_level_init() {
    let source = "Attribute VB_Name = \"M\"\n\
Dim gCount As Integer\n\
Const BASE As Long = 100\n\
Sub Main()\n\
    Debug.Print gCount\n\
    Debug.Print BASE\n\
End Sub\n";
    let out = run_source(source).expect("interpretation failed");
    assert_eq!(out, vec!["0", "100"]);
}

#[test]
fn division_by_zero_reports_line() {
    let source = "Attribute VB_Name = \"M\"\n\
Sub Main()\n\
    Dim x As Double\n\
    x = 1 / 0\n\
End Sub\n";
    let error = run_source(source).expect_err("expected division by zero");
    assert_eq!(error.error.number, 11);
    assert!(error.to_string().contains("line 3"));
}

#[test]
fn print_separators() {
    let out = run("    Debug.Print \"a\"; \"b\"\n    Debug.Print \"c\"\n    Debug.Print \"x\";\n    Debug.Print \"y\"\n");
    assert_eq!(out, vec!["ab", "c", "xy"]);
}

#[test]
fn like_operator() {
    let out = run("    Debug.Print \"Hello\" Like \"H*\"\n\
         Debug.Print \"hello\" Like \"H*\"\n\
         Debug.Print \"abc123\" Like \"???###\"\n\
         Debug.Print \"cat\" Like \"[a-d]at\"\n\
         Debug.Print \"eat\" Like \"[!a-d]at\"\n\
         Debug.Print \"x?y\" Like \"x[?]y\"\n\
         Debug.Print \"[a\" Like \"[[]a\"\n\
         Debug.Print \"hello\" Like \"h[eo]l?o\"\n");
    assert_eq!(
        out,
        vec!["True", "True", "True", "True", "True", "True", "True", "True"]
    );
}

#[test]
fn is_operator() {
    let out = run("    Dim v As Variant\n\
         Debug.Print v Is Nothing\n\
         v = Nothing\n\
         Debug.Print v Is Nothing\n\
         Dim w As Variant\n\
         w = \"x\"\n\
         Debug.Print w Is Nothing\n");
    assert_eq!(out, vec!["False", "True", "False"]);
}

#[test]
fn bitwise_logical_operators() {
    let out = run("    Debug.Print 5 And 3\n\
         Debug.Print 5 Or 2\n\
         Debug.Print 5 Xor 1\n\
         Debug.Print 5 Eqv 3\n\
         Debug.Print True And False\n\
         Debug.Print True Imp False\n");
    assert_eq!(out, vec!["1", "7", "4", "-7", "False", "False"]);
}
