# vb6harness

A differential test harness for the VB6 interpreter and compiler. It runs a
corpus of VB6 test modules through each engine (`vb6interpret`, the future
`vb6compile`, and the legacy `VB6.exe`) and compares their `Print` output
against committed golden files. Everything runs from Linux or Windows; the
legacy compiler is exercised natively on Windows or through Wine on Linux.

All commands below run from the workspace root (the directory containing the
top-level `Cargo.toml`).

## Layout

```
projects/
  vb6harness/         this crate
    src/engines/      vb6interpret / vb6compile / VB6.exe backends
tests/
  suite/              the test corpus (.bas modules)
    basic/
  golden/             committed expected output, one .txt per test module
```

## Quick start (no VB6 needed)

Run every corpus module through `vb6interpret` and compare against the goldens:

```
cargo run -p vb6harness -- run
```

This is the same check the CI gate runs:

```
cargo test -p vb6harness
cargo test --workspace --locked
```

The harness crate has a `golden_suite_matches_interpreter` test (in
`src/main.rs`) that fails the build if interpreter output diverges from the
committed goldens.

Exit code is `0` only when every engine/test combination passes; otherwise the
mismatches are printed and the exit code is `1`.

## Running on Linux

Interpreter only (no VB6 installed):

```
cargo run -p vb6harness -- run
```

Also run the legacy compiler through Wine (requires a Wine prefix with VB6):

```
cargo run -p vb6harness -- run --vb6 --vb6-path "$HOME/.wine/drive_c/Program Files (x86)/Microsoft Visual Studio/VB6/VB6.EXE"
```

You can set `VB6_PATH` instead of passing `--vb6-path`:

```
export VB6_PATH="$HOME/.wine/drive_c/Program Files (x86)/Microsoft Visual Studio/VB6/VB6.EXE"
cargo run -p vb6harness -- run --vb6
```

Wine path notes:
- Pass the Wine-side path to `VB6.EXE` (the harness invokes `wine <path>`).
- The output file and generated project are written to
  `target/harness-work/<stem>/`, which Wine maps from the Linux path directly,
  so no extra drive-letter mapping is needed.

## Running on Windows

Interpreter only:

```
cargo run -p vb6harness -- run
```

Also run the legacy compiler natively (requires VB6 to be installed):

```
cargo run -p vb6harness -- run --vb6 --vb6-path "C:\Program Files (x86)\Microsoft Visual Studio\VB6\VB6.EXE"
```

Or via the environment variable:

```
$env:VB6_PATH = "C:\Program Files (x86)\Microsoft Visual Studio\VB6\VB6.EXE"
cargo run -p vb6harness -- run --vb6
```

## Selecting tests

Filter by category (defined per module by the `CATEGORY` header directive) or
by a path substring:

```
cargo run -p vb6harness -- run --category basic
cargo run -p vb6harness -- run --test arithmetic
```

Combine with `--vb6` to narrow a compiler run:

```
cargo run -p vb6harness -- run --vb6 --test variables
```

## Golden files

Golden files live in `tests/golden/<stem>.txt` (one line per expected output
line). Example for `tests/suite/basic/variables.bas`:

```text
100005
world
hello world
3.375
3.75
True
False
3
```

Comparison is per line:

- Lines are whitespace-trimmed before comparing. Real VB6 pads numbers when
  writing with `Print #`, so leading/trailing spaces and column widths are
  ignored.
- Lines that parse as numbers are compared as `f64` within a tolerance so
  equivalent float spellings match (default `1e-12`, overridable per test with
  the `TOLERANCE` directive). `0.30000000000000004` matches `0.3`.
- Everything else must match exactly, including order and line count.

Regenerate goldens from an engine's current output with `update-golden`. The
default source is the interpreter:

```
cargo run -p vb6harness -- update-golden
```

To stamp goldens from the legacy compiler's real output (Windows or Wine):

```
cargo run -p vb6harness -- update-golden --engine vb6 --vb6-path <path/to/VB6.EXE>
```

Always review the diff before committing regenerated goldens:

```
git diff -- tests/golden
```

## Writing corpus tests

Test modules are plain, VB6-native `.bas` files. Output goes to a file with
`Print #1, <expr>`, one value per line:

```vb
Attribute VB_Name = "Variables"
Option Explicit

' TEST: Variables and constants
' CATEGORY: basic
' DESCRIPTION: Declarations, assignment, and typed arithmetic.

Const GREETING = "hello"

Sub Main()
    Dim x As Integer
    Dim firstName As String
    x = 5
    firstName = "world"
    Print #1, x + 5
    Print #1, firstName
    Print #1, GREETING & " " & firstName
End Sub
```

Because the corpus is VB6-native, the file compiled by `VB6.exe` is the
committed file itself; the harness only wraps the entry point (see below).

Optional header directives appear in comment lines before the first
`Sub`/`Function` and take the form `' KEY: value`:

| Directive          | Meaning                                                       |
| ------------------ | ------------------------------------------------------------- |
| `TEST`             | Display name (defaults to the file name).                     |
| `CATEGORY`         | Grouping tag, matched by `--category`.                        |
| `TIMEOUT`          | Per-engine timeout in seconds (default 30).                   |
| `TOLERANCE`        | Numeric comparison tolerance for this test.                   |
| `KNOWN_ISSUE`      | Expected failure; the reason is reported.                     |
| `SKIP_VB6`         | Skip the VB6 engine; value is the reason.                     |
| `SKIP_INTERPRETER` | Skip the interpreter; value is the reason.                    |
| `SKIP_COMPILER`    | Skip the compiler; value is the reason.                       |

Unknown directives are ignored so the files stay valid VB6.

Two interpreter-only extensions let corpus files probe console behavior; tag
those modules `SKIP_VB6` because bare `Print` does not compile in a standard
module and `Debug.Print` is a no-op in a compiled exe:

```vb
Sub Main()
    Print "bare print"     ' console output extension
    Debug.Print "debug"    ' console output extension
End Sub
```

## Additional options

```
--verbose     print every result, not just failures
--junit FILE  write a JUnit XML report (written on both success and failure)
--compiler    also run the vb6compile engine (currently always skipped)
--suite DIR   suite directory, absolute or relative to the workspace root
--golden-dir DIR   golden directory (default tests/golden)
```

`VB6_WORKSPACE_ROOT` overrides the workspace root discovery (normally located
by walking up from the harness crate to the root `Cargo.toml`).

## How the VB6 engine works

For each module the engine builds a throwaway project under
`target/harness-work/<stem>/`:

1. Copies the committed `.bas`, renaming `Sub Main` to `Sub TestMain`.
2. Adds a generated `startup.bas` that wraps the entry point:
   `Open "out.txt" For Output As #1` / `TestMain` / `Close #1`.
3. Compiles with `VB6.exe /make test.vbp` (via `wine` on non-Windows hosts).
4. Runs the produced `test.exe` from the build directory and reads `out.txt`.
5. Returns the output lines for comparison against the golden file.

The committed corpus files are never modified.

## Limitations

- `--compiler` is not implemented yet: `vb6compile` does not emit program
  output, so that engine always reports `Skipped`.
- The real-VB6 path can only be exercised where VB6 runs: a Windows machine or
  a Wine prefix with VB6 installed. The golden gate (`cargo test`) does not
  require it.
