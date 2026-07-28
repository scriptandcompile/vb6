# vb6format

Library for formatting VB6 source files. Used by `aspen fmt` and available
as a standalone crate for tools that need to reformat VB6 code programmatically.

## Usage

```rust
use vb6format::{fmt_source, FmtSettings};

let source = "Sub Foo()\nIf True Then\nx=1\nEnd If\nEnd Sub";
let settings = FmtSettings { indent_size: 4 };
let formatted = fmt_source(source, &settings)?;
```

## Formatting Rules

- Re-indents block bodies: `Sub`/`Function`/`Property`, `If`/`ElseIf`/`Else`, `For`, `Do`/`Loop`, `While`/`Wend`, `With`, `Select`/`Case`, `Type`, `Enum`
- Single-line `If Then` statements are preserved on one line
- Closing keywords (`End If`, `End Sub`, `Loop`, `Next`, etc.) are un-indented to the parent level
- `Else`, `ElseIf`, and `Case` act as both closers and openers, matching VB6 convention
- Blank lines and comments are preserved

## Dependencies

- `vb6parse` — CST validation via `ConcreteSyntaxTree::from_text`
