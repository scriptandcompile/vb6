//! Control-flow statements: `If`/`ElseIf`/`Else`, `For`/`Next`, `Do`/`Loop`,
//! `While`/`Wend`, `Select Case`, and `Exit`.

use vb6core::error::VBError;
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::VBVariant;

use super::count_newlines;
use crate::error::RunResult;
use crate::eval::{arith, ArithmeticOperator};
use crate::interpreter::{Flow, Interpreter};
use crate::program::is_statement_kind;

/// Whether a node kind is a literal token (usable as a case value or bound).
fn is_literal_kind(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::IntegerLiteral
            | SyntaxKind::LongLiteral
            | SyntaxKind::SingleLiteral
            | SyntaxKind::DoubleLiteral
            | SyntaxKind::CurrencyLiteral
            | SyntaxKind::DecimalLiteral
            | SyntaxKind::DateLiteral
            | SyntaxKind::StringLiteral
            | SyntaxKind::TrueKeyword
            | SyntaxKind::FalseKeyword
    )
}

/// Trait to find the node following a keyword in a significant-child list.
trait NextSignificant {
    fn next_significant<'a>(&self, siblings: &[&'a CstNode]) -> Option<&'a CstNode>;
}

impl NextSignificant for CstNode {
    fn next_significant<'a>(&self, siblings: &[&'a CstNode]) -> Option<&'a CstNode> {
        siblings
            .iter()
            .position(|c| std::ptr::eq(*c, self))
            .and_then(|i| siblings.get(i + 1))
            .copied()
    }
}

impl Interpreter {
    /// `If` statement (block or single-line form).
    pub(crate) fn exec_if(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();

        // Find the ThenKeyword; the condition is the last significant child
        // before it.
        let then_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::ThenKeyword)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let cond = children[..then_index]
            .iter()
            .rev()
            .find(|c| c.is_significant())
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let cond_true = self.eval_expr(cond)?.as_bool()?;

        let is_block = node
            .first_child_by_kind(SyntaxKind::StatementList)
            .is_some();
        if !is_block {
            // Single-line form: statements after Then, optionally an Else.
            let mut matched = cond_true;
            let mut executed = false;
            for child in &children[then_index + 1..] {
                if child.kind() == SyntaxKind::ElseKeyword {
                    matched = !cond_true;
                    continue;
                }
                if !matched || !is_statement_kind(child.kind()) {
                    continue;
                }
                let flow = self.exec_stmt(child, line)?;
                if flow != Flow::Next {
                    return Ok(flow);
                }
                executed = true;
            }
            let _ = executed;
            return Ok(Flow::Next);
        }

