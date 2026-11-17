//! Statement execution over the CST.
//!
//! Statements are dispatched by [`SyntaxKind`] and executed directly against
//! the tree. Line numbers are tracked by counting newlines: each block walks
//! its raw children, so nested bodies receive accurate start lines without
//! accumulating loop iterations.
//!
//! Submodules hold the statement families; this file keeps the statement
//! dispatch loop and the shared error helpers.

mod assignment;
mod call;
mod control_flow;
mod declarations;
mod file_io;
mod print;
mod statements;
mod util;

pub(crate) use util::{coerce, count_newlines};

use vb6core::error::{VBError, err_number};
use vb6core::types::VBType;
use vb6parse::parsers::SyntaxKind;
use vb6parse::parsers::cst::CstNode;
use vb6runtime::VBVariant;

use crate::error::{BuiltinCallInfo, RunError, RunResult};
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
            SyntaxKind::ContinueStatement => self.exec_continue(node),
            SyntaxKind::ReturnStatement => self.exec_return(node),
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
            | SyntaxKind::ResumeStatement
            | SyntaxKind::OnGoToStatement
            | SyntaxKind::OnGoSubStatement => Err(self.unsupported(node, "control-flow statement")),
            other => Err(self.unsupported(node, &format!("statement {other:?}"))),
        }
    }

    fn exec_continue(&mut self, node: &CstNode) -> RunResult<Flow> {
        let target = if node
            .significant_children()
            .any(|child| child.kind() == SyntaxKind::ForKeyword)
        {
            crate::interpreter::LoopKind::For
        } else {
            crate::interpreter::LoopKind::Do
        };
        let depth = self.frames.len();
        if !self
            .active_loops
            .iter()
            .any(|(kind, loop_depth)| *kind == target && *loop_depth == depth)
        {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        }
        Ok(match target {
            crate::interpreter::LoopKind::For => Flow::ContinueFor,
            crate::interpreter::LoopKind::Do => Flow::ContinueDo,
        })
    }

    fn exec_return(&mut self, node: &CstNode) -> RunResult<Flow> {
        let value = node
            .significant_children()
            .find(|child| child.kind() != SyntaxKind::ReturnKeyword)
            .map(|expression| self.eval_expr(expression))
            .transpose()?;
        let Some(value) = value else {
            return Err(self.unsupported(node, "legacy GoSub/Return"));
        };
        let Some(frame) = self.frames.last().filter(|frame| frame.is_function) else {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        };
        let return_type = frame.return_type.clone();
        let value = Self::coerce_return_value(value, &return_type)
            .map_err(|error| self.error_here(error, None))?;
        self.frames
            .last_mut()
            .expect("function frame was checked above")
            .return_value = Some(value);
        Ok(Flow::Return)
    }

    fn coerce_return_value(value: VBVariant, ty: &VBType) -> Result<VBVariant, VBError> {
        match ty {
            VBType::Variant | VBType::Unknown => Ok(value),
            VBType::Byte => value.as_byte().map(VBVariant::Byte),
            VBType::Integer => value.as_i16().map(VBVariant::Integer),
            VBType::Long | VBType::Enum(_) => value.as_i32().map(VBVariant::Long),
            VBType::Single => value.as_f32().map(VBVariant::Single),
            VBType::Double => value.as_f64().map(VBVariant::Double),
            VBType::Currency => value.as_currency_scaled().map(VBVariant::Currency),
            VBType::Decimal => value.as_vbdecimal().map(VBVariant::Decimal),
            VBType::String => value.as_string().map(VBVariant::from_string),
            VBType::Boolean => value.as_bool().map(VBVariant::Boolean),
            VBType::Date => value.as_date_serial().map(VBVariant::Date),
            VBType::Object | VBType::Class(_) if value.is_object() || value.is_nothing() => {
                Ok(value)
            }
            VBType::Nothing if value.is_nothing() => Ok(value),
            VBType::Array(_) if matches!(&value, VBVariant::Array(_)) => Ok(value),
            VBType::Error if matches!(&value, VBVariant::Error(_)) => Ok(value),
            _ => Err(VBError::type_mismatch()),
        }
    }

    /// Unsupported-construct error.
    pub(crate) fn unsupported(&self, _node: &CstNode, what: &str) -> RunError {
        self.error_here(
            VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                format!("{what} is not supported yet"),
            ),
            None,
        )
    }

    /// Build an error tagged with the current source location.
    pub(crate) fn error_here(
        &self,
        error: VBError,
        call_info: Option<BuiltinCallInfo>,
    ) -> RunError {
        RunError::new(error)
            .at_line(self.current_stmt_line)
            .in_procedure(&self.current_procedure_name())
            .with_builtin_call(call_info)
    }
}
