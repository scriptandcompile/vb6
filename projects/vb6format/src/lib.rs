use anyhow::Result;
use vb6parse::parsers::{CstNode, SyntaxKind};

pub struct FmtSettings {
    pub indent_size: usize,
    pub blank_lines_around_directives: bool,
    pub blank_lines_inside_directives: bool,
}

impl Default for FmtSettings {
    fn default() -> Self {
        Self {
            indent_size: 4,
            blank_lines_around_directives: false,
            blank_lines_inside_directives: false,
        }
    }
}

pub fn fmt_source(source: &str, settings: &FmtSettings) -> Result<String> {
    let line_ending = detect_line_ending(source);

    let parse_result = vb6parse::ConcreteSyntaxTree::from_text("fmt_input", source);
    let (cst_opt, _failures) = parse_result.unpack();
    let cst = cst_opt.ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;
    let root = cst.to_root_node();

    let mut output = String::with_capacity(source.len());
    {
        let mut formatter = CstFormatter {
            output: &mut output,
            indent_level: 0,
            pending_indent: true,
            line_ending,
            settings,
        };
        formatter.walk_node(&root);
    }

    post_process_directives(&mut output, settings, line_ending);
    post_process_type_enum(&mut output, settings, line_ending);

    Ok(output)
}

fn detect_line_ending(source: &str) -> &'static str {
    if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

struct CstFormatter<'a> {
    output: &'a mut String,
    indent_level: usize,
    pending_indent: bool,
    line_ending: &'a str,
    settings: &'a FmtSettings,
}

