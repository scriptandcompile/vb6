//! Expression evaluation over the CST.
//!
//! Expressions are evaluated directly against the tree produced by
//! `vb6parse`, producing [`VBVariant`]s from `vb6runtime`. Pure operator
//! semantics live in [`operators`], literal parsing in [`literals`], and
//! `Like` matching in [`like`].

mod like;
mod literals;
mod operators;

pub(crate) use literals::literal_value;
pub(crate) use operators::{arith, ArithmeticOperator};

use crate::builtins;
use crate::error::{RunError, RunResult};
use crate::interpreter::Interpreter;
use crate::scope::normalize;
use vb6core::error::{err_number, VBError, VBResult};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::VBVariant;

impl Interpreter {
    /// Evaluate an expression node to a value.
    pub(crate) fn eval_expr(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        match node.kind() {
            SyntaxKind::LiteralExpression
            | SyntaxKind::NumericLiteralExpression
            | SyntaxKind::StringLiteralExpression
            | SyntaxKind::BooleanLiteralExpression
            | SyntaxKind::IntegerLiteral
            | SyntaxKind::LongLiteral
            | SyntaxKind::SingleLiteral
            | SyntaxKind::DoubleLiteral
            | SyntaxKind::CurrencyLiteral
            | SyntaxKind::DecimalLiteral
            | SyntaxKind::StringLiteral
            | SyntaxKind::DateLiteral => self.eval_literal(node),
            SyntaxKind::IdentifierExpression => self.eval_identifier(node),
            SyntaxKind::BinaryExpression => {
                let parts: Vec<&CstNode> = node.significant_children().collect();
                if parts.len() != 3 {
                    return Err(self.error_at(node, VBError::invalid_procedure_call()));
                }
                self.eval_binary(parts[0], parts[1], parts[2])
            }
            SyntaxKind::UnaryExpression => {
                let parts: Vec<&CstNode> = node.significant_children().collect();
                if parts.len() != 2 {
                    return Err(self.error_at(node, VBError::invalid_procedure_call()));
                }
                self.eval_unary(parts[0], parts[1])
            }
            SyntaxKind::ParenthesizedExpression => {
                let parts: Vec<&CstNode> = node.significant_children().collect();
                // [LeftParenthesis, expr, RightParenthesis]
                if let Some(expr) = parts.into_iter().find(|part| {
                    !matches!(
                        part.kind(),
                        SyntaxKind::LeftParenthesis | SyntaxKind::RightParenthesis
                    )
                }) {
                    self.eval_expr(expr)
                } else {
                    Err(self.error_at(node, VBError::invalid_procedure_call()))
                }
            }
            SyntaxKind::CallExpression => self.eval_call(node),
            SyntaxKind::MemberAccessExpression => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "Object member access is not supported yet",
                ),
            )),
            SyntaxKind::TypeOfExpression => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "TypeOf requires object support, which is not implemented yet",
                ),
            )),
            SyntaxKind::NewExpression => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "New object creation is not implemented yet",
                ),
            )),
            SyntaxKind::AddressOfExpression => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "AddressOf is not implemented yet",
                ),
            )),
            other => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    format!("Unsupported expression node: {other:?}"),
                ),
            )),
        }
    }

    /// Evaluate a literal node (also usable for bare literal tokens, as found
    /// in `Case` clauses and `ReDim` bounds).
    pub(crate) fn eval_literal(&self, node: &CstNode) -> RunResult<VBVariant> {
        if let Some(token) = node.first_child() {
            return literal_value(token.text(), token.kind())
                .ok_or_else(|| self.error_at(node, VBError::type_mismatch()));
        }
        literal_value(node.text(), node.kind())
            .ok_or_else(|| self.error_at(node, VBError::type_mismatch()))
    }

    /// Evaluate an identifier reference.
    fn eval_identifier(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        let name = crate::program::identifier_name(node);

        match name.to_lowercase().as_str() {
            "true" => return Ok(VBVariant::Boolean(true)),
            "false" => return Ok(VBVariant::Boolean(false)),
            "nothing" => return Ok(VBVariant::Nothing),
            "null" => return Ok(VBVariant::Null),
            "empty" => return Ok(VBVariant::Empty),
            "me" => {
                return Err(self.error_at(
                    node,
                    VBError::with_description(
                        err_number::OBJECT_REQUIRED,
                        "'Me' is not available in a standard module",
                    ),
                ));
            }
            _ => {}
        }

        match self.lookup(&name) {
            Some(value) => Ok(value.clone()),
            // Undeclared references read as Empty (VB6 without Option Explicit).
            None => Ok(VBVariant::Empty),
        }
    }

    /// Evaluate a binary operation. `op` is the operator token node.
    fn eval_binary(
        &mut self,
        left: &CstNode,
        op: &CstNode,
        right: &CstNode,
    ) -> RunResult<VBVariant> {
        let lhs = self.eval_expr(left)?;
        let rhs = self.eval_expr(right)?;

        match op.kind() {
            SyntaxKind::AdditionOperator => {
                operators::add(lhs, rhs).map_err(|e| self.error_at(op, e))
            }
            SyntaxKind::SubtractionOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::Subtract)
                    .map_err(|e| self.error_at(op, e))
            }
            SyntaxKind::MultiplicationOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::Multiply)
                    .map_err(|e| self.error_at(op, e))
            }
            SyntaxKind::DivisionOperator => operators::arith(lhs, rhs, ArithmeticOperator::Divide)
                .map_err(|e| self.error_at(op, e)),
            SyntaxKind::BackwardSlashOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::IntegerDivide)
                    .map_err(|e| self.error_at(op, e))
            }
            SyntaxKind::ModKeyword => operators::arith(lhs, rhs, ArithmeticOperator::Modulus)
                .map_err(|e| self.error_at(op, e)),
            SyntaxKind::ExponentiationOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::Exponent)
                    .map_err(|e| self.error_at(op, e))
            }
            SyntaxKind::Ampersand => {
                let left = lhs.as_string()?;
                let right = rhs.as_string()?;
                Ok(VBVariant::from_string(format!("{left}{right}")))
            }
            SyntaxKind::EqualityOperator => Ok(VBVariant::Boolean(lhs == rhs)),
            SyntaxKind::InequalityOperator => Ok(VBVariant::Boolean(lhs != rhs)),
            // Ordered comparisons intentionally produce unpositioned errors
            // (as they always have): only arithmetic wraps position here.
            SyntaxKind::LessThanOperator => {
                operators::compare_ord(lhs, rhs, operators::Ordering::Less).map_err(RunError::new)
            }
            SyntaxKind::LessThanOrEqualOperator => {
                operators::compare_ord(lhs, rhs, operators::Ordering::LessOrEqual)
                    .map_err(RunError::new)
            }
            SyntaxKind::GreaterThanOperator => {
                operators::compare_ord(lhs, rhs, operators::Ordering::Greater)
                    .map_err(RunError::new)
            }
            SyntaxKind::GreaterThanOrEqualOperator => {
                operators::compare_ord(lhs, rhs, operators::Ordering::GreaterOrEqual)
                    .map_err(RunError::new)
            }
            SyntaxKind::AndKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::And)
                .map_err(|e| self.error_at(op, e)),
            SyntaxKind::OrKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Or)
                .map_err(|e| self.error_at(op, e)),
            SyntaxKind::XorKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Xor)
                .map_err(|e| self.error_at(op, e)),
            SyntaxKind::EqvKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Eqv)
                .map_err(|e| self.error_at(op, e)),
            SyntaxKind::ImpKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Imp)
                .map_err(|e| self.error_at(op, e)),
            SyntaxKind::IsKeyword => {
                let result = match (&lhs, &rhs) {
                    (VBVariant::Nothing, VBVariant::Nothing) => true,
                    (VBVariant::Nothing, _) | (_, VBVariant::Nothing) => false,
                    // No object model yet: fall back to value equality.
                    _ => lhs == rhs,
                };
                Ok(VBVariant::Boolean(result))
            }
            SyntaxKind::LikeKeyword => {
                let text = lhs.as_string()?;
                let pattern = rhs.as_string()?;
                Ok(VBVariant::Boolean(like::like_match(&pattern, &text)))
            }
            other => Err(self.error_at(
                op,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    format!("Unsupported binary operator: {other:?}"),
                ),
            )),
        }
    }

    /// Evaluate a unary operation. `op` is the operator token node.
    fn eval_unary(&mut self, op: &CstNode, operand: &CstNode) -> RunResult<VBVariant> {
        let value = self.eval_expr(operand)?;
        match op.kind() {
            SyntaxKind::SubtractionOperator => {
                let number = value.as_f64()?;
                Ok(VBVariant::from_double(-number))
            }
            SyntaxKind::AdditionOperator => {
                let number = value.as_f64()?;
                Ok(VBVariant::from_double(number))
            }
            SyntaxKind::NotKeyword => {
                let boolean = value.as_bool()?;
                Ok(VBVariant::Boolean(!boolean))
            }
            other => Err(self.error_at(
                op,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    format!("Unsupported unary operator: {other:?}"),
                ),
            )),
        }
    }

    /// Evaluate a call expression: array indexing, user function, or builtin.
    fn eval_call(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        let name = crate::program::identifier_name(node);

        let argument_list = node
            .significant_children()
            .find(|child| child.kind() == SyntaxKind::ArgumentList);
        let args = match argument_list {
            Some(list) => self.eval_args(list)?,
            None => Vec::new(),
        };

        // Array reference: the name resolves to an array variable. An empty
        // argument list (`Values()`) passes the whole array; any other
        // argument list is element indexing.
        if let Some(VBVariant::Array(_)) = self.lookup(&name) {
            let array = self.lookup(&name).ok_or_else(VBError::object_not_set)?;
            if args.is_empty() {
                return Ok(array.clone());
            }
            let indices: VBResult<Vec<i32>> = args.iter().map(|arg| arg.as_i32()).collect();
            let indices = indices?;
            if let VBVariant::Array(array) = array {
                let element = array.get(&indices)?;
                return Ok(element.clone());
            }
        }

        // User-defined function.
        let key = normalize(&name);
        if self.procedures.contains_key(&key) {
            return self.call_function(&name, args);
        }

        // Builtin function.
        builtins::call_builtin(&name, &args).map_err(|e| self.error_at(node, e))
    }

    /// Evaluate an `ArgumentList` into positional argument values.
    pub(crate) fn eval_args(&mut self, node: &CstNode) -> RunResult<Vec<VBVariant>> {
        let mut values = Vec::new();
        for argument in node.children_by_kind(SyntaxKind::Argument) {
            if let Some(expr) = argument
                .significant_children()
                .find(|child| !matches!(child.kind(), SyntaxKind::Comma))
            {
                values.push(self.eval_expr(expr)?);
            }
        }
        Ok(values)
    }

    /// Look up a variable in the current frame, then in globals.
    pub(crate) fn lookup(&self, name: &str) -> Option<&VBVariant> {
        let key = normalize(name);
        if let Some(frame) = self.frames.last() {
            if let Some(value) = frame.locals.get(&key) {
                return Some(value);
            }
        }
        self.globals.get(&key)
    }

    /// Attach source line information to a VB6 error.
    fn error_at(&self, _node: &CstNode, error: VBError) -> RunError {
        RunError::new(error)
            .at_line(self.current_stmt_line)
            .in_procedure(&self.current_procedure_name())
    }
}

// Re-exported for builtins that need currency scaling.
const _: i64 = vb6runtime::CURRENCY_SCALE;
