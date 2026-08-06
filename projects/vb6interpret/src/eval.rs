//! Expression evaluation over the CST.
//!
//! Expressions are evaluated directly against the tree produced by
//! `vb6parse`, producing [`Value`]s from `vb6runtime`.

use crate::builtins;
use crate::error::{RunError, RunResult};
use crate::interpreter::Interpreter;
use crate::scope::normalize;
use vb6core::error::{err_number, VBError, VBResult};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::Value;

impl Interpreter {
    /// Evaluate an expression node to a value.
    pub(crate) fn eval_expr(&mut self, node: &CstNode) -> RunResult<Value> {
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
                if let Some(expr) = parts.into_iter().find(|c| {
                    !matches!(
                        c.kind(),
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
    pub(crate) fn eval_literal(&self, node: &CstNode) -> RunResult<Value> {
        if let Some(token) = node.first_child() {
            return literal_value(token.text(), token.kind())
                .ok_or_else(|| self.error_at(node, VBError::type_mismatch()));
        }
        literal_value(node.text(), node.kind())
            .ok_or_else(|| self.error_at(node, VBError::type_mismatch()))
    }

    /// Evaluate an identifier reference.
    fn eval_identifier(&mut self, node: &CstNode) -> RunResult<Value> {
        let name = crate::program::identifier_name(node);

        match name.to_lowercase().as_str() {
            "true" => return Ok(Value::Boolean(true)),
            "false" => return Ok(Value::Boolean(false)),
            "nothing" => return Ok(Value::Nothing),
            "null" => return Ok(Value::Null),
            "empty" => return Ok(Value::Empty),
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
            None => Ok(Value::Empty),
        }
    }

    /// Evaluate a binary operation. `op` is the operator token node.
    fn eval_binary(&mut self, left: &CstNode, op: &CstNode, right: &CstNode) -> RunResult<Value> {
        let lhs = self.eval_expr(left)?;
        let rhs = self.eval_expr(right)?;

        match op.kind() {
            SyntaxKind::AdditionOperator => self.add(lhs, rhs),
            SyntaxKind::SubtractionOperator => self.arith(lhs, rhs, '-'),
            SyntaxKind::MultiplicationOperator => self.arith(lhs, rhs, '*'),
            SyntaxKind::DivisionOperator => self.arith(lhs, rhs, '/'),
            SyntaxKind::BackwardSlashOperator => self.arith(lhs, rhs, '\\'),
            SyntaxKind::ModKeyword => self.arith(lhs, rhs, 'M'),
            SyntaxKind::ExponentiationOperator => self.arith(lhs, rhs, '^'),
            SyntaxKind::Ampersand => {
                let s = lhs.as_string().map_err(VBError::from)?;
                let t = rhs.as_string().map_err(VBError::from)?;
                Ok(Value::from_string(format!("{s}{t}")))
            }
            SyntaxKind::EqualityOperator => Ok(Value::Boolean(lhs == rhs)),
            SyntaxKind::InequalityOperator => Ok(Value::Boolean(lhs != rhs)),
            SyntaxKind::LessThanOperator => compare_ord(lhs, rhs, Ordering::Less),
            SyntaxKind::LessThanOrEqualOperator => compare_ord(lhs, rhs, Ordering::LessOrEqual),
            SyntaxKind::GreaterThanOperator => compare_ord(lhs, rhs, Ordering::Greater),
            SyntaxKind::GreaterThanOrEqualOperator => {
                compare_ord(lhs, rhs, Ordering::GreaterOrEqual)
            }
            SyntaxKind::AndKeyword => {
                let a = lhs.as_bool().map_err(VBError::from)?;
                let b = rhs.as_bool().map_err(VBError::from)?;
                Ok(Value::Boolean(a && b))
            }
            SyntaxKind::OrKeyword => {
                let a = lhs.as_bool().map_err(VBError::from)?;
                let b = rhs.as_bool().map_err(VBError::from)?;
                Ok(Value::Boolean(a || b))
            }
            SyntaxKind::XorKeyword => {
                let a = lhs.as_bool().map_err(VBError::from)?;
                let b = rhs.as_bool().map_err(VBError::from)?;
                Ok(Value::Boolean(a != b))
            }
            SyntaxKind::EqvKeyword => {
                let a = lhs.as_bool().map_err(VBError::from)?;
                let b = rhs.as_bool().map_err(VBError::from)?;
                Ok(Value::Boolean(a == b))
            }
            SyntaxKind::ImpKeyword => {
                let a = lhs.as_bool().map_err(VBError::from)?;
                let b = rhs.as_bool().map_err(VBError::from)?;
                Ok(Value::Boolean(!a || b))
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
    fn eval_unary(&mut self, op: &CstNode, operand: &CstNode) -> RunResult<Value> {
        let value = self.eval_expr(operand)?;
        match op.kind() {
            SyntaxKind::SubtractionOperator => {
                let n = value.as_f64().map_err(VBError::from)?;
                Ok(Value::from_double(-n))
            }
            SyntaxKind::AdditionOperator => {
                let n = value.as_f64().map_err(VBError::from)?;
                Ok(Value::from_double(n))
            }
            SyntaxKind::NotKeyword => {
                let b = value.as_bool().map_err(VBError::from)?;
                Ok(Value::Boolean(!b))
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
    fn eval_call(&mut self, node: &CstNode) -> RunResult<Value> {
        let name = crate::program::identifier_name(node);

        let argument_list = node
            .significant_children()
            .find(|c| c.kind() == SyntaxKind::ArgumentList);
        let args = match argument_list {
            Some(list) => self.eval_args(list)?,
            None => Vec::new(),
        };

        // Array indexing: the name resolves to an array variable.
        if let Some(Value::Array(_)) = self.lookup(&name) {
            let indices: VBResult<Vec<i32>> = args
                .iter()
                .map(|a| a.as_i32().map_err(VBError::from))
                .collect();
            let indices = indices.map_err(VBError::from)?;
            let array = self.lookup(&name).ok_or_else(VBError::object_not_set)?;
            if let Value::Array(array) = array {
                let element = array.get(&indices).map_err(VBError::from)?;
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
    pub(crate) fn eval_args(&mut self, node: &CstNode) -> RunResult<Vec<Value>> {
        let mut values = Vec::new();
        for argument in node.children_by_kind(SyntaxKind::Argument) {
            if let Some(expr) = argument
                .significant_children()
                .find(|c| !matches!(c.kind(), SyntaxKind::Comma))
            {
                values.push(self.eval_expr(expr)?);
            }
        }
        Ok(values)
    }

    /// `+` operator: numeric addition, or string concatenation when both
    /// operands are strings.
    fn add(&self, lhs: Value, rhs: Value) -> RunResult<Value> {
        match (&lhs, &rhs) {
            (Value::String(_), Value::String(_)) => {
                let s = lhs.as_string().map_err(VBError::from)?;
                let t = rhs.as_string().map_err(VBError::from)?;
                Ok(Value::from_string(format!("{s}{t}")))
            }
            (Value::String(_), _) | (_, Value::String(_)) => {
                Err(self.error_here(VBError::type_mismatch()))
            }
            _ => self.arith(lhs, rhs, '+'),
        }
    }

    /// Generic arithmetic dispatch on an operator char.
    pub(crate) fn arith(&self, lhs: Value, rhs: Value, op: char) -> RunResult<Value> {
        // Integer arithmetic when both operands are integral.
        if op != '/' && op != '^' {
            let li = lhs.as_i64().ok();
            let ri = rhs.as_i64().ok();
            if let (Some(a), Some(b)) = (li, ri) {
                let result = match op {
                    '+' => a.checked_add(b),
                    '-' => a.checked_sub(b),
                    '*' => a.checked_mul(b),
                    '\\' => {
                        if b == 0 {
                            return Err(self.error_here(VBError::division_by_zero()));
                        }
                        Some(a.div_euclid(b))
                    }
                    'M' => {
                        if b == 0 {
                            return Err(self.error_here(VBError::division_by_zero()));
                        }
                        Some(a.rem_euclid(b))
                    }
                    _ => None,
                };
                if let Some(v) = result {
                    return Ok(Value::from_i64(v));
                }
            }
        }

        let a = lhs.as_f64().map_err(VBError::from)?;
        let b = rhs.as_f64().map_err(VBError::from)?;
        let result = match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => {
                if b == 0.0 {
                    return Err(self.error_here(VBError::division_by_zero()));
                }
                a / b
            }
            '\\' => {
                if b == 0.0 {
                    return Err(self.error_here(VBError::division_by_zero()));
                }
                (a / b).floor()
            }
            'M' => {
                if b == 0.0 {
                    return Err(self.error_here(VBError::division_by_zero()));
                }
                a % b
            }
            '^' => a.powf(b),
            _ => unreachable!(),
        };
        Ok(Value::from_double(result))
    }

    /// Look up a variable in the current frame, then in globals.
    pub(crate) fn lookup(&self, name: &str) -> Option<&Value> {
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ordering {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

/// Ordered comparison with VB6 coercion.
fn compare_ord(lhs: Value, rhs: Value, ord: Ordering) -> RunResult<Value> {
    let a = match (&lhs, &rhs) {
        (Value::String(s), Value::String(t)) => compare_strings(s, t),
        _ => match (lhs.as_f64(), rhs.as_f64()) {
            (Ok(a), Ok(b)) => a.partial_cmp(&b),
            _ => {
                return Err(RunError::new(VBError::type_mismatch()));
            }
        },
    };
    let result = match (a, ord) {
        (Some(std::cmp::Ordering::Less), Ordering::Less) => true,
        (Some(std::cmp::Ordering::Greater), Ordering::Greater) => true,
        (Some(std::cmp::Ordering::Less), Ordering::LessOrEqual) => true,
        (Some(std::cmp::Ordering::Equal), Ordering::LessOrEqual) => true,
        (Some(std::cmp::Ordering::Greater), Ordering::GreaterOrEqual) => true,
        (Some(std::cmp::Ordering::Equal), Ordering::GreaterOrEqual) => true,
        _ => false,
    };
    Ok(Value::Boolean(result))
}

/// Case-insensitive string comparison (VB6 default `Option Compare`).
fn compare_strings(a: &str, b: &str) -> Option<std::cmp::Ordering> {
    let la = a.to_lowercase();
    let lb = b.to_lowercase();
    Some(la.cmp(&lb))
}

/// Parse a literal token's text into a runtime value.
///
/// Handles the raw literal token kinds (`IntegerLiteral`, `StringLiteral`, ...)
/// and suffix characters (`%` Integer, `&` Long, `!` Single, `#` Double, `@`
/// Currency).
pub(crate) fn literal_value(text: &str, kind: SyntaxKind) -> Option<Value> {
    let raw = text.trim();

    match kind {
        SyntaxKind::StringLiteral => {
            let inner = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"'))?;
            let unescaped = inner.replace("\"\"", "\"");
            Some(Value::from_string(unescaped))
        }
        SyntaxKind::DateLiteral => {
            let inner = raw.strip_prefix('#').and_then(|s| s.strip_suffix('#'))?;
            Value::from_string(inner)
                .as_date_serial()
                .ok()
                .map(Value::Date)
        }
        SyntaxKind::TrueKeyword => Some(Value::Boolean(true)),
        SyntaxKind::FalseKeyword => Some(Value::Boolean(false)),
        SyntaxKind::IntegerLiteral => parse_integer(raw),
        SyntaxKind::LongLiteral => parse_long(raw),
        SyntaxKind::SingleLiteral => {
            let s = strip_suffix(raw);
            s.parse::<f32>().ok().map(Value::from_single)
        }
        SyntaxKind::DoubleLiteral => {
            let s = strip_suffix(raw);
            s.parse::<f64>().ok().map(Value::from_double)
        }
        SyntaxKind::CurrencyLiteral => {
            let s = strip_suffix(raw);
            s.parse::<f64>().ok().map(Value::from_currency)
        }
        SyntaxKind::DecimalLiteral => {
            let s = strip_suffix(raw);
            s.parse::<f64>().ok().map(Value::from_double)
        }
        _ => None,
    }
}

/// Strip a trailing VB6 type-suffix character.
fn strip_suffix(raw: &str) -> &str {
    match raw.chars().last() {
        Some('%') | Some('&') | Some('!') | Some('#') | Some('@') => &raw[..raw.len() - 1],
        _ => raw,
    }
}

/// Parse an integer literal into Integer (i16) or Long (i32) semantics.
fn parse_integer(raw: &str) -> Option<Value> {
    let s = strip_suffix(raw);
    let upper = s.to_ascii_uppercase();
    if let Some(digits) = upper.strip_prefix("&H") {
        return radix_value(digits, 16);
    }
    if let Some(digits) = upper.strip_prefix("&O") {
        return radix_value(digits, 8);
    }
    let value = s.parse::<i64>().ok()?;
    Some(Value::from_i64(value))
}

/// Parse a `LongLiteral` (always a Long).
fn parse_long(raw: &str) -> Option<Value> {
    let s = strip_suffix(raw);
    let upper = s.to_ascii_uppercase();
    if let Some(digits) = upper.strip_prefix("&H") {
        return radix_value(digits, 16);
    }
    if let Some(digits) = upper.strip_prefix("&O") {
        return radix_value(digits, 8);
    }
    s.parse::<i32>().ok().map(Value::Long)
}

/// Parse a radix-prefixed literal, honoring VB6's wrap of 32-bit values.
fn radix_value(digits: &str, radix: u32) -> Option<Value> {
    let digits = digits.trim().trim_end_matches('%').trim_end_matches('&');
    if digits.is_empty() {
        return None;
    }
    if let Ok(v) = i64::from_str_radix(digits, radix) {
        return Some(Value::from_i64(v));
    }
    // `&HFFFFFFFF` wraps to -1 Long in VB6.
    if let Ok(v) = u32::from_str_radix(digits, radix) {
        return Some(Value::Long(v as i32));
    }
    None
}

// Re-exported for builtins that need currency scaling.
const _: i64 = vb6runtime::CURRENCY_SCALE;
