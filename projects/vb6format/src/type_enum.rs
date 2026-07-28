use crate::settings::FmtSettings;

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

pub(super) fn post_process_type_enum(
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
