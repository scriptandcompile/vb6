//! Integration tests for the `vb6interpret` tree-walking interpreter.

use vb6interpret::run_source;
use vb6interpret::Interpreter;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6runtime::state::settings as settings_state;

use std::sync::Mutex;

/// Serializes tests that install an environment on the shared runtime
/// snapshot; every `run_module` resets it, so parallel runs would stomp each
/// other's assignments.
static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

/// Run a module body and return the captured `Debug.Print` output.
fn run(body: &str) -> Vec<String> {
    let source = format!("Attribute VB_Name = \"M\"\nSub Main()\n{}\nEnd Sub\n", body);
    run_source(&source).expect("interpretation failed")
}

/// Run a module like the playground's plain run (no trace snapshots) and
/// return the final reported execution line.
fn run_final_line(source: &str) -> usize {
    let source_file = SourceFile::from_string("scratch.bas", source);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.run_module(&module);
    interpreter.current_line()
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
fn math_functions() {
    let out = run("    Debug.Print Abs(-5)\n\
         Debug.Print Sqr(16)\n\
         Debug.Print Int(3.7)\n\
         Debug.Print Fix(-3.7)\n\
         Debug.Print Sgn(-7)\n\
         Debug.Print Round(123.456, 2)\n\
         Debug.Print Exp(0)\n\
         Debug.Print Log(1)\n");
    assert_eq!(out, vec!["5", "4", "3", "-3", "-1", "123.46", "1", "0"]);
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

#[test]
fn run_final_line_lands_on_end_sub_after_loop() {
    let source = "Attribute VB_Name = \"M\"\n\n\
Sub Main()\n\
    For i = 1 To 2\n\
        Debug.Print i\n\
    Next i\n\
End Sub\n";
    // A normally-completing program ends with the highlight on the `End Sub`
    // line, not the loop's closing keyword or header.
    assert_eq!(run_final_line(source), 7);
}

#[test]
fn run_final_line_lands_on_end_sub_after_while() {
    let source = "Attribute VB_Name = \"M\"\n\n\
Sub Main()\n\
    i = 0\n\
    While i < 2\n\
        i = i + 1\n\
    Wend\n\
End Sub\n";
    assert_eq!(run_final_line(source), 8);
}

#[test]
fn run_final_line_lands_on_end_sub_after_do() {
    let source = "Attribute VB_Name = \"M\"\n\n\
Sub Main()\n\
    i = 0\n\
    Do While i < 2\n\
        i = i + 1\n\
    Loop\n\
End Sub\n";
    assert_eq!(run_final_line(source), 8);
}

#[test]
fn run_final_line_lands_on_end_sub_after_post_test_loop() {
    let source = "Attribute VB_Name = \"M\"\n\n\
Sub Main()\n\
    i = 0\n\
    Do\n\
        i = i + 1\n\
    Loop While i < 2\n\
End Sub\n";
    assert_eq!(run_final_line(source), 8);
}

#[test]
fn run_final_line_lands_on_end_sub_after_function_entry() {
    let source = "Attribute VB_Name = \"M\"\n\n\
Function Answer() As Integer\n\
    Answer = 42\n\
End Function\n\
Sub Main()\n\
    Debug.Print Answer\n\
End Sub\n";
    assert_eq!(run_final_line(source), 8);
}

#[test]
fn irr_passes_whole_array_with_empty_parens() {
    let source = "Attribute VB_Name = \"M\"\n\
Sub Main()\n\
    Dim Guess, RetRate\n\
    Dim Values(5) As Double\n\
    Guess = .1\n\
    Values(0) = -70000\n\
    Values(1) = 22000\n\
    Values(2) = 25000\n\
    Values(3) = 28000\n\
    Values(4) = 31000\n\
    RetRate = IRR(Values(), Guess) * 100\n\
    Debug.Print Format(RetRate, \"0.0\")\n\
End Sub\n";
    let out = run_source(source).expect("interpretation failed");
    assert_eq!(out, vec!["17.7"]);
}

#[test]
fn irr_default_guess_with_empty_parens() {
    let source = "Attribute VB_Name = \"M\"\n\
Sub Main()\n\
    Dim cfs(0 To 3) As Double\n\
    cfs(0) = -1000\n\
    cfs(1) = 400\n\
    cfs(2) = 400\n\
    cfs(3) = 400\n\
    Debug.Print Format(IRR(cfs()), \"0.00\")\n\
End Sub\n";
    let out = run_source(source).expect("interpretation failed");
    assert_eq!(out, vec!["0.10"]);
}

#[test]
fn array_element_indexing_still_works() {
    let source = "Attribute VB_Name = \"M\"\n\
Sub Main()\n\
    Dim a(3) As Integer\n\
    a(0) = 10\n\
    a(1) = 20\n\
    a(2) = 30\n\
    Debug.Print a(1)\n\
    Debug.Print a(0)\n\
End Sub\n";
    let out = run_source(source).expect("interpretation failed");
    assert_eq!(out, vec!["20", "10"]);
}

/// Run a module body with an environment variable assigned before the run.
fn run_with_env(body: &str, name: &str, value: &str) -> Vec<String> {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let source = format!("Attribute VB_Name = \"M\"\nSub Main()\n{body}\nEnd Sub\n");
    let mut interpreter = Interpreter::new();
    interpreter.set_environment(name, value);
    interpreter
        .run_source(&source)
        .expect("interpretation failed");
    interpreter.output().to_vec()
}

#[test]
fn environ_reads_variable_assigned_before_running() {
    let out = run_with_env(
        "    Debug.Print Environ(\"VB6_TEST_VAR\")\n",
        "VB6_TEST_VAR",
        "hello",
    );
    assert_eq!(out, vec!["hello"]);
}

#[test]
fn environ_numeric_argument_enumerates_the_table() {
    let out = run_with_env(
        "    Dim i As Integer\n\
         i = 1\n\
         Do While Environ(i) <> \"\"\n\
             Debug.Print Environ(i)\n\
             i = i + 1\n\
         Loop\n",
        "VB6_TEST_VAR",
        "hello",
    );
    // The interpreter's overrides are appended at the end of the table, so the
    // assigned variable is the final entry and appears in the enumeration.
    assert!(!out.is_empty());
    for entry in &out {
        assert!(entry.contains('='));
    }
    assert_eq!(out.last().map(String::as_str), Some("VB6_TEST_VAR=hello"));
}

#[test]
fn environ_error_for_bad_index() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let source = "Attribute VB_Name = \"M\"\n\
Sub Main()\n\
    Debug.Print Environ(0)\n\
End Sub\n";
    let mut interpreter = Interpreter::new();
    interpreter.set_environment("VB6_TEST_VAR", "hello");
    let error = interpreter
        .run_source(source)
        .expect_err("expected error 5");
    assert_eq!(error.error.number, 5);
}

#[test]
fn environ_dollar_reads_variable_assigned_before_running() {
    let out = run_with_env(
        "    Debug.Print Environ$(\"VB6_TEST_VAR\")\n",
        "VB6_TEST_VAR",
        "hello",
    );
    assert_eq!(out, vec!["hello"]);
}

#[test]
fn environ_dollar_lookup_is_case_insensitive() {
    let out = run_with_env(
        "    Debug.Print Environ$(\"vb6_test_var\")\n",
        "VB6_TEST_VAR",
        "hello",
    );
    assert_eq!(out, vec!["hello"]);
}

#[test]
fn environ_dollar_returns_empty_for_unset_variable() {
    let out = run_with_env(
        "    Debug.Print \"[\" & Environ$(\"VB6_TEST_MISSING\") & \"]\"\n",
        "VB6_TEST_VAR",
        "hello",
    );
    assert_eq!(out, vec!["[]"]);
}

#[test]
fn environ_dollar_numeric_argument_enumerates_the_table() {
    let out = run_with_env(
        "    Dim i As Integer\n\
         i = 1\n\
         Do While Environ$(i) <> \"\"\n\
             Debug.Print Environ$(i)\n\
             i = i + 1\n\
         Loop\n",
        "VB6_TEST_VAR",
        "hello",
    );
    // The interpreter's overrides are appended at the end of the table, so the
    // assigned variable is the final entry and appears in the enumeration.
    assert!(!out.is_empty());
    for entry in &out {
        assert!(entry.contains('='));
    }
    assert_eq!(out.last().map(String::as_str), Some("VB6_TEST_VAR=hello"));
}

#[test]
fn environ_dollar_assignment_survives_repeated_runs() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.set_environment("VB6_TEST_VAR", "one");
    interpreter
        .run_source(
            "Attribute VB_Name = \"M\"\nSub Main()\n    Debug.Print Environ$(\"VB6_TEST_VAR\")\nEnd Sub\n",
        )
        .expect("interpretation failed");
    assert_eq!(interpreter.output().to_vec(), vec!["one"]);

    interpreter
        .run_source(
            "Attribute VB_Name = \"M\"\nSub Main()\n    Debug.Print Environ$(\"VB6_TEST_VAR\")\nEnd Sub\n",
        )
        .expect("interpretation failed");
    assert_eq!(interpreter.output().to_vec(), vec!["one"]);
}

