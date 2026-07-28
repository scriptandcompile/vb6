# vb6format

Library for formatting VB6 source files. Used by `aspen fmt` and available
as a standalone crate for tools that need to reformat VB6 code programmatically.

## Usage

```rust
use vb6format::{fmt_source, FmtSettings};

let source = "Sub Foo()\nIf True Then\nx=1\nEnd If\nEnd Sub";
let settings = FmtSettings::default();
let formatted = fmt_source(source, &settings)?;
```

## Settings

| Field | Type | Default | Description |
|---|---|---|---|
| `indent_size` | `usize` | `4` | Spaces per indent level |
| `blank_lines_around_directives` | `bool` | `false` | Insert blank line before `#If` and after `#End If` |
| `blank_lines_inside_directives` | `bool` | `false` | Insert blank lines between `#If`/`#ElseIf`/`#Else`/`#End If` and their bodies |

## Formatting Rules

- Re-indents block bodies: `Sub`/`Function`/`Property`, `If`/`ElseIf`/`Else`, `For`,
  `Do`/`Loop`, `While`/`Wend`, `With`, `Select`/`Case`, `Type`, `Enum`
- Compiler directives (`#If`/`#ElseIf`/`#Else`/`#End If`) are indented correctly
- Single-line `If Then` statements are preserved on one line
- Closing keywords (`End If`, `End Sub`, `Loop`, `Next`, etc.) are un-indented
  to the parent level
- `Else`, `ElseIf`, and `Case` act as both closers and openers
- Blank lines and comments are preserved

## Dependencies

- `vb6parse` — CST validation via `ConcreteSyntaxTree::from_text`
