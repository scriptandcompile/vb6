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
pub(crate) use operators::{ArithmeticOperator, arith};

use crate::builtins;
use crate::error::{BuiltinCallInfo, RunError, RunResult};
use crate::interpreter::Interpreter;
use crate::scope::normalize;
use vb6core::error::{VBError, VBResult, err_number};
use vb6core::types::VBType;
use vb6parse::parsers::SyntaxKind;
use vb6parse::parsers::cst::CstNode;
use vb6runtime::{ArrayDimension, ArrayValue, VBVariant};

#[derive(Debug, PartialEq)]
enum InterpolatedPart {
    Text(String),
    Expression(String),
}

impl Interpreter {
    fn eval_member_access(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        let parts: Vec<&CstNode> = node.significant_children().collect();
        let Some(receiver) = parts.first().copied() else {
            return Err(self.error_at(node, VBError::invalid_procedure_call(), None));
        };
        let optional = parts
            .iter()
            .any(|part| part.kind() == SyntaxKind::OptionalMemberAccessOperator);
        let value = self.eval_expr(receiver)?;

        if optional && value == VBVariant::Nothing {
            return Ok(VBVariant::Nothing);
        }
        if optional && value == VBVariant::Null {
            return Err(self.error_at(
                node,
                VBError::with_description(err_number::INVALID_USE_OF_NULL, "Invalid use of Null"),
                None,
            ));
        }
        Err(self.error_at(
            node,
            VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                "Object member access is not supported yet",
            ),
            None,
        ))
    }

    /// Evaluate an expression node to a value.
    pub(crate) fn eval_expr(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        match node.kind() {
            SyntaxKind::LiteralExpression
            | SyntaxKind::NumericLiteralExpression
            | SyntaxKind::BooleanLiteralExpression
            | SyntaxKind::IntegerLiteral
            | SyntaxKind::LongLiteral
            | SyntaxKind::SingleLiteral
            | SyntaxKind::DoubleLiteral
            | SyntaxKind::CurrencyLiteral
            | SyntaxKind::DecimalLiteral
            | SyntaxKind::StringLiteral
            | SyntaxKind::DateLiteral => self.eval_literal(node),
            SyntaxKind::NothingKeyword | SyntaxKind::NullKeyword | SyntaxKind::EmptyKeyword => {
                self.eval_literal(node)
            }
            SyntaxKind::StringLiteralExpression => {
                if node
                    .first_child()
                    .is_some_and(|child| child.kind() == SyntaxKind::InterpolatedStringLiteral)
                {
                    self.eval_interpolated_string(node)
                } else {
                    self.eval_literal(node)
                }
            }
            SyntaxKind::ArrayLiteralExpression => self.eval_array_literal(node),
            SyntaxKind::IdentifierExpression => self.eval_identifier(node),
            SyntaxKind::BinaryExpression => {
                let parts: Vec<&CstNode> = node.significant_children().collect();
                if parts.len() != 3 {
                    return Err(self.error_at(node, VBError::invalid_procedure_call(), None));
                }
                self.eval_binary(parts[0], parts[1], parts[2])
            }
            SyntaxKind::UnaryExpression => {
                let parts: Vec<&CstNode> = node.significant_children().collect();
                if parts.len() != 2 {
                    return Err(self.error_at(node, VBError::invalid_procedure_call(), None));
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
                    Err(self.error_at(node, VBError::invalid_procedure_call(), None))
                }
            }
            SyntaxKind::CallExpression => self.eval_call(node),
            SyntaxKind::MemberAccessExpression => self.eval_member_access(node),
            SyntaxKind::TypeOfExpression => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "TypeOf requires object support, which is not implemented yet",
                ),
                None,
            )),
            SyntaxKind::NewExpression => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "New object creation is not implemented yet",
                ),
                None,
            )),
            SyntaxKind::AddressOfExpression => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "AddressOf is not implemented yet",
                ),
                None,
            )),
            other => Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    format!("Unsupported expression node: {other:?}"),
                ),
                None,
            )),
        }
    }

    fn eval_array_literal(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        let elements: Vec<&CstNode> = node
            .significant_children()
            .filter(|child| {
                !matches!(
                    child.kind(),
                    SyntaxKind::LeftSquareBracket
                        | SyntaxKind::RightSquareBracket
                        | SyntaxKind::Comma
                )
            })
            .collect();
        if elements.is_empty() {
            return Err(self.error_at(
                node,
                VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "Empty array literal requires a typed context, which is not available",
                ),
                None,
            ));
        }

        let mut values = Vec::with_capacity(elements.len());
        for element in elements {
            values.push(self.eval_expr(element)?);
        }
        let element_type = common_array_element_type(&values)
            .ok_or_else(|| self.error_at(node, VBError::type_mismatch(), None))?;
        let upper = i32::try_from(values.len() - 1)
            .map_err(|_| self.error_at(node, VBError::out_of_memory(), None))?;
        let mut array = ArrayValue::new_fixed(element_type, &[ArrayDimension::new(0, upper)])
            .map_err(|error| self.error_at(node, error, None))?;
        for (index, value) in values.into_iter().enumerate() {
            array
                .set(&[index as i32], value)
                .map_err(|error| self.error_at(node, error, None))?;
        }
        Ok(VBVariant::Array(array))
    }

    /// Evaluate a literal node (also usable for bare literal tokens, as found
    /// in `Case` clauses and `ReDim` bounds).
    pub(crate) fn eval_literal(&self, node: &CstNode) -> RunResult<VBVariant> {
        if let Some(token) = node.first_child() {
            return literal_value(token.text(), token.kind())
                .ok_or_else(|| self.error_at(node, VBError::type_mismatch(), None));
        }

        literal_value(node.text(), node.kind())
            .ok_or_else(|| self.error_at(node, VBError::type_mismatch(), None))
    }

    fn eval_interpolated_string(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        let raw = node
            .first_child()
            .map(|child| child.text())
            .ok_or_else(|| self.error_at(node, invalid_interpolation("missing token"), None))?;
        let parts = Self::split_interpolated_string(raw).ok_or_else(|| {
            self.error_at(node, invalid_interpolation("malformed delimiters"), None)
        })?;
        let mut result = String::new();
        for part in parts {
            match part {
                InterpolatedPart::Text(text) => result.push_str(&text),
                InterpolatedPart::Expression(expression) => {
                    if expression.trim().is_empty() {
                        return Err(self.error_at(
                            node,
                            invalid_interpolation("empty embedded expression"),
                            None,
                        ));
                    }
                    let value = self.eval_interpolated_expression(&expression, node)?;
                    result.push_str(
                        &value
                            .as_string()
                            .map_err(|error| self.error_at(node, error, None))?,
                    );
                }
            }
        }
        Ok(VBVariant::from_string(result))
    }

    fn eval_interpolated_expression(
        &mut self,
        expression: &str,
        node: &CstNode,
    ) -> RunResult<VBVariant> {
        let source = format!(
            "Attribute VB_Name = \"MxInterpolation\"\nOption VB6RExtensions On\nSub MxInterpolation()\nMxValue = ({expression})\nEnd Sub\n"
        );
        let source_file = vb6parse::io::SourceFile::from_string("mx-interpolation.bas", &source);
        let module = vb6parse::files::ModuleFile::parse(&source_file)
            .ok_or_errors()
            .map_err(|_| {
                self.error_at(
                    node,
                    invalid_interpolation("embedded expression did not parse"),
                    None,
                )
            })?;
        let root = module.cst.to_root_node();
        fn find_assignment(node: &CstNode) -> Option<CstNode> {
            if node.kind() == SyntaxKind::AssignmentStatement {
                return Some(node.clone());
            }
            node.children().iter().find_map(find_assignment)
        }
        let assignment = find_assignment(&root).ok_or_else(|| {
            self.error_at(
                node,
                invalid_interpolation("embedded expression was not found"),
                None,
            )
        })?;
        let expression = assignment
            .significant_children()
            .last()
            .cloned()
            .ok_or_else(|| {
                self.error_at(
                    node,
                    invalid_interpolation("embedded expression is empty"),
                    None,
                )
            })?;
        self.eval_expr(&expression)
    }

    fn split_interpolated_string(raw: &str) -> Option<Vec<InterpolatedPart>> {
        let inner = raw.strip_prefix("$\"")?.strip_suffix('"')?;
        let chars: Vec<char> = inner.chars().collect();
        let mut parts = Vec::new();
        let mut text = String::new();
        let mut index = 0;
        while index < chars.len() {
            match chars[index] {
                '"' if chars.get(index + 1) == Some(&'"') => {
                    text.push('"');
                    index += 2;
                }

                '{' if chars.get(index + 1) == Some(&'{') => {
                    text.push('{');
                    index += 2;
                }
                '}' if chars.get(index + 1) == Some(&'}') => {
                    text.push('}');
                    index += 2;
                }
                '{' => {
                    if !text.is_empty() {
                        parts.push(InterpolatedPart::Text(std::mem::take(&mut text)));
                    }
                    index += 1;
                    let start = index;
                    let mut quoted = false;
                    while index < chars.len() {
                        match chars[index] {
                            '"' if quoted && chars.get(index + 1) == Some(&'"') => index += 2,
                            '"' => {
                                quoted = !quoted;
                                index += 1;
                            }
                            '}' if !quoted => break,
                            _ => index += 1,
                        }
                    }
                    if index == chars.len() {
                        return None;
                    }
                    parts.push(InterpolatedPart::Expression(
                        chars[start..index].iter().collect(),
                    ));
                    index += 1;
                }
                '}' => return None,
                character => {
                    text.push(character);
                    index += 1;
                }
            }
        }
        if !text.is_empty() {
            parts.push(InterpolatedPart::Text(text));
        }
        Some(parts)
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
                    None,
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
        let operator_text = op.text();
        if op.kind() == SyntaxKind::NullCoalescingOperator {
            return if lhs == VBVariant::Nothing {
                self.eval_expr(right)
            } else {
                Ok(lhs)
            };
        }
        if operator_text.eq_ignore_ascii_case("AndAlso") {
            let left_value = lhs.as_bool()?;
            if !left_value {
                return Ok(VBVariant::Boolean(false));
            }
            return Ok(VBVariant::Boolean(self.eval_expr(right)?.as_bool()?));
        }
        if operator_text.eq_ignore_ascii_case("OrElse") {
            let left_value = lhs.as_bool()?;
            if left_value {
                return Ok(VBVariant::Boolean(true));
            }
            return Ok(VBVariant::Boolean(self.eval_expr(right)?.as_bool()?));
        }
        let rhs = self.eval_expr(right)?;

        match op.kind() {
            SyntaxKind::AdditionOperator => {
                operators::add(lhs, rhs).map_err(|e| self.error_at(op, e, None))
            }
            SyntaxKind::SubtractionOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::Subtract)
                    .map_err(|e| self.error_at(op, e, None))
            }
            SyntaxKind::MultiplicationOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::Multiply)
                    .map_err(|e| self.error_at(op, e, None))
            }
            SyntaxKind::DivisionOperator => operators::arith(lhs, rhs, ArithmeticOperator::Divide)
                .map_err(|e| self.error_at(op, e, None)),
            SyntaxKind::BackwardSlashOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::IntegerDivide)
                    .map_err(|e| self.error_at(op, e, None))
            }
            SyntaxKind::ModKeyword => operators::arith(lhs, rhs, ArithmeticOperator::Modulus)
                .map_err(|e| self.error_at(op, e, None)),
            SyntaxKind::ExponentiationOperator => {
                operators::arith(lhs, rhs, ArithmeticOperator::Exponent)
                    .map_err(|e| self.error_at(op, e, None))
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
                .map_err(|e| self.error_at(op, e, None)),
            SyntaxKind::OrKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Or)
                .map_err(|e| self.error_at(op, e, None)),
            SyntaxKind::XorKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Xor)
                .map_err(|e| self.error_at(op, e, None)),
            SyntaxKind::EqvKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Eqv)
                .map_err(|e| self.error_at(op, e, None)),
            SyntaxKind::ImpKeyword => operators::bitwise(lhs, rhs, operators::LogicalOperator::Imp)
                .map_err(|e| self.error_at(op, e, None)),
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
                None,
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
                None,
            )),
        }
    }

    /// Evaluate a call expression: array indexing, user function, or builtin.
    fn eval_call(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        let name = crate::program::identifier_name(node);

        let argument_list = node
            .significant_children()
            .find(|child| child.kind() == SyntaxKind::ArgumentList);
        let (args, spread_values) = match argument_list {
            Some(list) => self.eval_args_with_spread(list)?,
            None => (Vec::new(), Vec::new()),
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
            if let Some(procedure) = self.procedures.get(&key)
                && procedure
                    .params
                    .iter()
                    .zip(spread_values.iter())
                    .any(|(param, spread)| *spread && param.by_ref)
            {
                return Err(self.error_at(
                    node,
                    VBError::with_description(
                        err_number::INVALID_PROCEDURE_CALL,
                        "Spread arguments cannot bind to ByRef parameters",
                    ),
                    None,
                ));
            }
            return self.call_function(&name, args);
        }

        // Builtin function.
        builtins::call_builtin(&name, &args).map_err(|e| {
            let param_info = e
                .param_index
                .map(|idx| (idx, e.param_name.clone().unwrap_or_default()));
            let arg_byte_ranges = argument_list.map(|list| {
                list.children_by_kind(SyntaxKind::Argument)
                    .map(|arg| arg.byte_range())
                    .collect()
            });
            let call_info = param_info.map(|(param_index, param_name)| BuiltinCallInfo {
                param_index,
                param_name,
                arg_byte_ranges,
            });
            self.error_at(node, e, call_info)
        })
    }

    /// Evaluate an `ArgumentList` into positional argument values.
    pub(crate) fn eval_args(&mut self, node: &CstNode) -> RunResult<Vec<VBVariant>> {
        self.eval_args_with_spread(node).map(|(values, _)| values)
    }

    pub(crate) fn eval_spread_expression(&mut self, expr: &CstNode) -> RunResult<Vec<VBVariant>> {
        let source = expr
            .significant_children()
            .find(|child| child.kind() != SyntaxKind::PeriodOperator)
            .ok_or_else(|| self.error_at(expr, VBError::type_mismatch(), None))?;
        let value = self.eval_expr(source)?;
        let VBVariant::Array(array) = value else {
            return Err(self.error_at(
                expr,
                VBError::with_description(
                    err_number::TYPE_MISMATCH,
                    "Spread source must be an initialized rank-1 array",
                ),
                None,
            ));
        };
        if array.rank() != 1 || !array.is_initialized() {
            return Err(self.error_at(
                expr,
                VBError::with_description(
                    err_number::TYPE_MISMATCH,
                    "Spread source must be an initialized rank-1 array",
                ),
                None,
            ));
        }
        let dimension = &array.dimensions()[0];
        let mut values = Vec::with_capacity(array.len());
        for index in dimension.lower..=dimension.upper {
            values.push(array.get(&[index])?.clone());
        }
        Ok(values)
    }

    pub(crate) fn eval_args_with_spread(
        &mut self,
        node: &CstNode,
    ) -> RunResult<(Vec<VBVariant>, Vec<bool>)> {
        let mut values = Vec::new();
        let mut spread_values = Vec::new();
        for argument in node.children_by_kind(SyntaxKind::Argument) {
            if let Some(expr) = argument
                .significant_children()
                .find(|child| !matches!(child.kind(), SyntaxKind::Comma))
            {
                if expr.kind() == SyntaxKind::SpreadExpression {
                    for value in self.eval_spread_expression(expr)? {
                        values.push(value);
                        spread_values.push(true);
                    }
                } else {
                    values.push(self.eval_expr(expr)?);
                    spread_values.push(false);
                }
            }
        }
        Ok((values, spread_values))
    }

    /// Look up a variable in the current frame, then in globals.
    pub(crate) fn lookup(&self, name: &str) -> Option<&VBVariant> {
        let key = normalize(name);
        if let Some(frame) = self.frames.last()
            && let Some(value) = frame.locals.get(&key)
        {
            return Some(value);
        }
        self.globals.get(&key)
    }

    /// Attach source line information to a VB6 error.
    fn error_at(
        &self,
        _node: &CstNode,
        error: VBError,
        call_info: Option<BuiltinCallInfo>,
    ) -> RunError {
        RunError::new(error)
            .at_line(self.current_stmt_line)
            .in_procedure(&self.current_procedure_name())
            .with_builtin_call(call_info)
    }
}