#[test]
fn environ_dollar_error_for_bad_index() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let source = "Attribute VB_Name = \"M\"\n\
Sub Main()\n\
    Debug.Print Environ$(0)\n\
End Sub\n";
    let mut interpreter = Interpreter::new();
    interpreter.set_environment("VB6_TEST_VAR", "hello");
    let error = interpreter
        .run_source(source)
        .expect_err("expected error 5");
    assert_eq!(error.error.number, 5);
}

#[test]
fn get_setting_reads_from_the_settings_store() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    settings_state::set_store_root(dir.path());
    settings_state::set("MyApp", "Startup", "Left", "150").unwrap();

    let out = run(
        "    Debug.Print GetSetting(\"MyApp\", \"Startup\", \"Left\", \"0\")\n\
         Debug.Print GetSetting(\"MyApp\", \"Startup\", \"Missing\", \"42\")\n\
         Debug.Print GetSetting(\"myapp\", \"startup\", \"left\")\n",
    );
    assert_eq!(out, vec!["150", "42", "150"]);

    settings_state::reset_store_root();
}

/// Redirect the shared settings store to a fresh temp directory for the
/// duration of the test, restoring the default root on drop so later tests
/// never touch the user's real settings.
struct TempSettingsStore {
    _dir: tempfile::TempDir,
}