impl CstFormatter<'_> {
    fn walk_node(&mut self, node: &CstNode) {
        if node.is_token() {
            self.emit_token(node);
            return;
        }

        let is_stmt_list = node.kind() == SyntaxKind::StatementList;
        if is_stmt_list {
            self.indent_level += 1;
        }

        for child in node.children() {
            self.walk_node(child);
        }

        if is_stmt_list {
            self.indent_level = self.indent_level.saturating_sub(1);
        }
    }

    fn emit_token(&mut self, token: &CstNode) {
        match token.kind() {
            SyntaxKind::Newline => {
                self.output.push_str(self.line_ending);
                self.pending_indent = true;
            }
            SyntaxKind::Whitespace => {
                if !self.pending_indent {
                    self.output.push(' ');
                }
            }
            SyntaxKind::ErrorExpectedTokens | SyntaxKind::ErrorMissingTokens => {}
            _ => {
                if self.pending_indent {
                    let indent = self.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        self.output.push(' ');
                    }
                    self.pending_indent = false;
                }
                self.output.push_str(token.text());
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectiveKind {
    If,
    ElseIf,
    Else,
    EndIf,
}

fn classify_directive(trimmed: &str) -> Option<DirectiveKind> {
    if !trimmed.starts_with('#') {
        return None;
    }
    let after_hash = trimmed[1..].trim_start();
    let mut parts = after_hash.split_whitespace();
    let keyword = parts.next()?;

    match keyword {
        "If" if trimmed.contains("Then") => {
            if trimmed.contains("#End If") {
                return None;
            }
            Some(DirectiveKind::If)
        }
        "ElseIf" => Some(DirectiveKind::ElseIf),
        "Else" => {
            if parts.next() == Some("If") {
                None
            } else {
                Some(DirectiveKind::Else)
            }
        }
        "End" => {
            if parts.next() == Some("If") {
                Some(DirectiveKind::EndIf)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_continuation(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.ends_with('_') && !trimmed.ends_with("__")
}

fn post_process_directives(output: &mut String, settings: &FmtSettings, line_ending: &str) {
    let le = line_ending;

    let original: Vec<&str> = if le == "\r\n" {
        output.split("\r\n").collect()
    } else {
        output.split('\n').collect()
    };

    struct DirectiveFrame {
        base_indent: usize,
    }

    let mut result_lines: Vec<String> = Vec::with_capacity(original.len());
    let mut stack: Vec<DirectiveFrame> = Vec::new();
    let mut last_was_blank = false;

    for (i, raw_line) in original.iter().enumerate() {
        let trimmed = raw_line.trim();
        let is_empty = trimmed.is_empty();
        let is_last = i == original.len() - 1;

        if is_empty {
            result_lines.push(String::new());
            last_was_blank = true;
            continue;
        }

        if i > 0 && is_continuation(original[i - 1]) {
            result_lines.push(raw_line.to_string());
            last_was_blank = false;
            continue;
        }

        let current_indent = raw_line.len() - trimmed.len();

        let line;

        if let Some(dir_kind) = classify_directive(trimmed) {
            match dir_kind {
                DirectiveKind::If => {
                    if settings.blank_lines_around_directives && !last_was_blank {
                        result_lines.push(String::new());
                    }

                    let base = if let Some(frame) = stack.last() {
                        frame.base_indent + settings.indent_size
                    } else {
                        current_indent
                    };

                    stack.push(DirectiveFrame { base_indent: base });

                    if settings.blank_lines_inside_directives {
                        result_lines.push(format!("{}{}", " ".repeat(base), trimmed));
                        result_lines.push(String::new());
                        last_was_blank = true;
                        continue;
                    }

                    line = format!("{}{}", " ".repeat(base), trimmed);
                }
                DirectiveKind::ElseIf | DirectiveKind::Else => {
                    if settings.blank_lines_inside_directives && !last_was_blank {
                        result_lines.push(String::new());
                    }

                    let base = stack
                        .last()
                        .map(|f| f.base_indent)
                        .unwrap_or(current_indent);

                    if settings.blank_lines_inside_directives {
                        result_lines.push(format!("{}{}", " ".repeat(base), trimmed));
                        result_lines.push(String::new());
                        last_was_blank = true;
                        continue;
                    }

                    line = format!("{}{}", " ".repeat(base), trimmed);
                }
                DirectiveKind::EndIf => {
                    if settings.blank_lines_inside_directives && !last_was_blank {
                        result_lines.push(String::new());
                    }

                    let base = stack.pop().map(|f| f.base_indent).unwrap_or(current_indent);

                    if settings.blank_lines_around_directives && !is_last {
                        result_lines.push(format!("{}{}", " ".repeat(base), trimmed));
                        result_lines.push(String::new());
                        last_was_blank = true;
                        continue;
                    }

                    line = format!("{}{}", " ".repeat(base), trimmed);
                }
            }
        } else if let Some(frame) = stack.last() {
            let body_indent = frame.base_indent + settings.indent_size;
            line = format!("{}{}", " ".repeat(body_indent), trimmed);
        } else {
            line = raw_line.to_string();
        }

        result_lines.push(line);
        last_was_blank = false;
    }

    // Collapse consecutive blank lines to at most one (ensures idempotency)
    let mut deduped: Vec<&str> = Vec::with_capacity(result_lines.len());
    let mut prev_blank = false;
    for line in &result_lines {
        if line.is_empty() {
            if !prev_blank {
                deduped.push("");
                prev_blank = true;
            }
        } else {
            deduped.push(line);
            prev_blank = false;
        }
    }
    *output = deduped.join(le);
}

fn is_type_enum_start(trimmed: &str) -> bool {
    let mut words = trimmed.split_whitespace();
    let first = match words.next() {
        Some(w) => w,
        None => return false,
    };
    match first {
        "Type" | "Enum" => true,
        "Private" | "Public" | "Friend" => matches!(words.next(), Some("Type") | Some("Enum")),
        _ => false,
    }
}

fn is_type_enum_end(trimmed: &str) -> bool {
    let mut words = trimmed.split_whitespace();
    words.next() == Some("End") && matches!(words.next(), Some("Type") | Some("Enum"))
}

fn post_process_type_enum(output: &mut String, settings: &FmtSettings, line_ending: &str) {
    let le = line_ending;

    let original: Vec<&str> = if le == "\r\n" {
        output.split("\r\n").collect()
    } else {
        output.split('\n').collect()
    };

    let mut result_lines: Vec<String> = Vec::with_capacity(original.len());
    let mut type_enum_stack: Vec<usize> = Vec::new();

    for raw_line in &original {
        let trimmed = raw_line.trim();

        if trimmed.is_empty() {
            result_lines.push(String::new());
            continue;
        }

        let current_indent = raw_line.len() - trimmed.len();

        if is_type_enum_end(trimmed) {
            let base = type_enum_stack.pop().unwrap_or(current_indent);
            let indent = if base < current_indent {
                base
            } else {
                current_indent
            };
            result_lines.push(format!("{}{}", " ".repeat(indent), trimmed));
            continue;
        }

        if is_type_enum_start(trimmed) {
            type_enum_stack.push(current_indent);
            result_lines.push(raw_line.to_string());
            continue;
        }

        if let Some(&base) = type_enum_stack.last() {
            let body_indent = base + settings.indent_size;
            result_lines.push(format!("{}{}", " ".repeat(body_indent), trimmed));
        } else {
            result_lines.push(raw_line.to_string());
        }
    }

    *output = result_lines.join(le);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fmt(source: &str, expected: &str) {
        assert_fmt_with(source, expected, &FmtSettings::default());
    }

    fn assert_fmt_with(source: &str, expected: &str, settings: &FmtSettings) {
        let once = fmt_source(source, settings).unwrap();
        assert_eq!(once, expected, "first format mismatch");
        let twice = fmt_source(&once, settings).unwrap();
        assert_eq!(once, twice, "format is not idempotent on:\n{once:?}");
    }

    fn assert_stable(source: &str) {
        assert_fmt(source, source);
    }

    // -----------------------------------------------------------------------
    // Basic / edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_source() {
        assert_stable("");
    }

    #[test]
    fn test_simple_no_indent() {
        assert_stable("Dim x As Integer\nx = 42\n");
    }

    // -----------------------------------------------------------------------
    // Block indentation (Sub / Function / Property)
    // -----------------------------------------------------------------------

    #[test]
    fn test_sub_body_indent() {
        assert_fmt(
            "Public Sub Foo()\nDim x As Integer\nEnd Sub\n",
            "Public Sub Foo()\n    Dim x As Integer\nEnd Sub\n",
        );
    }

    #[test]
    fn test_function_body_indent() {
        assert_fmt(
            "Public Function Add(a, b)\nAdd = a + b\nEnd Function\n",
            "Public Function Add(a, b)\n    Add = a + b\nEnd Function\n",
        );
    }

    #[test]
    fn test_property_get() {
        assert_fmt(
            "Property Get Name() As String\nName = m_Name\nEnd Property\n",
            "Property Get Name() As String\n    Name = m_Name\nEnd Property\n",
        );
    }

    #[test]
    fn test_nested_sub_indent() {
        let expected = "\
Public Sub Outer()
    Dim x As Integer

    Public Sub Inner()
        Dim y As Integer
    End Sub
End Sub
";
        assert_fmt(
            "Public Sub Outer()\nDim x As Integer\n\nPublic Sub Inner()\nDim y As Integer\nEnd Sub\nEnd Sub\n",
            expected,
        );
    }

    // -----------------------------------------------------------------------
    // Control-flow blocks
    // -----------------------------------------------------------------------

    #[test]
    fn test_if_block() {
        assert_fmt(
            "Sub Foo()\nIf True Then\nx = 1\nEnd If\nEnd Sub\n",
            "Sub Foo()\n    If True Then\n        x = 1\n    End If\nEnd Sub\n",
        );
    }

    #[test]
    fn test_if_else() {
        assert_fmt(
            "Sub Foo()\nIf a Then\nx = 1\nElse\nx = 2\nEnd If\nEnd Sub\n",
            "Sub Foo()\n    If a Then\n        x = 1\n    Else\n        x = 2\n    End If\nEnd Sub\n",
        );
    }

    #[test]
    fn test_if_elseif_else() {
        assert_fmt(
            "Sub Foo()\nIf a Then\nx = 1\nElseIf b Then\nx = 2\nElse\nx = 3\nEnd If\nEnd Sub\n",
            "Sub Foo()\n    If a Then\n        x = 1\n    ElseIf b Then\n        x = 2\n    Else\n        x = 3\n    End If\nEnd Sub\n",
        );
    }

    #[test]
    fn test_for_loop() {
        assert_fmt(
            "Sub Foo()\nFor i = 1 To 10\nTotal = Total + i\nNext\nEnd Sub\n",
            "Sub Foo()\n    For i = 1 To 10\n        Total = Total + i\n    Next\nEnd Sub\n",
        );
    }

    #[test]
    fn test_do_loop() {
        assert_fmt(
            "Sub Foo()\nDo While True\nx = x + 1\nLoop\nEnd Sub\n",
            "Sub Foo()\n    Do While True\n        x = x + 1\n    Loop\nEnd Sub\n",
        );
    }

    #[test]
    fn test_while_wend() {
        assert_fmt(
            "Sub Foo()\nWhile x < 10\nx = x + 1\nWend\nEnd Sub\n",
            "Sub Foo()\n    While x < 10\n        x = x + 1\n    Wend\nEnd Sub\n",
        );
    }

    #[test]
    fn test_with_block() {
        assert_fmt(
            "Sub Foo()\nWith obj\n.Name = \"bar\"\nEnd With\nEnd Sub\n",
            "Sub Foo()\n    With obj\n        .Name = \"bar\"\n    End With\nEnd Sub\n",
        );
    }

    #[test]
    fn test_select_case() {
        assert_fmt(
            "Sub Foo()\nSelect Case x\nCase 1\nDoOne\nCase 2\nDoTwo\nCase Else\nDoDefault\nEnd Select\nEnd Sub\n",
            "Sub Foo()\n    Select Case x\n    Case 1\n        DoOne\n    Case 2\n        DoTwo\n    Case Else\n        DoDefault\n    End Select\nEnd Sub\n",
        );
    }

    // -----------------------------------------------------------------------
    // Single-line If (must not be treated as block)
    // -----------------------------------------------------------------------

    #[test]
    fn test_single_line_if() {
        assert_stable("Sub Foo()\n    If True Then x = 1\nEnd Sub\n");
    }

    // -----------------------------------------------------------------------
    // Line continuation
    // -----------------------------------------------------------------------

    #[test]
    fn test_continuation() {
        assert_fmt(
            "Sub Foo()\nx = 1 + _\n        2 + _\n        3\nEnd Sub\n",
            "Sub Foo()\n    x = 1 + _\n    2 + _\n    3\nEnd Sub\n",
        );
    }

    // -----------------------------------------------------------------------
    // Type / Enum body indentation
    // -----------------------------------------------------------------------

    #[test]
    fn test_type_body() {
        assert_fmt(
            "Public Type MyType\nx As Integer\ny As String\nEnd Type\n",
            "Public Type MyType\n    x As Integer\n    y As String\nEnd Type\n",
        );
    }

    #[test]
    fn test_private_type() {
        assert_fmt(
            "Private Type MyType\nx As Integer\nEnd Type\n",
            "Private Type MyType\n    x As Integer\nEnd Type\n",
        );
    }

    #[test]
    fn test_enum_body() {
        assert_fmt(
            "Enum MyEnum\na = 1\nb = 2\nEnd Enum\n",
            "Enum MyEnum\n    a = 1\n    b = 2\nEnd Enum\n",
        );
    }

    // -----------------------------------------------------------------------
    // Compiler directives
    // -----------------------------------------------------------------------

    #[test]
    fn test_directive_simple() {
        assert_fmt(
            "Sub Foo()\n#If DEBUG Then\nDebug.Print \"hi\"\n#End If\nEnd Sub\n",
            "Sub Foo()\n    #If DEBUG Then\n        Debug.Print \"hi\"\n    #End If\nEnd Sub\n",
        );
    }

    #[test]
    fn test_directive_with_else() {
        assert_fmt(
            "Sub Foo()\n#If A Then\nx = 1\n#ElseIf B Then\nx = 2\n#Else\nx = 3\n#End If\nEnd Sub\n",
            "Sub Foo()\n    #If A Then\n        x = 1\n    #ElseIf B Then\n        x = 2\n    #Else\n        x = 3\n    #End If\nEnd Sub\n",
        );
    }

    #[test]
    fn test_directive_nested() {
        assert_fmt(
            "Sub Foo()\n#If A Then\nx = 1\n#If B Then\ny = 2\n#End If\nz = 3\n#End If\nEnd Sub\n",
            "Sub Foo()\n    #If A Then\n        x = 1\n        #If B Then\n            y = 2\n        #End If\n        z = 3\n    #End If\nEnd Sub\n",
        );
    }

    #[test]
    fn test_directive_top_level() {
        assert_fmt(
            "#If Win64 Then\nPtrSafe\n#End If\n",
            "#If Win64 Then\n    PtrSafe\n#End If\n",
        );
    }

    // -----------------------------------------------------------------------
    // Directive blank-line settings
    // -----------------------------------------------------------------------

    #[test]
    fn test_directive_blank_lines_around() {
        let settings = FmtSettings {
            blank_lines_around_directives: true,
            ..FmtSettings::default()
        };
        let input = "Sub Foo()\n#If DEBUG Then\nDebug.Print \"hi\"\n#End If\nEnd Sub\n";
        let expected = "\
Sub Foo()

    #If DEBUG Then
        Debug.Print \"hi\"
    #End If

End Sub
";
        assert_fmt_with(input, expected, &settings);
    }

    #[test]
    fn test_directive_blank_lines_inside() {
        let settings = FmtSettings {
            blank_lines_inside_directives: true,
            ..FmtSettings::default()
        };
        let input = "Sub Foo()\n#If DEBUG Then\nDebug.Print \"hi\"\n#End If\nEnd Sub\n";
        let expected = "\
Sub Foo()
    #If DEBUG Then

        Debug.Print \"hi\"

    #End If
End Sub
";
        assert_fmt_with(input, expected, &settings);
    }

    // -----------------------------------------------------------------------
    // Comments
    // -----------------------------------------------------------------------

    #[test]
    fn test_comments_preserved() {
        assert_stable(
            "' this is a comment\nSub Foo()\n    ' inside comment\n    x = 1\nEnd Sub\n' trailing\n",
        );
    }

    #[test]
    fn test_comment_only_lines() {
        assert_stable("' just a comment\n' another one\n");
    }

    // -----------------------------------------------------------------------
    // Line endings
    // -----------------------------------------------------------------------

    #[test]
    fn test_crlf_preserved() {
        assert_fmt(
            "Sub Foo()\r\nDim x As Integer\r\nEnd Sub\r\n",
            "Sub Foo()\r\n    Dim x As Integer\r\nEnd Sub\r\n",
        );
    }

    // -----------------------------------------------------------------------
    // Custom indent size
    // -----------------------------------------------------------------------

    #[test]
    fn test_custom_indent_size() {
        let settings = FmtSettings {
            indent_size: 2,
            ..FmtSettings::default()
        };
        assert_fmt_with(
            "Sub Foo()\nx = 1\nEnd Sub\n",
            "Sub Foo()\n  x = 1\nEnd Sub\n",
            &settings,
        );
    }

    // -----------------------------------------------------------------------
    // Batch idempotency check on already-formatted snippets
    // -----------------------------------------------------------------------

    #[test]
    fn test_idempotent_on_formatted() {
        let cases = [
            "",
            "Dim x As Integer\n",
            "Sub Foo()\n    x = 1\nEnd Sub\n",
            "Sub Foo()\n    If True Then\n        x = 1\n    End If\nEnd Sub\n",
            "#If DEBUG Then\n    x = 1\n#End If\n",
            "Type T\n    x As Integer\nEnd Type\n",
            "Enum E\n    A\n    B\nEnd Enum\n",
            "Sub Foo()\n    x = 1 + _\n    2\nEnd Sub\n",
            "' comment\nSub Foo()\n    x = 1\nEnd Sub\n' comment\n",
        ];
        for src in &cases {
            assert_stable(src);
        }
    }
}
