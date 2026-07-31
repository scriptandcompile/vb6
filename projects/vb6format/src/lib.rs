pub mod context;
mod cst_formatter;
pub mod rewrite;
pub mod settings;

pub use settings::FmtSettings;

use anyhow::Result;
use vb6parse::ConcreteSyntaxTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    CrLf,
    Lf,
}

pub fn fmt_source(source: &str, settings: &FmtSettings) -> Result<String> {
    let line_ending = cst_formatter::detect_line_ending(source);

    let parse_result = ConcreteSyntaxTree::from_text("fmt_input", source);
    let (cst_opt, _failures) = parse_result.unpack();
    let cst = cst_opt.ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;
    let root = cst.to_root_node();

    let mut output = String::with_capacity(source.len());
    {
        let mut formatter = cst_formatter::CstFormatter::new(&mut output, line_ending, settings);
        formatter.walk_node(&root);
    }

    if settings.blank_lines_around_top_level {
        insert_blank_lines_around_top_level(&mut output, &line_ending);
    }

    deduplicate_blank_lines(&mut output, &line_ending);

    Ok(output)
}

fn insert_blank_lines_around_top_level(output: &mut String, line_ending: &LineEnding) {
    let lines: Vec<&str> = if matches!(line_ending, LineEnding::CrLf) {
        output.split("\r\n").collect()
    } else {
        output.split('\n').collect()
    };

    let mut rewritten = Vec::<String>::with_capacity(lines.len() + 4);
    let mut saw_top_level_block = false;
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];
        let is_trailing = line.is_empty() && index == lines.len() - 1;
        if is_trailing {
            break;
        }

        if is_top_level_construct_block_start(&lines, index) {
            if saw_top_level_block {
                let previous_is_blank = rewritten
                    .last()
                    .is_some_and(|previous| is_blank_line(previous));
                if !previous_is_blank {
                    rewritten.push(String::new());
                }
            }
            saw_top_level_block = true;

            let mut block_end = index;
            if is_comment_line(line) {
                let mut probe = index + 1;
                while probe < lines.len() {
                    let candidate = lines[probe];
                    if is_blank_line(candidate) {
                        probe += 1;
                        continue;
                    }

                    if is_top_level_construct_line(candidate) {
                        block_end = probe;
                        break;
                    }

                    block_end = probe;
                    probe += 1;
                }
            }

            for item in lines.iter().take(block_end + 1).skip(index) {
                rewritten.push(item.to_string());
            }
            index = block_end + 1;
            continue;
        }

        rewritten.push(line.to_string());
        index += 1;
    }

    *output = rewritten.join(match *line_ending {
        LineEnding::CrLf => "\r\n",
        LineEnding::Lf => "\n",
    });
}

fn is_top_level_construct_block_start(lines: &[&str], start_index: usize) -> bool {
    let line = lines[start_index];
    if is_top_level_construct_line(line) {
        return true;
    }

    if !is_comment_line(line) {
        return false;
    }

    let mut probe = start_index + 1;
    while probe < lines.len() {
        let candidate = lines[probe];
        if is_blank_line(candidate) {
            probe += 1;
            continue;
        }

        return is_top_level_construct_line(candidate);
    }

    false
}

fn is_top_level_construct_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with("'") || trimmed.starts_with("REM") {
        return false;
    }

    let Some(first) = trimmed.split_whitespace().next() else {
        return false;
    };

    match first.to_ascii_lowercase().as_str() {
        "sub" | "function" | "property" | "declare" | "event" | "enum" | "type" => true,
        "private" | "public" | "friend" => trimmed.split_whitespace().nth(1).is_some_and(|next| {
            matches!(
                next.to_ascii_lowercase().as_str(),
                "sub" | "function" | "property" | "declare" | "event"
            )
        }),
        _ => false,
    }
}

fn is_comment_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("'") || trimmed.starts_with("REM")
}

fn is_blank_line(line: &str) -> bool {
    line.trim().is_empty()
}

fn deduplicate_blank_lines(output: &mut String, line_ending: &LineEnding) {
    let lines: Vec<&str> = if matches!(line_ending, LineEnding::CrLf) {
        output.split("\r\n").collect()
    } else {
        output.split('\n').collect()
    };

    let mut result = String::with_capacity(output.len());
    let mut prev_blank = false;

    for (i, line) in lines.iter().enumerate() {
        let is_trailing = line.is_empty() && i == lines.len() - 1;
        if is_trailing {
            break;
        }
        if line.is_empty() {
            if !prev_blank {
                result.push_str(match line_ending {
                    LineEnding::CrLf => "\r\n",
                    LineEnding::Lf => "\n",
                });
                prev_blank = true;
            }
        } else {
            result.push_str(line);
            result.push_str(match line_ending {
                LineEnding::CrLf => "\r\n",
                LineEnding::Lf => "\n",
            });
            prev_blank = false;
        }
    }
    *output = result;
}