impl TempSettingsStore {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        settings_state::set_store_root(dir.path());
        Self { _dir: dir }
    }
}

impl Drop for TempSettingsStore {
    fn drop(&mut self) {
        settings_state::reset_store_root();
    }
}

/// Run a module body in a fresh interpreter whose settings were staged
/// beforehand, and return the captured `Debug.Print` output.
fn run_with_settings(body: &str, setup: impl FnOnce(&mut Interpreter)) -> Vec<String> {
    let source = format!("Attribute VB_Name = \"M\"\nSub Main()\n{body}\nEnd Sub\n");
    let mut interpreter = Interpreter::new();
    setup(&mut interpreter);
    interpreter
        .run_source(&source)
        .expect("interpretation failed");
    interpreter.output().to_vec()
}

#[test]
fn staged_settings_are_visible_to_getsetting_during_a_run() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let _store = TempSettingsStore::new();
    let out = run_with_settings(
        "    Debug.Print GetSetting(\"MyApp\", \"Startup\", \"Left\", \"0\")\n",
        |i| i.set_setting("MyApp", "Startup", "Left", "150"),
    );
    assert_eq!(out, vec!["150"]);
}

#[test]
fn get_setting_returns_staged_values_before_and_after_a_run() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let _store = TempSettingsStore::new();
    let mut interpreter = Interpreter::new();
    interpreter.set_setting("MyApp", "Startup", "Left", "150");
    assert_eq!(
        interpreter.get_setting("MyApp", "Startup", "Left"),
        Some("150".to_string())
    );
    assert_eq!(interpreter.get_setting("MyApp", "Startup", "Missing"), None);
    interpreter
        .run_source("Attribute VB_Name = \"M\"\nSub Main()\nEnd Sub\n")
        .expect("interpretation failed");
    assert_eq!(
        interpreter.get_setting("myapp", "startup", "left"),
        Some("150".to_string())
    );
}

#[test]
fn staged_settings_override_values_already_in_the_store() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let _store = TempSettingsStore::new();
    settings_state::set("MyApp", "Startup", "Left", "150").unwrap();
    let out = run_with_settings(
        "    Debug.Print GetSetting(\"MyApp\", \"Startup\", \"Left\", \"0\")\n",
        |i| i.set_setting("MyApp", "Startup", "Left", "200"),
    );
    assert_eq!(out, vec!["200"]);
}

#[test]
fn remove_setting_removes_staged_and_store_values() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let _store = TempSettingsStore::new();
    let mut interpreter = Interpreter::new();
    interpreter.set_setting("MyApp", "Startup", "Left", "150");
    interpreter.set_setting("MyApp", "Startup", "Right", "300");
    interpreter.remove_setting("MyApp", "Startup", "Left");
    assert_eq!(interpreter.get_setting("MyApp", "Startup", "Left"), None);
    assert_eq!(
        interpreter.get_setting("MyApp", "Startup", "Right"),
        Some("300".to_string())
    );
    // The store value was written during a run, so it must be gone too.
    interpreter
        .run_source("Attribute VB_Name = \"M\"\nSub Main()\nEnd Sub\n")
        .expect("interpretation failed");
    interpreter.remove_setting("MyApp", "Startup", "Right");
    assert_eq!(interpreter.get_setting("MyApp", "Startup", "Right"), None);
}

