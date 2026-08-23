//! Assignment: `Let`/`Set` statements, writes into variables, array
//! elements, function return slots, and implicit declaration.

use vb6core::error::{VBError, VBResult};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::VBVariant;

use super::super::program;
use crate::error::RunResult;
use crate::interpreter::Interpreter;

impl Interpreter {
    /// Assignment, `Set`, or `Let`: `lhs = rhs`.
    pub(crate) fn exec_assignment(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        // LHS may be preceded by `Let`/`Set` keywords.
        let lhs = significant
            .iter()
            .find(|c| {
                matches!(
                    c.kind(),
                    SyntaxKind::IdentifierExpression
                        | SyntaxKind::CallExpression
                        | SyntaxKind::MemberAccessExpression
                )
            })
            .copied()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        // RHS is the last significant child (a `Set` LHS object may have `New`).
        let rhs = significant
            .last()
            .copied()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let value = self.eval_expr(rhs)?;
        self.assign(lhs, value)
    }

    /// Write a value into a variable, array element, or function result.
    pub(crate) fn assign(&mut self, lhs: &CstNode, value: VBVariant) -> RunResult<()> {
        match lhs.kind() {
            SyntaxKind::IdentifierExpression => {
                let name = program::identifier_name(lhs);
                self.assign_to_name(&name, value);
                Ok(())
            }
            SyntaxKind::CallExpression => {
                let significant: Vec<&CstNode> = lhs.significant_children().collect();
                let name = significant
                    .iter()
                    .find(|c| program::is_identifier_like(c))
                    .map(|c| c.text().trim().to_string())
                    .unwrap_or_default();
                let argument_list = significant
                    .iter()
                    .find(|c| c.kind() == SyntaxKind::ArgumentList);
                let args = match argument_list {
                    Some(list) => self.eval_args(list)?,
                    None => Vec::new(),
                };
                let indices: Vec<i32> = args
                    .iter()
                    .map(|arg| arg.as_i32())
                    .collect::<VBResult<_>>()?;

                let existing = self
                    .lookup(&name)
                    .cloned()
                    .ok_or_else(|| self.error_here(VBError::subscript_out_of_range()))?;
                if let VBVariant::Array(mut array) = existing {
                    array.set(&indices, value)?;
                    self.set_variable(&name, VBVariant::Array(array));
                    Ok(())
                } else {
                    Err(self.error_here(VBError::type_mismatch()))
                }
            }
            SyntaxKind::MemberAccessExpression => Err(self.unsupported(lhs, "member assignment")),
            _ => Err(self.error_here(VBError::invalid_procedure_call())),
        }
    }

    /// Store into a named variable, or into the enclosing function's return
    /// slot when the name matches the function itself.
    fn assign_to_name(&mut self, name: &str, value: VBVariant) {
        // Assigning to the Function's name sets its return value.
        if let Some(frame) = self.frames.last() {
            if frame.is_function && name.to_lowercase() == frame.name.to_lowercase() {
                if let Some(frame) = self.frames.last_mut() {
                    frame.return_value = Some(value);
                }
                return;
            }
        }
        self.set_variable(name, value);
    }

    /// `Set obj = expr`: object-reference assignment.
    ///
    /// The parser keeps `Set` statements as flat token runs (unlike `Let`,
    /// which builds real expression nodes), so both sides are interpreted
    /// from raw tokens here: the target is an identifier and the source is
    /// evaluated by [`Interpreter::eval_flat_expression`].
    pub(crate) fn exec_set_statement(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;

        // Target: the identifier between `Set` and `=`. Member targets
        // (`Set Form1.Picture = ...`) need object support.
        let target = &significant[1..eq_index];
        if target.len() != 1 || !program::is_identifier_like(target[0]) {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }
        let name = target[0].text().trim().to_string();

        let value = self.eval_flat_expression(&significant[eq_index + 1..])?;
        self.assign_to_name(&name, value);
        Ok(())
    }

    /// Declare a variable in the current scope (globals at module level).
    pub(crate) fn declare_in(&mut self, name: &str, value: VBVariant) {
        if self.frames.is_empty() {
            self.globals.declare(name, value);
        } else if let Some(frame) = self.frames.last_mut() {
            frame.locals.declare(name, value);
        }
    }

    /// Set a variable, implicit-declaring it in the current scope if needed.
    pub(crate) fn set_variable(&mut self, name: &str, value: VBVariant) {
        if !self.frames.is_empty() {
            if let Some(frame) = self.frames.last_mut() {
                if frame.locals.set(name, value.clone()) {
                    return;
                }
            }
        }
        if self.globals.set(name, value.clone()) {
            return;
        }
        self.declare_in(name, value);
    }
}
