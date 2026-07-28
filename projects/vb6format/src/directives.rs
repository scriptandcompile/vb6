use crate::settings::FmtSettings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectiveKind {
    If,
    ElseIf,
    Else,
    EndIf,
}

pub(super) fn classify_directive(trimmed: &str) -> Option<DirectiveKind> {
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

pub(super) fn post_process_directives(
    output: &mut String,
    settings: &FmtSettings,
    line_ending: &str,
) {
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