#[test]
fn clear_settings_removes_staged_and_store_values() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let _store = TempSettingsStore::new();
    let mut interpreter = Interpreter::new();
    interpreter.set_setting("MyApp", "Startup", "Left", "150");
    interpreter.set_setting("MyApp", "Startup", "Right", "300");
    interpreter.clear_settings();
    assert_eq!(interpreter.get_setting("MyApp", "Startup", "Left"), None);
    assert_eq!(interpreter.get_setting("MyApp", "Startup", "Right"), None);
}

#[test]
fn staged_settings_survive_clear() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let _store = TempSettingsStore::new();
    let mut interpreter = Interpreter::new();
    interpreter.set_setting("MyApp", "Startup", "Left", "150");
    interpreter.clear();
    assert_eq!(
        interpreter.get_setting("MyApp", "Startup", "Left"),
        Some("150".to_string())
    );
    interpreter
        .run_source(
            "Attribute VB_Name = \"M\"\n\
             Sub Main()\n\
                 Debug.Print GetSetting(\"MyApp\", \"Startup\", \"Left\", \"0\")\n\
             End Sub\n",
        )
        .expect("interpretation failed");
    assert_eq!(interpreter.output(), &["150"]);
}

#[test]
fn set_settings_backend_switches_to_new_backend() {
    use vb6runtime::state::settings::memory::MemoryBackend;

    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let interpreter = Interpreter::new();

    // Switch to memory backend
    interpreter.set_settings_backend(Box::new(MemoryBackend::new()));

    // Set a value in the memory backend
    settings_state::set("MyApp", "TestSection", "TestKey", "MemValue").unwrap();

    // Verify it's accessible
    let out = run_with_settings(
        "    Debug.Print GetSetting(\"MyApp\", \"TestSection\", \"TestKey\", \"default\")\n",
        |_| {},
    );
    assert_eq!(out, vec!["MemValue"]);

    // Reset backend
    interpreter.reset_settings_backend();
}

#[test]
fn reset_settings_backend_restores_default() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let _store = TempSettingsStore::new();
    let interpreter = Interpreter::new();

    // Set a value in the file backend
    settings_state::set("MyApp", "TestSection", "TestKey", "FileValue").unwrap();

    // Verify it works
    let out = run_with_settings(
        "    Debug.Print GetSetting(\"MyApp\", \"TestSection\", \"TestKey\", \"default\")\n",
        |_| {},
    );
    assert_eq!(out, vec!["FileValue"]);

    // Reset backend (should restore default)
    interpreter.reset_settings_backend();
}

/// The `Stop` test module: a global is set and printed before `Stop`, and
/// another print follows it (which must never run).
const STOP_SOURCE: &str = "Attribute VB_Name = \"M\"\n\
     Dim gX As Integer\n\
     Sub Main()\n        \
     gX = 42\n        \
     Debug.Print \"before\"\n        \
     Stop\n        \
     Debug.Print \"after\"\n    \
     End Sub\n";

#[test]
fn stop_terminates_like_end_outside_a_debugger() {
    let source_file = SourceFile::from_string("scratch.bas", STOP_SOURCE);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    let mut interpreter = Interpreter::new();
    interpreter.run_module(&module).unwrap();

    // Compiled-`.exe` behavior: `Stop` acts like `End`.
    assert!(interpreter.is_terminated());
    assert_eq!(interpreter.output(), vec!["before".to_string()]);
}

#[test]
fn stop_enters_break_mode_with_a_debugger_attached() {
    let source_file = SourceFile::from_string("scratch.bas", STOP_SOURCE);
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();
    let mut interpreter = Interpreter::new();
    interpreter.set_record_debug_snapshots(true);
    let error = interpreter.run_module(&module).unwrap_err();

    // Development-environment behavior: suspend execution (break mode).
    assert!(error.is_debug_pause());
    assert_eq!(error.line, Some(5));
    assert_eq!(error.procedure.as_deref(), Some("Main"));

    // Unlike `End`, no files are closed and no variables cleared.
    assert_eq!(
        interpreter.global("gX").and_then(|v| v.as_i32().ok()),
        Some(42)
    );
    assert_eq!(interpreter.output(), vec!["before".to_string()]);
}
