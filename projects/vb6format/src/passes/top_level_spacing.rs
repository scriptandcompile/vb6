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
        let mut previous_category: Option<TopLevelCategory> = None;
        let mut index = 0;

        while index < lines.len() {
            let line = lines[index];
            let is_trailing = line.is_empty() && index == lines.len() - 1;
            if is_trailing {
                break;
            }

            if let Some(current_category) = top_level_construct_block_start_category(&lines, index)
            {
                if let Some(previous) = previous_category {
                    let previous_is_blank = index > 0 && is_blank_line(lines[index - 1]);
                    if !previous_is_blank && should_insert_between(previous, current_category) {
                        insertion_lines.push(index);
                    }
                }

                previous_category = Some(current_category);

                let mut block_end = index;
                if is_comment_line(line) {
                    let mut probe = index + 1;
                    while probe < lines.len() {
                        let candidate = lines[probe];
                        if is_blank_line(candidate) {
                            probe += 1;
                            continue;
                        }

                        if top_level_construct_category(candidate).is_some() {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopLevelCategory {
    Option,
    Declare,
    Other,
}

fn top_level_construct_block_start_category(
    lines: &[&str],
    start_index: usize,
) -> Option<TopLevelCategory> {
    let line = lines[start_index];
    if let Some(category) = top_level_construct_category(line) {
        return Some(category);
    }

    if !is_comment_line(line) {
        return None;
    }

    let mut probe = start_index + 1;
    while probe < lines.len() {
        let candidate = lines[probe];
        if is_blank_line(candidate) {
            probe += 1;
            continue;
        }

        return top_level_construct_category(candidate);
    }

    None
}

fn top_level_construct_category(line: &str) -> Option<TopLevelCategory> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('\'') || trimmed.starts_with("REM") {
        return None;
    }

    let mut words = trimmed.split_whitespace();
    let first = words.next()?;

    match first.to_ascii_lowercase().as_str() {
        "option" => Some(TopLevelCategory::Option),
        "declare" => Some(TopLevelCategory::Declare),
        "sub" | "function" | "property" | "event" | "enum" | "type" => {
            Some(TopLevelCategory::Other)
        }
        "private" | "public" | "friend" => {
            let next = words.next()?;

            match next.to_ascii_lowercase().as_str() {
                "declare" => Some(TopLevelCategory::Declare),
                "sub" | "function" | "property" | "event" => Some(TopLevelCategory::Other),
                _ => None,
            }
        }
        _ => None,
    }
}

fn should_insert_between(previous: TopLevelCategory, current: TopLevelCategory) -> bool {
    !matches!(
        (previous, current),
        (TopLevelCategory::Declare, TopLevelCategory::Declare)
            | (TopLevelCategory::Option, TopLevelCategory::Option)
    )
}

fn is_comment_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('\'') || trimmed.starts_with("REM")
}

fn is_blank_line(line: &str) -> bool {
    line.trim().is_empty()
}
