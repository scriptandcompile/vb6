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

/// Logical/bitwise operators shared by `And`, `Or`, `Xor`, `Eqv`, `Imp`.
#[derive(Clone, Copy, Debug)]
enum LogicalOperator {
    And,
    Or,
    Xor,
    Eqv,
    Imp,
}

/// Arithmetic operators shared by `+`, `-`, `*`, `/`, `\`, `Mod`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArithmaticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    IntegerDivide,
    Modulus,
    Exponent,
}

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
            SyntaxKind::SubtractionOperator => self.arith(lhs, rhs, ArithmaticOperator::Subtract),
            SyntaxKind::MultiplicationOperator => {
                self.arith(lhs, rhs, ArithmaticOperator::Multiply)
            }
            SyntaxKind::DivisionOperator => self.arith(lhs, rhs, ArithmaticOperator::Divide),
            SyntaxKind::BackwardSlashOperator => {
                self.arith(lhs, rhs, ArithmaticOperator::IntegerDivide)
            }
            SyntaxKind::ModKeyword => self.arith(lhs, rhs, ArithmaticOperator::Modulus),
            SyntaxKind::ExponentiationOperator => {
                self.arith(lhs, rhs, ArithmaticOperator::Exponent)
            }
            SyntaxKind::Ampersand => {
                let left = lhs.as_string()?;
                let right = rhs.as_string()?;
                Ok(Value::from_string(format!("{left}{right}")))
            }
            SyntaxKind::EqualityOperator => Ok(Value::Boolean(lhs == rhs)),
            SyntaxKind::InequalityOperator => Ok(Value::Boolean(lhs != rhs)),
            SyntaxKind::LessThanOperator => compare_ord(lhs, rhs, Ordering::Less),
            SyntaxKind::LessThanOrEqualOperator => compare_ord(lhs, rhs, Ordering::LessOrEqual),
            SyntaxKind::GreaterThanOperator => compare_ord(lhs, rhs, Ordering::Greater),
            SyntaxKind::GreaterThanOrEqualOperator => {
                compare_ord(lhs, rhs, Ordering::GreaterOrEqual)
            }
            SyntaxKind::AndKeyword => self.bitwise(lhs, rhs, LogicalOperator::And),
            SyntaxKind::OrKeyword => self.bitwise(lhs, rhs, LogicalOperator::Or),
            SyntaxKind::XorKeyword => self.bitwise(lhs, rhs, LogicalOperator::Xor),
            SyntaxKind::EqvKeyword => self.bitwise(lhs, rhs, LogicalOperator::Eqv),
            SyntaxKind::ImpKeyword => self.bitwise(lhs, rhs, LogicalOperator::Imp),
            SyntaxKind::IsKeyword => {
                let result = match (&lhs, &rhs) {
                    (Value::Nothing, Value::Nothing) => true,
                    (Value::Nothing, _) | (_, Value::Nothing) => false,
                    // No object model yet: fall back to value equality.
                    _ => lhs == rhs,
                };
                Ok(Value::Boolean(result))
            }
            SyntaxKind::LikeKeyword => {
                let text = lhs.as_string()?;
                let pattern = rhs.as_string()?;
                Ok(Value::Boolean(like_match(&pattern, &text)))
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
                let number = value.as_f64()?;
                Ok(Value::from_double(-number))
            }
            SyntaxKind::AdditionOperator => {
                let number = value.as_f64()?;
                Ok(Value::from_double(number))
            }
            SyntaxKind::NotKeyword => {
                let boolean = value.as_bool()?;
                Ok(Value::Boolean(!boolean))
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
            .find(|child| child.kind() == SyntaxKind::ArgumentList);
        let args = match argument_list {
            Some(list) => self.eval_args(list)?,
            None => Vec::new(),
        };

        // Array indexing: the name resolves to an array variable.
        if let Some(Value::Array(_)) = self.lookup(&name) {
            let indices: VBResult<Vec<i32>> = args.iter().map(|arg| arg.as_i32()).collect();
            let indices = indices?;
            let array = self.lookup(&name).ok_or_else(VBError::object_not_set)?;
            if let Value::Array(array) = array {
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
    pub(crate) fn eval_args(&mut self, node: &CstNode) -> RunResult<Vec<Value>> {
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

    /// `+` operator: numeric addition, or string concatenation when both
    /// operands are strings.
    fn add(&self, lhs: Value, rhs: Value) -> RunResult<Value> {
        match (&lhs, &rhs) {
            (Value::String(_), Value::String(_)) => {
                let left = lhs.as_string()?;
                let right = rhs.as_string()?;
                Ok(Value::from_string(format!("{left}{right}")))
            }
            (Value::String(_), _) | (_, Value::String(_)) => {
                Err(self.error_here(VBError::type_mismatch()))
            }
            _ => self.arith(lhs, rhs, ArithmaticOperator::Add),
        }
    }

    /// Generic arithmetic dispatch on an arithmetic operator.
    pub(crate) fn arith(&self, lhs: Value, rhs: Value, op: ArithmaticOperator) -> RunResult<Value> {
        // Integer arithmetic when both operands are integral types. (This
        // must not fire for Singles/Doubles/Currency, which hold fractional
        // values that `as_i64` would silently round.)
        if op != ArithmaticOperator::Divide && op != ArithmaticOperator::Exponent {
            if lhs.is_integral() && rhs.is_integral() {
                let li = lhs.as_i64().ok();
                let ri = rhs.as_i64().ok();
                if let (Some(left), Some(right)) = (li, ri) {
                    let result = match op {
                        ArithmaticOperator::Add => left.checked_add(right),
                        ArithmaticOperator::Subtract => left.checked_sub(right),
                        ArithmaticOperator::Multiply => left.checked_mul(right),
                        ArithmaticOperator::IntegerDivide => {
                            if right == 0 {
                                return Err(self.error_here(VBError::division_by_zero()));
                            }
                            Some(left.div_euclid(right))
                        }
                        ArithmaticOperator::Modulus => {
                            if right == 0 {
                                return Err(self.error_here(VBError::division_by_zero()));
                            }
                            Some(left.rem_euclid(right))
                        }
                        _ => None,
                    };
                    if let Some(value) = result {
                        return Ok(Value::from_i64(value));
                    }
                }
            }
        }

        let left = lhs.as_f64()?;
        let right = rhs.as_f64()?;
        let result = match op {
            ArithmaticOperator::Add => left + right,
            ArithmaticOperator::Subtract => left - right,
            ArithmaticOperator::Multiply => left * right,
            ArithmaticOperator::Divide => {
                if right == 0.0 {
                    return Err(self.error_here(VBError::division_by_zero()));
                }
                left / right
            }
            ArithmaticOperator::IntegerDivide => {
                if right == 0.0 {
                    return Err(self.error_here(VBError::division_by_zero()));
                }
                (left / right).floor()
            }
            ArithmaticOperator::Modulus => {
                if right == 0.0 {
                    return Err(self.error_here(VBError::division_by_zero()));
                }
                left % right
            }
            ArithmaticOperator::Exponent => left.powf(right),
            // Add other operators here if needed.
        };
        Ok(Value::from_double(result))
    }

    /// Logical/bitwise operators. Booleans combine logically and yield a
    /// Boolean; any other operands combine bitwise over their integral value
    /// (booleans coerce to -1/0), like VB6.
    fn bitwise(&self, lhs: Value, rhs: Value, op: LogicalOperator) -> RunResult<Value> {
        if let (Value::Boolean(left), Value::Boolean(right)) = (&lhs, &rhs) {
            let result = match op {
                LogicalOperator::And => *left && *right,
                LogicalOperator::Or => *left || *right,
                LogicalOperator::Xor => *left != *right,
                LogicalOperator::Eqv => *left == *right,
                LogicalOperator::Imp => !*left || *right,
            };
            return Ok(Value::Boolean(result));
        }

        let left = lhs.as_i64()?;
        let right = rhs.as_i64()?;
        let result = match op {
            LogicalOperator::And => left & right,
            LogicalOperator::Or => left | right,
            LogicalOperator::Xor => left ^ right,
            LogicalOperator::Eqv => !(left ^ right),
            LogicalOperator::Imp => !left | right,
        };

        Ok(Value::from_i64(result))
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
    let ordering = match (&lhs, &rhs) {
        (Value::String(left), Value::String(right)) => compare_strings(left, right),
        _ => match (lhs.as_f64(), rhs.as_f64()) {
            (Ok(left), Ok(right)) => left.partial_cmp(&right),
            _ => {
                return Err(RunError::new(VBError::type_mismatch()));
            }
        },
    };
    let result = matches!(
        (ordering, ord),
        (Some(std::cmp::Ordering::Less), Ordering::Less)
            | (Some(std::cmp::Ordering::Less), Ordering::LessOrEqual)
            | (Some(std::cmp::Ordering::Equal), Ordering::LessOrEqual)
            | (Some(std::cmp::Ordering::Greater), Ordering::Greater)
            | (Some(std::cmp::Ordering::Greater), Ordering::GreaterOrEqual)
            | (Some(std::cmp::Ordering::Equal), Ordering::GreaterOrEqual)
    );
    Ok(Value::Boolean(result))
}

/// Case-insensitive string comparison (VB6 default `Option Compare`).
fn compare_strings(left: &str, right: &str) -> Option<std::cmp::Ordering> {
    let left_lower = left.to_lowercase();
    let right_lower = right.to_lowercase();
    Some(left_lower.cmp(&right_lower))
}

/// VB6 `Like` pattern match, case-insensitive (the interpreter's default
/// `Option Compare`). Supports `?`, `*`, `#`, and `[charlist]` / `[!charlist]`
/// classes with `a-z` ranges. A literal `[`, `?`, `*`, or `#` is matched by
/// enclosing it in brackets (e.g. `[[]`, `[?]`).
fn like_match(pattern: &str, text: &str) -> bool {
    let pat: Vec<char> = pattern.chars().collect();
    let txt: Vec<char> = text.chars().collect();
    let mut memo = vec![vec![None; txt.len() + 1]; pat.len() + 1];
    like_match_at(&pat, &txt, 0, 0, &mut memo)
}

/// Memoized recursion over pattern position `pat_idx` and text position
/// `text_idx`.
fn like_match_at(
    pat: &[char],
    txt: &[char],
    pat_idx: usize,
    text_idx: usize,
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if let Some(cached) = memo[pat_idx][text_idx] {
        return cached;
    }
    let result = if pat_idx == pat.len() {
        text_idx == txt.len()
    } else if pat[pat_idx] == '*' {
        // Match zero or more characters.
        (like_match_at(pat, txt, pat_idx + 1, text_idx, memo))
            || (text_idx < txt.len() && like_match_at(pat, txt, pat_idx, text_idx + 1, memo))
    } else if pat[pat_idx] == '[' {
        let closed = pat[pat_idx + 1..].iter().position(|&ch| ch == ']');
        match closed {
            Some(close) if text_idx < txt.len() => {
                let (matched, next) = match_class(pat, pat_idx, pat_idx + 1 + close, txt[text_idx]);
                matched && like_match_at(pat, txt, next, text_idx + 1, memo)
            }
            // No closing bracket: treat `[` as a literal character.
            _ => {
                text_idx < txt.len()
                    && pat[pat_idx] == txt[text_idx]
                    && like_match_at(pat, txt, pat_idx + 1, text_idx + 1, memo)
            }
        }
    } else if text_idx < txt.len() {
        let ok = match pat[pat_idx] {
            '?' => true,
            '#' => txt[text_idx].is_ascii_digit(),
            ch => chars_equal(ch, txt[text_idx]),
        };
        ok && like_match_at(pat, txt, pat_idx + 1, text_idx + 1, memo)
    } else {
        false
    };
    memo[pat_idx][text_idx] = Some(result);
    result
}

/// Match a single character against a `[charlist]` class spanning
/// `pat[open]..=pat[close]`. Returns whether it matched and the pattern index
/// just past the closing `]`.
fn match_class(pat: &[char], open: usize, close: usize, ch: char) -> (bool, usize) {
    let mut index = open + 1;
    let negate = index < close && pat[index] == '!';
    if negate {
        index += 1;
    }
    let mut matched = false;
    while index < close {
        // `x-y` range.
        if index + 2 < close && pat[index + 1] == '-' {
            matched |= between_chars(pat[index], ch, pat[index + 2]);
            index += 3;
        } else {
            matched |= chars_equal(pat[index], ch);
            index += 1;
        }
    }
    (if negate { !matched } else { matched }, close + 1)
}

/// Case-insensitive character equality.
fn chars_equal(left: char, right: char) -> bool {
    left.to_lowercase().eq(right.to_lowercase())
}

/// Whether `lo <= ch <= hi`, case-insensitively.
fn between_chars(lo: char, ch: char, hi: char) -> bool {
    let lo = lo.to_lowercase().next().unwrap_or(lo);
    let ch = ch.to_lowercase().next().unwrap_or(ch);
    let hi = hi.to_lowercase().next().unwrap_or(hi);
    lo <= ch && ch <= hi
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
            let inner = raw
                .strip_prefix('"')
                .and_then(|rest| rest.strip_suffix('"'))?;
            let unescaped = inner.replace("\"\"", "\"");
            Some(Value::from_string(unescaped))
        }
        SyntaxKind::DateLiteral => {
            let inner = raw
                .strip_prefix('#')
                .and_then(|rest| rest.strip_suffix('#'))?;
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
            let text = strip_suffix(raw);
            text.parse::<f32>().ok().map(Value::from_single)
        }
        SyntaxKind::DoubleLiteral => {
            let text = strip_suffix(raw);
            text.parse::<f64>().ok().map(Value::from_double)
        }
        SyntaxKind::CurrencyLiteral => {
            let text = strip_suffix(raw);
            text.parse::<f64>().ok().map(Value::from_currency)
        }
        SyntaxKind::DecimalLiteral => {
            let text = strip_suffix(raw);
            text.parse::<f64>().ok().map(Value::from_double)
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
    let text = strip_suffix(raw);
    let upper = text.to_ascii_uppercase();
    if let Some(digits) = upper.strip_prefix("&H") {
        return radix_value(digits, 16);
    }
    if let Some(digits) = upper.strip_prefix("&O") {
        return radix_value(digits, 8);
    }
    let value = text.parse::<i64>().ok()?;
    Some(Value::from_i64(value))
}

/// Parse a `LongLiteral` (always a Long).
fn parse_long(raw: &str) -> Option<Value> {
    let text = strip_suffix(raw);
    let upper = text.to_ascii_uppercase();
    if let Some(digits) = upper.strip_prefix("&H") {
        return radix_value(digits, 16);
    }
    if let Some(digits) = upper.strip_prefix("&O") {
        return radix_value(digits, 8);
    }
    text.parse::<i32>().ok().map(Value::Long)
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
