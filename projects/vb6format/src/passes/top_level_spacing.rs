use std::cell::{Cell, RefCell};

use crate::context::Context;
use crate::passes::{FormatPass, TokenBuffer};
use crate::settings::FmtSettings;
use vb6parse::SyntaxKind;
use vb6parse::parsers::CstNode;

pub struct TopLevelSpacingPass<'a> {
    settings: &'a FmtSettings,
    initialized: Cell<bool>,
    current_line: Cell<usize>,
    at_line_start: Cell<bool>,
    insert_before_lines: RefCell<Vec<usize>>,
}

impl<'a> TopLevelSpacingPass<'a> {
    pub fn new(settings: &'a FmtSettings) -> Self {
        Self {
            settings,
            initialized: Cell::new(false),
            current_line: Cell::new(0),
            at_line_start: Cell::new(true),
            insert_before_lines: RefCell::new(Vec::new()),
        }
    }

    fn compute_insertion_lines(&self, root_text: &str) {
        let normalized = root_text.replace("\r\n", "\n");
        let lines: Vec<&str> = normalized.split('\n').collect();

        let mut insertion_lines = Vec::new();
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
                    let previous_is_blank = index > 0 && is_blank_line(lines[index - 1]);
                    if !previous_is_blank {
                        insertion_lines.push(index);
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

                index = block_end + 1;
                continue;
            }

            index += 1;
        }

        *self.insert_before_lines.borrow_mut() = insertion_lines;
    }
}

impl FormatPass for TopLevelSpacingPass<'_> {
    fn on_node_enter(&self, node: &CstNode, _context: &mut Context) {
        if self.initialized.get() {
            return;
        }

        self.initialized.set(true);
        self.current_line.set(0);
        self.at_line_start.set(true);

        if self.settings.blank_lines_around_top_level {
            self.compute_insertion_lines(node.text());
        }
    }

    fn on_token(&self, token: &CstNode, context: &mut Context, buffer: &mut TokenBuffer) {
        if self.settings.blank_lines_around_top_level {
            let is_skippable = matches!(
                token.kind(),
                SyntaxKind::Whitespace
                    | SyntaxKind::Newline
                    | SyntaxKind::ErrorExpectedTokens
                    | SyntaxKind::ErrorMissingTokens
            );

            if self.at_line_start.get() && !is_skippable {
                let line = self.current_line.get();
                if self.insert_before_lines.borrow().contains(&line) {
                    buffer.prefix.push_str(context.line_ending());
                }
                self.at_line_start.set(false);
            }
        }

        if token.kind() == SyntaxKind::Newline {
            self.current_line
                .set(self.current_line.get().saturating_add(1));
            self.at_line_start.set(true);
        }
    }
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
    if trimmed.is_empty() || trimmed.starts_with('\'') || trimmed.starts_with("REM") {
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
    trimmed.starts_with('\'') || trimmed.starts_with("REM")
}

fn is_blank_line(line: &str) -> bool {
    line.trim().is_empty()
}
