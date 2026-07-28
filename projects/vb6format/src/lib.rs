use anyhow::Result;

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
    let _parse_result = vb6parse::ConcreteSyntaxTree::from_text("fmt_input", source);
    let (cst_opt, _failures) = _parse_result.unpack();

    let _cst = cst_opt.ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;

    Ok(reindent_source(source, settings))
}

fn reindent_source(source: &str, settings: &FmtSettings) -> String {
    let mut output = String::new();
    let mut indent = 0usize;
    let mut last_was_blank = false;

    let lines: Vec<&str> = source.lines().collect();
    let source_ends_with_newline = source.ends_with('\n');
    let len = lines.len();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let is_last = i == len - 1;

        if trimmed.is_empty() {
            output.push('\n');
            last_was_blank = true;
            continue;
        }

        let first_word = trimmed.split([' ', '\t']).next().unwrap_or("");
        let is_directive = first_word.starts_with('#');
        let dir_key = first_word.strip_prefix('#').unwrap_or(first_word);

        if is_directive {
            if settings.blank_lines_around_directives && dir_key == "If" {
                if !last_was_blank && i > 0 {
                    output.push('\n');
                }
            }

            if settings.blank_lines_inside_directives
                && matches!(dir_key, "ElseIf" | "Else" | "End")
                && !last_was_blank
            {
                output.push('\n');
            }
        }

        let is_decrease =
            is_closing_keyword(first_word) && (i == 0 || !is_continuation(lines[i - 1]));

        if is_decrease {
            indent = indent.saturating_sub(1);
        }

        output.push_str(&" ".repeat(indent * settings.indent_size));
        output.push_str(trimmed);
        if !is_last || source_ends_with_newline {
            output.push('\n');
        }
        last_was_blank = false;

        let is_increase = is_opening_keyword(trimmed, first_word)
            && !is_single_line_if(trimmed)
            && (is_last || !is_continuation(trimmed));

        if is_increase {
            indent += 1;
        }

        if is_directive {
            let has_next = i + 1 < len;
            let next_blank = has_next && lines[i + 1].trim().is_empty();

            if settings.blank_lines_inside_directives
                && matches!(dir_key, "If" | "ElseIf" | "Else")
                && has_next
                && !next_blank
            {
                output.push('\n');
                last_was_blank = true;
            }

            if settings.blank_lines_around_directives && dir_key == "End" && has_next && !next_blank
            {
                output.push('\n');
                last_was_blank = true;
            }
        }
    }

    output
}

fn is_continuation(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.ends_with('_') && !trimmed.ends_with("__")
}

fn is_closing_keyword(first_word: &str) -> bool {
    let w = first_word.strip_prefix('#').unwrap_or(first_word);
    matches!(
        w,
        "End" | "Next" | "Loop" | "Wend" | "Else" | "ElseIf" | "Case"
    )
}

fn is_opening_keyword(trimmed: &str, first_word: &str) -> bool {
    let upper = first_word.strip_prefix('#').unwrap_or(first_word);

    if matches!(
        upper,
        "Sub" | "Function" | "Property" | "Type" | "Enum"
    ) {
        return true;
    }

    if matches!(upper, "For" | "Do" | "While" | "With") {
        return true;
    }

    if upper == "If" && trimmed.contains("Then") {
        return true;
    }

    if upper == "Select" && trimmed.contains("Case") {
        return true;
    }

    if upper == "Else" || upper == "ElseIf" {
        return true;
    }

    if upper == "Case" {
        return true;
    }

    if matches!(upper, "Private" | "Public" | "Friend") {
        if let Some(second) = trimmed.split([' ', '\t']).nth(1) {
            if matches!(second, "Sub" | "Function" | "Property" | "Type" | "Enum") {
                return true;
            }
        }
    }

    false
}

fn is_single_line_if(trimmed: &str) -> bool {
    let trimmed = trimmed.trim();
    if trimmed.starts_with('#') {
        return false;
    }
    if !trimmed.starts_with("If ") && !trimmed.starts_with("If\t") {
        return false;
    }

    let has_then = trimmed.contains(" Then ");
    if !has_then {
        return false;
    }

    let after_then = trimmed.split(" Then ").nth(1).unwrap_or("");
    let has_newline_after_then = after_then.contains('\n');

    !has_newline_after_then
}
