//! Statement execution over the CST.
//!
//! Statements are dispatched by [`SyntaxKind`] and executed directly against
//! the tree. Line numbers are tracked by counting newlines: each block walks
//! its raw children, so nested bodies receive accurate start lines without
//! accumulating loop iterations.
//!
//! Submodules hold the statement families and the flat token evaluator;
//! this file keeps the statement dispatch loop and the shared error
//! helpers.

mod assignment;
mod call;
mod control_flow;
mod declarations;
mod file_io;
mod flat;
mod print;
mod statements;
mod util;

pub(crate) use util::{coerce, count_newlines};

use vb6core::error::{err_number, VBError};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;

use crate::error::{RunError, RunResult};
use crate::interpreter::{Flow, Interpreter};
use crate::program::is_statement_kind;

impl Interpreter {
    /// Execute every statement in a block (a `StatementList` or the module
    /// root). `start_line` is the 1-based line the block's first statement is
    /// on.
    pub(crate) fn exec_statements(
        &mut self,
        parent: &CstNode,
        start_line: usize,
    ) -> RunResult<Flow> {
        let mut line = start_line;
        for child in parent.children() {
            match child.kind() {
                SyntaxKind::Newline => line += 1,
                SyntaxKind::Whitespace | SyntaxKind::EndOfLineComment | SyntaxKind::RemComment => {}
                SyntaxKind::LabelStatement => {
                    // Labels matter only for GoTo/GoSub, which are unsupported.
                }
                kind if is_statement_kind(kind) => {
                    self.current_stmt_line = line;
                    // Loops emit their own element-level trace snapshots, so
                    // skip the generic whole-line snapshot to avoid a
                    // duplicate highlight of the loop line at entry.
                    if self.record_debug_snapshots
                        && matches!(
                            kind,
                            SyntaxKind::ForStatement
                                | SyntaxKind::DoStatement
                                | SyntaxKind::WhileStatement
                        )
                    {
                        self.step_without_snapshot()?;
                    } else {
                        self.step()?;
                    }
                    let flow = self.exec_stmt(child, line)?;
                    if flow != Flow::Next {
                        return Ok(flow);
                    }
                    line += count_newlines(child);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// Execute a single statement.
    pub(crate) fn exec_stmt(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        match node.kind() {
            SyntaxKind::AssignmentStatement => {
                self.exec_assignment(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::LetStatement => {
                self.exec_assignment(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::SetStatement => {
                self.exec_set_statement(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::DimStatement | SyntaxKind::ConstStatement => {
                self.exec_dim(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::ReDimStatement => {
                self.exec_redim(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::IfStatement => self.exec_if(node, line),
            SyntaxKind::ForStatement => self.exec_for(node, line),
            SyntaxKind::ForEachStatement => Err(self.unsupported(node, "For Each")),
            SyntaxKind::DoStatement => self.exec_do(node, line),
            SyntaxKind::WhileStatement => self.exec_while(node, line),
            SyntaxKind::SelectCaseStatement => self.exec_select(node, line),
            SyntaxKind::CallStatement => self.exec_call(node),
            SyntaxKind::PrintStatement => {
                self.print_node(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::OpenStatement => {
                self.exec_open(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::CloseStatement => {
                self.exec_close(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::ExitStatement => self.exec_exit(node),
            SyntaxKind::EndStatement => {
                self.terminated = true;
                Ok(Flow::Terminate)
            }
            SyntaxKind::StopStatement => {
                vb6runtime::library::interaction::stop::stop();
                if self.record_debug_snapshots {
                    // Development environment: suspend execution (break
                    // mode) without closing files or clearing variables.
                    return Err(RunError::debug_pause()
                        .at_line(line)
                        .in_procedure(&self.current_procedure_name()));
                }
                // Compiled executable: `Stop` acts like `End`.
                self.terminated = true;
                Ok(Flow::Terminate)
            }
            SyntaxKind::BeepStatement => {
                vb6runtime::library::interaction::beep::beep();
                Ok(Flow::Next)
            }
            SyntaxKind::AppActivateStatement => {
                self.exec_app_activate(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::SendKeysStatement => {
                self.exec_send_keys(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::SavePictureStatement => {
                self.exec_save_picture(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::OptionStatement
            | SyntaxKind::TypeStatement
            | SyntaxKind::EnumStatement
            | SyntaxKind::DeclareStatement => Ok(Flow::Next),
            SyntaxKind::EraseStatement => {
                self.exec_erase(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::DateStatement => {
                self.exec_date_statement(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::TimeStatement => {
                self.exec_time_statement(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::LSetStatement => {
                self.exec_alignment_set(
                    node,
                    vb6runtime::library::string::lset_statement::lset_statement,
                )?;
                Ok(Flow::Next)
            }
            SyntaxKind::RSetStatement => {
                self.exec_alignment_set(
                    node,
                    vb6runtime::library::string::rset_statement::rset_statement,
                )?;
                Ok(Flow::Next)
            }
            SyntaxKind::MidStatement => {
                self.exec_mid_set(
                    node,
                    vb6runtime::library::string::mid_statement::mid_statement,
                )?;
                Ok(Flow::Next)
            }
            SyntaxKind::MidBStatement => {
                self.exec_mid_set(
                    node,
                    vb6runtime::library::string::midb_statement::midb_statement,
                )?;
                Ok(Flow::Next)
            }
            SyntaxKind::OnErrorStatement
            | SyntaxKind::GoSubStatement
            | SyntaxKind::GotoStatement
            | SyntaxKind::ReturnStatement
            | SyntaxKind::ResumeStatement
            | SyntaxKind::OnGoToStatement
            | SyntaxKind::OnGoSubStatement => Err(self.unsupported(node, "control-flow statement")),
            other => Err(self.unsupported(node, &format!("statement {other:?}"))),
        }
    }

    /// Unsupported-construct error.
    pub(crate) fn unsupported(&self, _node: &CstNode, what: &str) -> RunError {
        self.error_here(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            format!("{what} is not supported yet"),
        ))
    }

    /// Build an error tagged with the current source location.
    pub(crate) fn error_here(&self, error: VBError) -> RunError {
        RunError::new(error)
            .at_line(self.current_stmt_line)
            .in_procedure(&self.current_procedure_name())
    }
}
