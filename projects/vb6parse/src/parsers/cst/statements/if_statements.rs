//! `If`/`Then`/`Else`/`ElseIf` statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 conditional statements:
//! - `If`/`Then`/`Else` statements (both single-line and multi-line)
//! - `ElseIf` clauses
//! - `Else` clauses

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse an `If` statement: `If` condition `Then` ... `End If`
    /// Handles both single-line and multi-line `If` statements.
    ///
    /// If a compiler directive prefix (`#`) is detected before `If`,
    /// delegates to `parse_compiler_directive` instead.
    ///
    /// `IfStatement`
    /// ├─ `If` keyword
    /// ├─ condition tokens
    /// ├─ `Then` keyword
    /// ├─ body tokens
    /// ├─ `ElseIfClause` (if present)
    /// │  ├─ `ElseIf` keyword
    /// │  ├─ condition tokens
    /// │  ├─ `Then` keyword
    /// │  └─ body tokens
    /// ├─ `ElseClause` (if present)
    /// │  ├─ `Else` keyword
    /// │  └─ body tokens
    /// ├─ `End` keyword
    /// └─ `If` keyword
    ///
    pub(crate) fn parse_if_statement(&mut self) {
        if self.at_compiler_directive_keyword(Token::IfKeyword) {
            self.parse_compiler_directive();
            return;
        }

        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::IfStatement.to_raw());
        self.consume_whitespace();
        self.consume_token(); // If
        self.consume_whitespace();
        self.parse_expression();
        self.consume_whitespace();

        if self.at_token(Token::ThenKeyword) {
            self.consume_token();
        }
        self.consume_whitespace();

        // Skip trailing comment on the Then line — a comment after Then
        // does not constitute a single-line If body.
        while self.at_token(Token::EndOfLineComment) || self.at_token(Token::RemComment) {
            self.consume_token();
            self.consume_whitespace();
        }

        // Check if single-line If
        let is_single_line = !self.at_token(Token::Newline) && !self.is_at_end();

        if is_single_line {
            self.parse_single_line_if_statement();
            return;
        }

        // Multi-line If: parse body and ElseIf/Else clauses
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        // Parse If body - the recursive call here is now safe because
        // parse_statement_list handles control flow iteratively
        self.parse_statement_list(|parser| {
            (parser.at_token(Token::EndKeyword)
                && parser.peek_next_keyword() == Some(Token::IfKeyword))
                || parser.at_compiler_end_if_directive()
                || parser.at_token(Token::ElseIfKeyword)
                || parser.at_compiler_directive_keyword(Token::ElseIfKeyword)
                || parser.at_token(Token::ElseKeyword)
                || parser.at_compiler_directive_keyword(Token::ElseKeyword)
        });

        // Handle ElseIf and Else clauses
        while !self.is_at_end() {
            if self.at_compiler_directive_keyword(Token::ElseIfKeyword) {
                self.consume_compiler_directive_prefix();
            }

            if self.at_token(Token::ElseIfKeyword) {
                self.builder.start_node(SyntaxKind::ElseIfClause.to_raw());
                self.consume_token(); // ElseIf
                self.consume_whitespace();
                self.parse_expression();
                self.consume_whitespace();

                if self.at_token(Token::ThenKeyword) {
                    self.consume_token();
                }
                self.consume_whitespace();

                if self.at_token(Token::Newline) {
                    self.consume_token();
                }

                // Parse ElseIf body
                self.parse_statement_list(|parser| {
                    parser.at_token(Token::ElseIfKeyword)
                        || parser.at_compiler_directive_keyword(Token::ElseIfKeyword)
                        || parser.at_token(Token::ElseKeyword)
                        || parser.at_compiler_directive_keyword(Token::ElseKeyword)
                        || (parser.at_token(Token::EndKeyword)
                            && parser.peek_next_keyword() == Some(Token::IfKeyword))
                        || parser.at_compiler_end_if_directive()
                });

                self.builder.finish_node(); // ElseIfClause
            } else if self.at_compiler_directive_keyword(Token::ElseKeyword) {
                self.consume_compiler_directive_prefix();
            } else if self.at_token(Token::ElseKeyword) {
                self.builder.start_node(SyntaxKind::ElseClause.to_raw());
                self.consume_token(); // Else
                self.consume_whitespace();

                if self.at_token(Token::Newline) {
                    self.consume_token();
                }

                // Parse Else body
                self.parse_statement_list(|parser| {
                    parser.at_token(Token::EndKeyword)
                        && parser.peek_next_keyword() == Some(Token::IfKeyword)
                        || parser.at_compiler_end_if_directive()
                });

                self.builder.finish_node(); // ElseClause
            } else {
                break;
            }
        }

        // Consume "End If"
        if self.at_compiler_end_if_directive() {
            self.consume_compiler_directive_prefix();
        }

        if self.at_token(Token::EndKeyword) {
            self.consume_token();
            self.consume_whitespace();
            self.consume_token(); // If
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node(); // IfStatement
    }

    /// Parse a compiler directive (`#If ... [#ElseIf ...] [#Else ...] #End If`).
    ///
    /// Creates a `CompilerDirective` with:
    /// - `CompilerIfClause` wrapping the `#If` condition line
    /// - `StatementList` for the if-body
    /// - optional `CompilerElseIfClause` + `StatementList`
    /// - optional `CompilerElseClause` + `StatementList`
    /// - `CompilerEndIfClause` wrapping `#End If`
    ///
    /// The body `StatementList` uses stop conditions for `#ElseIf`, `#Else`,
    /// and `#End If`.  When those tokens appear inside a nested regular
    /// `IfStatement` frame, the iterative engine consumes them via its own
    /// stop logic (preserving existing behaviour).  In contexts where no
    /// regular `IfStatement` consumes them (e.g. module-level `#If`),
    /// the body stops at the directive token and this function handles the
    /// clause.
    ///
    /// Caller MUST have verified `at_compiler_directive_keyword(Token::IfKeyword)` first.
    pub(crate) fn parse_compiler_directive(&mut self) {
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::CompilerDirective.to_raw());

        // ---- CompilerIfClause ----
        self.builder
            .start_node(SyntaxKind::CompilerIfClause.to_raw());
        self.consume_token(); // Octothorpe
        self.consume_whitespace();
        self.consume_token(); // If
        self.consume_whitespace();
        self.parse_expression();
        self.consume_whitespace();
        if self.at_token(Token::ThenKeyword) {
            self.consume_token();
        }
        self.consume_whitespace();
        while self.at_token(Token::EndOfLineComment) || self.at_token(Token::RemComment) {
            self.consume_token();
            self.consume_whitespace();
        }
        if self.at_token(Token::Newline) {
            self.consume_token();
        }
        self.builder.finish_node(); // CompilerIfClause

        // ---- Body / ElseIf / Else clauses ----
        loop {
            // Parse body (stops at #ElseIf, #Else, #End If, or end-of-scope)
            self.parse_statement_list(|parser| {
                parser.at_compiler_directive_keyword(Token::ElseIfKeyword)
                    || parser.at_compiler_directive_keyword(Token::ElseKeyword)
                    || parser.at_compiler_end_if_directive()
            });

            // Check what stopped us and handle if it's a directive clause
            if self.at_compiler_directive_keyword(Token::ElseIfKeyword) {
                // CompilerElseIfClause
                self.builder
                    .start_node(SyntaxKind::CompilerElseIfClause.to_raw());
                self.consume_token(); // Octothorpe
                self.consume_whitespace();
                self.consume_token(); // ElseIf
                self.consume_whitespace();
                self.parse_expression();
                self.consume_whitespace();
                if self.at_token(Token::ThenKeyword) {
                    self.consume_token();
                }
                self.consume_whitespace();
                if self.at_token(Token::Newline) {
                    self.consume_token();
                }
                self.builder.finish_node(); // CompilerElseIfClause
                // Continue loop to parse elsif body
                continue;
            }

            if self.at_compiler_directive_keyword(Token::ElseKeyword) {
                // CompilerElseClause
                self.builder
                    .start_node(SyntaxKind::CompilerElseClause.to_raw());
                self.consume_token(); // Octothorpe
                self.consume_whitespace();
                self.consume_token(); // Else
                self.consume_whitespace();
                if self.at_token(Token::Newline) {
                    self.consume_token();
                }
                self.builder.finish_node(); // CompilerElseClause
                // Continue loop to parse else body
                continue;
            }

            if self.at_compiler_end_if_directive() {
                // CompilerEndIfClause
                self.builder
                    .start_node(SyntaxKind::CompilerEndIfClause.to_raw());
                self.consume_token(); // Octothorpe
                self.consume_whitespace();
                self.consume_token(); // End
                self.consume_whitespace();
                self.consume_token(); // If
                self.consume_until_after(Token::Newline);
                self.builder.finish_node(); // CompilerEndIfClause
            }

            break;
        }

        self.builder.finish_node(); // CompilerDirective
    }

    /// Parse a single-line If statement.
    ///
    /// Single-line If statements have the form:
    /// `If` condition `Then` statement [ `Else` statement ]
    /// The statement(s) can be any valid VB6 statement, including procedure calls,
    /// assignments, and even another single-line If.
    fn parse_single_line_if_statement(&mut self) {
        // Single-line If: parse inline statements.
        // Statement parsers (e.g. parse_assignment_statement) consume the
        // trailing newline. We must detect that and stop the loop, otherwise
        // the single-line If would keep parsing onto subsequent lines.
        while !self.is_at_end() && !self.at_token(Token::Newline) {
            if self.at_token(Token::ElseKeyword) {
                break;
            }

            let pos_before = self.pos;

            if self.is_control_flow_keyword() {
                self.parse_control_flow_statement();
            } else if self.is_library_statement_keyword() {
                self.parse_library_statement();
            } else if self.is_variable_declaration_keyword() {
                self.parse_array_statement();
            } else if self.is_statement_keyword() {
                self.parse_statement();
            } else {
                match self.current_token() {
                    Some(
                        Token::Whitespace
                        | Token::EndOfLineComment
                        | Token::RemComment
                        | Token::ColonOperator,
                    ) => {
                        self.consume_token();
                    }
                    _ => {
                        if self.at_token(Token::LetKeyword) {
                            self.parse_let_statement();
                        } else if self.at_token(Token::PeriodOperator) {
                            // Handle dot-prefixed member access in With blocks
                            if self.is_at_with_member_assignment() {
                                self.parse_assignment_statement();
                            } else {
                                self.parse_procedure_call();
                            }
                        } else if self.is_at_assignment() {
                            self.parse_assignment_statement();
                        } else {
                            self.consume_token();
                        }
                    }
                }
            }

            // If a statement parser consumed a newline as part of the
            // statement, we've reached the end of this single line.
            if self.pos > pos_before
                && self.tokens[pos_before..self.pos]
                    .iter()
                    .any(|(_, t)| *t == Token::Newline)
            {
                break;
            }
        }

        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.builder.finish_node();
        // IfStatement
    }
}
