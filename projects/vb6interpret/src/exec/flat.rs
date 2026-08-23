//! The "flat" evaluator: expression evaluation over raw token runs.
//!
//! Most statements arrive from the parser with real expression nodes, which
//! [`Interpreter::eval_expr`] walks. A handful — assignment right-hand
//! sides, `Date =` / `Time =`, `Mid=` / `MidB=`, `SavePicture`,
//! `AppActivate`, `SendKeys`, and `Open` / `Close` — instead carry an
//! unstructured run of tokens, because the parser does not build expression
//! subtrees for them yet. Until the parser changes, those operands cannot go
//! through `eval_expr` directly; this module is the token-level fallback.
//!
//! The family, from widest to narrowest input:
//!
//! - [`Interpreter::eval_flat_operand`] evaluates a single operand node:
//!   identifier-like tokens resolve as variables (`Empty` when undeclared,
//!   literal keywords such as `True` included), anything else defers to
//!   `eval_expr`.
//! - [`Interpreter::eval_simple_operand`] is the stricter variant used where
//!   a statement's grammar guarantees a single atom or expression node.
//! - [`Interpreter::eval_flat_expression`] evaluates a whole token run:
//!   a single atom, a direct call `Name(arg, ...)` dispatched user-function
//!   first then builtin, or an error (`New` unsupported).
//! - [`Interpreter::eval_flat_arguments`] splits a run on top-level commas.
//! - [`Interpreter::eval_flat_atom`] evaluates one bare token.
//! - [`Interpreter::eval_flat_token`] is the minimal variant used by
//!   `Open` / `Close`: bare identifier-or-literal only.
//!
//! These functions deliberately duplicate a slice of operator semantics
//! instead of being unified with `crate::eval`; when the parser eventually
//! emits expression nodes for these statements they should be deleted
//! outright rather than grown.

use vb6core::error::{err_number, VBError};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::VBVariant;

use crate::error::RunResult;
use crate::interpreter::Interpreter;
use crate::program::is_identifier_like;

impl Interpreter {
    /// Evaluate one operand of a flat-token statement: an identifier-like
    /// token names a variable (`Empty` when undeclared, literal keywords
    /// such as `True` included), anything else evaluates as an expression
    /// node.
    pub(crate) fn eval_flat_operand(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        if is_identifier_like(node) {
            let name = node.text().trim();
            if let Some(value) = self.lookup(name) {
                return Ok(value.clone());
            }
            return match node.kind() {
                SyntaxKind::TrueKeyword => Ok(VBVariant::Boolean(true)),
                SyntaxKind::FalseKeyword => Ok(VBVariant::Boolean(false)),
                SyntaxKind::NullKeyword => Ok(VBVariant::Null),
                SyntaxKind::NothingKeyword => Ok(VBVariant::Nothing),
                _ => Ok(VBVariant::Empty),
            };
        }
        self.eval_expr(node)
    }

    /// Evaluate a flat token run (a statement parsed without expression
    /// nodes). Handles single atoms, `New` (unsupported), and direct calls
    /// `Name(arg, ...)` whose arguments are themselves flat expressions.
    pub(crate) fn eval_flat_expression(&mut self, tokens: &[&CstNode]) -> RunResult<VBVariant> {
        let Some((first, rest)) = tokens.split_first() else {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        };
        if rest.is_empty() {
            return self.eval_flat_atom(first);
        }
        match first.kind() {
            SyntaxKind::Identifier => {
                if rest[0].kind() != SyntaxKind::LeftParenthesis
                    || rest
                        .last()
                        .is_none_or(|t| t.kind() != SyntaxKind::RightParenthesis)
                {
                    return Err(self.error_here(VBError::invalid_procedure_call()));
                }
                let inner = &rest[1..rest.len() - 1];
                let args = self.eval_flat_arguments(inner)?;
                let name = first.text().trim();
                // Same dispatch as `eval_call`: user function first, then
                // builtins (which raise error 35 for unknown names).
                if self.procedures.contains_key(&crate::scope::normalize(name)) {
                    return self.call_function(name, args);
                }
                crate::builtins::call_builtin(name, &args).map_err(|e| self.error_here(e))
            }
            SyntaxKind::NewKeyword => Err(self.error_here(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                "New object creation is not implemented yet",
            ))),
            _ => Err(self.error_here(VBError::invalid_procedure_call())),
        }
    }

    /// Split a flat token run on top-level commas and evaluate each part.
    fn eval_flat_arguments(&mut self, tokens: &[&CstNode]) -> RunResult<Vec<VBVariant>> {
        let mut parts: Vec<Vec<&CstNode>> = Vec::new();
        let mut depth = 0usize;
        for token in tokens {
            match token.kind() {
                SyntaxKind::LeftParenthesis => depth += 1,
                SyntaxKind::RightParenthesis => depth = depth.saturating_sub(1),
                SyntaxKind::Comma if depth == 0 => {
                    parts.push(Vec::new());
                    continue;
                }
                _ => {}
            }
            match parts.last_mut() {
                Some(part) => part.push(token),
                None => parts.push(vec![token]),
            }
        }
        parts
            .into_iter()
            .map(|part| self.eval_flat_expression(&part))
            .collect()
    }

    /// Evaluate one operand of a simple builtin statement. These statements
    /// keep raw tokens, so a bare identifier must be resolved directly
    /// instead of through `eval_expr`.
    pub(crate) fn eval_simple_operand(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        if node.kind() == SyntaxKind::Identifier {
            self.eval_flat_atom(node)
        } else {
            self.eval_expr(node)
        }
    }

    /// Evaluate a single-token flat expression: literals, the special
    /// keywords, or a variable reference.
    fn eval_flat_atom(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        match node.kind() {
            SyntaxKind::NothingKeyword => Ok(VBVariant::Nothing),
            SyntaxKind::NullKeyword => Ok(VBVariant::Null),
            SyntaxKind::EmptyKeyword => Ok(VBVariant::Empty),
            SyntaxKind::Identifier => {
                let name = node.text().trim();
                match self.lookup(name) {
                    Some(value) => Ok(value.clone()),
                    None => Ok(VBVariant::Empty),
                }
            }
            SyntaxKind::StringLiteral
            | SyntaxKind::IntegerLiteral
            | SyntaxKind::LongLiteral
            | SyntaxKind::SingleLiteral
            | SyntaxKind::DoubleLiteral
            | SyntaxKind::CurrencyLiteral
            | SyntaxKind::DateLiteral
            | SyntaxKind::TrueKeyword
            | SyntaxKind::FalseKeyword => self.eval_literal(node),
            _ => Err(self.error_here(VBError::invalid_procedure_call())),
        }
    }

    /// Evaluate a bare literal or `Identifier` token, as found in the flat
    /// token stream of `Open`/`Close` (which aren't parsed into nested
    /// expression nodes like other statements).
    pub(crate) fn eval_flat_token(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        if node.kind() == SyntaxKind::Identifier {
            let name = node.text().trim().to_string();
            return Ok(self.lookup(&name).cloned().unwrap_or(VBVariant::Empty));
        }
        self.eval_literal(node)
    }
}