fn invalid_interpolation(reason: &str) -> VBError {
    VBError::with_description(
        err_number::INVALID_PROCEDURE_CALL,
        format!("Invalid interpolated string: {reason}"),
    )
}

fn common_array_element_type(values: &[VBVariant]) -> Option<VBType> {
    let mut common = values.first()?.type_of();
    for value in &values[1..] {
        let next = value.type_of();
        common = match (&common, &next) {
            (left, right) if left == right => left.clone(),
            (
                VBType::Byte | VBType::Integer | VBType::Long | VBType::Enum(_),
                VBType::Byte | VBType::Integer | VBType::Long | VBType::Enum(_),
            ) => integer_widening_type(&common, &next)?,
            (
                VBType::Byte | VBType::Integer | VBType::Long | VBType::Enum(_) | VBType::Single,
                VBType::Single,
            )
            | (VBType::Single, VBType::Byte | VBType::Integer | VBType::Long | VBType::Enum(_)) => {
                VBType::Single
            }
            (
                VBType::Byte
                | VBType::Integer
                | VBType::Long
                | VBType::Enum(_)
                | VBType::Single
                | VBType::Double,
                VBType::Double,
            )
            | (
                VBType::Double,
                VBType::Byte | VBType::Integer | VBType::Long | VBType::Enum(_) | VBType::Single,
            ) => VBType::Double,
            (VBType::Currency, VBType::Byte | VBType::Integer | VBType::Long | VBType::Enum(_))
            | (VBType::Byte | VBType::Integer | VBType::Long | VBType::Enum(_), VBType::Currency) => {
                VBType::Currency
            }
            (VBType::Currency, VBType::Single | VBType::Double)
            | (VBType::Single | VBType::Double, VBType::Currency) => VBType::Double,
            _ => return None,
        };
    }
    Some(match common {
        VBType::Enum(_) => VBType::Long,
        other => other,
    })
}

fn integer_widening_type(left: &VBType, right: &VBType) -> Option<VBType> {
    fn rank(ty: &VBType) -> Option<u8> {
        match ty {
            VBType::Byte => Some(0),
            VBType::Integer => Some(1),
            VBType::Long | VBType::Enum(_) => Some(2),
            _ => None,
        }
    }
    match rank(left)?.max(rank(right)?) {
        0 => Some(VBType::Byte),
        1 => Some(VBType::Integer),
        _ => Some(VBType::Long),
    }
}

// Re-exported for builtins that need currency scaling.
const _: i64 = vb6runtime::CURRENCY_SCALE;