        // Block form: walk children, tracking the line, executing the body of
        // the first matching branch only.
        let mut cur_line = line;
        let mut taken = false;
        for child in &children[then_index + 1..] {
            match child.kind() {
                SyntaxKind::Newline => cur_line += 1,
                SyntaxKind::StatementList => {
                    if cond_true && !taken {
                        let flow = self.exec_statements(child, cur_line)?;
                        if flow != Flow::Next {
                            return Ok(flow);
                        }
                        taken = true;
                    }
                }
                SyntaxKind::ElseIfClause => {
                    if !cond_true && !taken {
                        let clause_children: Vec<&CstNode> = child.children().iter().collect();
                        if let Some(ti) = clause_children
                            .iter()
                            .position(|c| c.kind() == SyntaxKind::ThenKeyword)
                        {
                            let elseif_cond = clause_children[..ti]
                                .iter()
                                .rev()
                                .find(|c| c.is_significant())
                                .ok_or_else(|| {
                                    self.error_here(VBError::invalid_procedure_call())
                                })?;
                            let elseif_true = self.eval_expr(elseif_cond)?.as_bool()?;
                            if elseif_true {
                                let flow = self.exec_body_of(child, cur_line)?;
                                if flow != Flow::Next {
                                    return Ok(flow);
                                }
                                taken = true;
                            }
                        }
                    }
                    cur_line += count_newlines(child);
                }
                SyntaxKind::ElseClause => {
                    if !taken {
                        let flow = self.exec_body_of(child, cur_line)?;
                        if flow != Flow::Next {
                            return Ok(flow);
                        }
                        taken = true;
                    }
                    cur_line += count_newlines(child);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// Execute the first `StatementList` inside a clause, starting at `line`.
    fn exec_body_of(&mut self, clause: &CstNode, line: usize) -> RunResult<Flow> {
        let mut cur_line = line;
        for child in clause.children() {
            match child.kind() {
                SyntaxKind::Newline => cur_line += 1,
                SyntaxKind::StatementList => {
                    return self.exec_statements(child, cur_line);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// `For var = start To end [Step step] ... Next [var]`.
    pub(crate) fn exec_for(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let counter = node.first_child_by_kind(SyntaxKind::IdentifierExpression);
        let name = counter
            .and_then(|e| e.first_child_by_kind(SyntaxKind::Identifier))
            .map(|t| t.text().trim().to_string())
            .unwrap_or_default();

        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator);
        let to_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::ToKeyword);
        let (Some(eq_index), Some(to_index)) = (eq_index, to_index) else {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        };

        let start_node = significant
            .get(eq_index + 1)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let end_node = significant
            .get(to_index + 1)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;

        let mut step = VBVariant::from_long(1);
        let mut step_cursor = None;
        if let Some(step_idx) = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::StepKeyword)
        {
            if let Some(step_node) = significant.get(step_idx + 1) {
                step = self.eval_expr(step_node)?;
                step_cursor = Some((significant[step_idx].start_offset(), step_node.end_offset()));
            }
        }

        let start = self.eval_expr(start_node)?;
        let end = self.eval_expr(end_node)?;
        let step_f = step.as_f64()?;
        if step_f == 0.0 {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }

        let body_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::StatementList);
        let body_line = match body_index {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        // Sub-line cursor targets for the loop's own elements: the counter
        // assignment (`i = 1`), the end check (`To 100`), the step
        // (`Step 5`), and the `Next i` closing line.
        let start_cursor = counter.map(|c| (c.start_offset(), start_node.end_offset()));
        let to_cursor = Some((significant[to_index].start_offset(), end_node.end_offset()));
        let next_line = match children
            .iter()
            .position(|c| c.kind() == SyntaxKind::NextKeyword)
        {
            Some(next_idx) => {
                line + children[..next_idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };
        let next_cursor = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::NextKeyword)
            .map(|next_idx| {
                let next_kw = significant[next_idx];
                let next_end = significant
                    .get(next_idx + 1)
                    .filter(|c| c.kind() == SyntaxKind::Identifier)
                    .map(|c| c.end_offset())
                    .unwrap_or_else(|| next_kw.end_offset());
                (next_kw.start_offset(), next_end)
            });

        let mut counter_value = start;
        let mut first = true;
        loop {
            // On the first pass highlight the counter assignment; on later
            // passes highlight the check against the end value. Rewinding the
            // line to the loop top is only meaningful for trace snapshots; in
            // run mode it would leave the final highlight stuck on the loop
            // header instead of the `Next` line where the loop ends.
            if self.record_debug_snapshots {
                self.current_stmt_line = line;
            }
            let cursor = if first { start_cursor } else { to_cursor };
            first = false;
            self.step_marked(cursor)?;
            let current = counter_value.as_f64()?;
            let end_value = end.as_f64()?;
            let done = if step_f >= 0.0 {
                current > end_value
            } else {
                current < end_value
            };
            if done {
                break;
            }
            self.set_variable(&name, counter_value.clone());
            if let Some(cursor) = step_cursor {
                self.step_marked(Some(cursor))?;
            }
            if let Some(idx) = body_index {
                let flow = self.exec_statements(children[idx], body_line)?;
                match flow {
                    Flow::Next => {}
                    Flow::BreakLoop => break,
                    Flow::Return | Flow::Terminate => return Ok(flow),
                }
            }
            self.current_stmt_line = next_line;
            self.step_marked(next_cursor)?;
            counter_value = arith(counter_value, step.clone(), ArithmeticOperator::Add)
                .map_err(|e| self.error_here(e))?;
        }
        self.set_variable(&name, counter_value);
        Ok(Flow::Next)
    }

    /// `Do ... Loop` with optional `While`/`Until` pre- and post-tests.
    pub(crate) fn exec_do(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let body_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::StatementList);
        let body_line = match body_index {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        let do_index = significant
            .iter()
            .position(|part| part.kind() == SyntaxKind::DoKeyword)
            .unwrap_or(0);
        let do_cursor = significant
            .get(do_index)
            .map(|part| (part.start_offset(), part.end_offset()));
        let loop_index = significant
            .iter()
            .position(|part| part.kind() == SyntaxKind::LoopKeyword);
        let loop_cursor = loop_index
            .and_then(|li| significant.get(li))
            .map(|part| (part.start_offset(), part.end_offset()));
        let loop_line = match children
            .iter()
            .position(|c| c.kind() == SyntaxKind::LoopKeyword)
        {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        // Pre-test: `Do While cond` / `Do Until cond`. The cursor spans the
        // `While`/`Until` keyword through the end of the condition.
        let mut pre_test: Option<(bool, &CstNode)> = None;
        let mut pre_cursor = None;
        for part in &significant[do_index + 1..] {
            match part.kind() {
                SyntaxKind::WhileKeyword | SyntaxKind::UntilKeyword => {
                    if let Some(next) = part.next_significant(significant.as_slice()) {
                        let invert = part.kind() == SyntaxKind::UntilKeyword;
                        pre_test = Some((invert, next));
                        pre_cursor = Some((part.start_offset(), next.end_offset()));
                    }
                    break;
                }
                SyntaxKind::StatementList => break,
                _ => {}
            }
        }

        // Post-test: `Loop While cond` / `Loop Until cond`. The cursor spans
        // the `While`/`Until` keyword through the end of the condition.
        let mut post_test: Option<(bool, &CstNode)> = None;
        let mut post_cursor = None;
        if let Some(li) = loop_index {
            let after: Vec<&CstNode> = significant[li + 1..].to_vec();
            for part in &after {
                match part.kind() {
                    SyntaxKind::WhileKeyword | SyntaxKind::UntilKeyword => {
                        if let Some(next) = part.next_significant(&after) {
                            let invert = part.kind() == SyntaxKind::UntilKeyword;
                            post_test = Some((invert, next));
                            post_cursor = Some((part.start_offset(), next.end_offset()));
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }

        loop {
            // Entry: highlight the pre-test condition, or the bare `Do`
            // keyword when the loop has no pre-test. Rewinding the line to
            // the loop top is only meaningful for trace snapshots; in run
            // mode it would leave the final highlight stuck on the `Do` line
            // instead of the `Loop` line where the loop ends.
            if self.record_debug_snapshots {
                self.current_stmt_line = line;
            }
            self.step_marked(pre_cursor.or(do_cursor))?;
            if let Some((invert, cond)) = pre_test {
                let condition = self.eval_expr(cond)?.as_bool()?;
                if condition == invert {
                    break;
                }
            }
            if let Some(idx) = body_index {
                let flow = self.exec_statements(children[idx], body_line)?;
                match flow {
                    Flow::Next => {}
                    Flow::BreakLoop => break,
                    Flow::Return | Flow::Terminate => return Ok(flow),
                }
            }
            // After the body: highlight the post-test condition, or the
            // `Loop` keyword when the loop has no post-test.
            self.current_stmt_line = loop_line;
            if post_test.is_some() {
                self.step_marked(post_cursor)?;
                if let Some((invert, cond)) = post_test {
                    let b = self.eval_expr(cond)?.as_bool()?;
                    if b == invert {
                        break;
                    }
                }
            } else if let Some(loop_c) = loop_cursor {
                self.step_marked(Some(loop_c))?;
            }
        }
        Ok(Flow::Next)
    }

    /// `While cond ... Wend`.
    pub(crate) fn exec_while(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let cond = significant
            .iter()
            .find(|c| !matches!(c.kind(), SyntaxKind::WhileKeyword))
            .copied()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let cond_cursor = Some((cond.start_offset(), cond.end_offset()));
        let wend_cursor = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::WendKeyword)
            .map(|idx| {
                let wend = significant[idx];
                (wend.start_offset(), wend.end_offset())
            });
        let wend_line = match children
            .iter()
            .position(|c| c.kind() == SyntaxKind::WendKeyword)
        {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        let body_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::StatementList);
        let body_line = match body_index {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        loop {
            // Rewinding the line to the loop top is only meaningful for trace
            // snapshots; in run mode it would leave the final highlight stuck
            // on the `While` header instead of the `Wend` line where the loop
            // ends.
            if self.record_debug_snapshots {
                self.current_stmt_line = line;
            }
            self.step_marked(cond_cursor)?;
            let b = self.eval_expr(cond)?.as_bool()?;
            if !b {
                break;
            }
            if let Some(idx) = body_index {
                let flow = self.exec_statements(children[idx], body_line)?;
                match flow {
                    Flow::Next => {}
                    Flow::BreakLoop => break,
                    Flow::Return | Flow::Terminate => return Ok(flow),
                }
            }
            self.current_stmt_line = wend_line;
            if let Some(cursor) = wend_cursor {
                self.step_marked(Some(cursor))?;
            }
        }
        Ok(Flow::Next)
    }

    /// `Select Case expr ... Case ... End Select`.
    pub(crate) fn exec_select(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let case_keyword = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::CaseKeyword);
        let selector = match case_keyword {
            Some(idx) => {
                let expr = significant
                    .get(idx + 1)
                    .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
                self.eval_expr(expr)?
            }
            None => VBVariant::Empty,
        };

        let mut cur_line = line;
        for child in children {
            match child.kind() {
                SyntaxKind::Newline => cur_line += 1,
                SyntaxKind::CaseClause | SyntaxKind::CaseElseClause => {
                    let is_else = child.kind() == SyntaxKind::CaseElseClause;
                    let matches = if is_else {
                        true
                    } else {
                        self.case_clause_matches(child, &selector)?
                    };
                    if matches {
                        let flow = self.exec_body_of(child, cur_line)?;
                        return Ok(flow);
                    }
                    cur_line += count_newlines(child);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// Whether a `CaseClause` matches the selector.
    fn case_clause_matches(&mut self, clause: &CstNode, selector: &VBVariant) -> RunResult<bool> {
        let significant: Vec<&CstNode> = clause.significant_children().collect();
        // Skip the leading CaseKeyword and stop at the body StatementList.
        let mut spec: Vec<&CstNode> = Vec::new();
        for c in &significant[1..] {
            if c.kind() == SyntaxKind::StatementList {
                break;
            }
            spec.push(c);
        }

        let mut index = 0;
        while index < spec.len() {
            match spec[index].kind() {
                SyntaxKind::Comma => index += 1,
                SyntaxKind::IsKeyword => {
                    let op = spec
                        .get(index + 1)
                        .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
                    let value_node = spec
                        .get(index + 2)
                        .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
                    let value = self.eval_case_value(value_node)?;
                    let matched = self.apply_compare(selector, op.kind(), &value)?;
                    if matched {
                        return Ok(true);
                    }
                    index += 3;
                }
                SyntaxKind::ToKeyword => {
                    // Range: `low To high`.
                    let low = self.eval_case_value(spec[index - 1])?;
                    let high = self.eval_case_value(spec[index + 1])?;
                    if self.in_range(selector, &low, &high)? {
                        return Ok(true);
                    }
                    index += 2;
                }
                _ => {
                    let value = self.eval_case_value(spec[index])?;
                    let mut matched = selector == &value;
                    // A trailing `To` opens a range.
                    if index + 1 < spec.len() && spec[index + 1].kind() == SyntaxKind::ToKeyword {
                        let high = self.eval_case_value(spec[index + 2])?;
                        matched = self.in_range(selector, &value, &high)?;
                        index += 2;
                    }
                    if matched {
                        return Ok(true);
                    }
                    index += 1;
                }
            }
        }
        Ok(false)
    }

    /// Evaluate a case value, which may be a bare literal token or an
    /// expression node.
    fn eval_case_value(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        if is_literal_kind(node.kind()) {
            self.eval_literal(node)
        } else {
            self.eval_expr(node)
        }
    }

    /// `low <= selector <= high` (numeric or string range).
    fn in_range(
        &mut self,
        selector: &VBVariant,
        low: &VBVariant,
        high: &VBVariant,
    ) -> RunResult<bool> {
        let s = selector.as_f64()?;
        let lo = low.as_f64()?;
        let hi = high.as_f64()?;
        Ok(s >= lo && s <= hi)
    }

    /// Apply a comparison operator between the selector and a case value.
    fn apply_compare(
        &mut self,
        selector: &VBVariant,
        op: SyntaxKind,
        value: &VBVariant,
    ) -> RunResult<bool> {
        let a = selector.as_f64()?;
        let b = value.as_f64()?;
        Ok(match op {
            SyntaxKind::EqualityOperator => a == b,
            SyntaxKind::InequalityOperator => a != b,
            SyntaxKind::LessThanOperator => a < b,
            SyntaxKind::LessThanOrEqualOperator => a <= b,
            SyntaxKind::GreaterThanOperator => a > b,
            SyntaxKind::GreaterThanOrEqualOperator => a >= b,
            _ => return Err(self.error_here(VBError::invalid_procedure_call())),
        })
    }

    /// `Exit Sub|Function|For|Do|While`.
    pub(crate) fn exec_exit(&mut self, node: &CstNode) -> RunResult<Flow> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let target = significant
            .iter()
            .find(|c| {
                matches!(
                    c.kind(),
                    SyntaxKind::SubKeyword
                        | SyntaxKind::FunctionKeyword
                        | SyntaxKind::ForKeyword
                        | SyntaxKind::DoKeyword
                        | SyntaxKind::WhileKeyword
                )
            })
            .map(|c| c.kind());
        Ok(match target {
            Some(SyntaxKind::ForKeyword | SyntaxKind::DoKeyword | SyntaxKind::WhileKeyword) => {
                Flow::BreakLoop
            }
            Some(SyntaxKind::SubKeyword | SyntaxKind::FunctionKeyword) => Flow::Return,
            _ => Flow::Next,
        })
    }
}
