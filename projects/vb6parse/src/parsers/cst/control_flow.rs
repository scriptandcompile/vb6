/// Maximum depth for nested control flow statements
pub const MAX_STATEMENT_DEPTH: usize = 500;

/// Simple enum to identify the type of control flow frame.
/// This avoids using magic numbers (i32) for frame type identification.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ControlFlowFrameType {
    /// `StatementList` frame type
    StatementList,
    /// `IfStatement` frame type
    IfStatement,
    /// `ForStatement` frame type
    ForStatement,
    /// `SelectCase` frame type
    SelectCase,
    /// `WhileStatement` frame type
    WhileStatement,
    /// `DoStatement` frame type
    DoStatement,
    /// `WithStatement` frame type
    WithStatement,
}

/// Parsing state for control flow statements
///
/// This enum represents the state machine frames for parsing control flow
/// statements without mutual recursion. Each frame type corresponds to a
/// control flow construct and tracks its parsing progress through phases.
#[derive(Debug, Clone)]
pub enum ControlFlowFrame {
    /// Parsing a statement list
    StatementList {
        /// Nesting depth
        depth: usize,
        /// Context for determining when to stop parsing
        context: StatementListContext,
        /// Whether `start_node` has been called for this frame
        started: bool,
    },

    /// Parsing an If statement
    IfStatement {
        /// Current parsing phase
        phase: IfPhase,
        /// Nesting depth
        depth: usize,
    },

    /// Parsing a For loop
    ForStatement {
        /// Current parsing phase
        phase: ForPhase,
        /// Whether this is a For Each loop
        is_for_each: bool,
        /// Nesting depth
        depth: usize,
    },

    /// Parsing a Select Case
    SelectCase {
        /// Current parsing phase
        phase: SelectPhase,
        /// Nesting depth
        depth: usize,
    },

    /// Parsing a While loop
    WhileStatement {
        /// Current parsing phase
        phase: WhilePhase,
        /// Nesting depth
        depth: usize,
    },

    /// Parsing a Do loop
    DoStatement {
        /// Current parsing phase
        phase: DoPhase,
        /// Nesting depth
        depth: usize,
    },

    /// Parsing a With block
    WithStatement {
        /// Current parsing phase
        phase: WithPhase,
        /// Nesting depth
        depth: usize,
    },
}

/// Context for statement list parsing to determine stop conditions
#[derive(Debug, Copy, Clone)]
pub enum StatementListContext {
    /// Top-level statement list (stops at end of input)
    TopLevel,
    /// `If/Then` body (stops at `ElseIf`, `Else`, or `End If`)
    IfThenBody,
    /// `ElseIf` body (stops at `ElseIf`, `Else`, or `End If`)  
    ElseIfBody,
    /// `Else` body (stops at `End If`)
    ElseBody,
    /// `For` loop body (stops at `Next`)
    ForBody,
    /// `Select Case` body (stops at `Case`, `Case Else`, or `End Select`)
    SelectCaseBody,
    /// `While` loop body (stops at `Wend`)
    WhileBody,
    /// `Do` loop body (stops at `Loop`)
    DoBody,
    /// `With` block body (stops at `End With`)
    WithBody,
}

/// Phases for parsing an If statement
#[derive(Debug, Copy, Clone)]
pub enum IfPhase {
    /// Start parsing the If statement (parse condition and Then keyword)
    Start,
    /// Parse the Then body (statement list pushed separately)
    ThenBody,
    /// Check for and parse `ElseIf` condition
    CheckElseIf,
    /// Parse `ElseIf` body (statement list pushed separately)
    ElseIfBody,
    /// Check for and parse `Else`
    CheckElse,
    /// Parse `Else` body (statement list pushed separately)
    ElseBody,
    /// Finish the `If` statement (parse `End If`)
    Finish,
}

/// Phases for parsing a `For` loop
#[derive(Debug, Copy, Clone)]
pub enum ForPhase {
    /// Start parsing (parse `For` variable = start `To` end [`Step` step])
    Start,
    /// Parse loop body (statement list pushed separately)
    Body,
    /// Finish the loop (parse `Next`)
    Finish,
}

/// Phases for parsing a `Select Case` statement
#[derive(Debug, Copy, Clone)]
pub enum SelectPhase {
    /// Start parsing (parse `Select Case` expression)
    Start,
    /// Parse `Case` clause
    CaseClause,
    /// Parse `Case` body (statement list pushed separately)
    CaseBody,
    /// Check for more `Case` clauses or `Case Else`
    CheckNextCase,
    /// Parse `Case Else` body (statement list pushed separately)
    CaseElseBody,
    /// Finish (parse `End Select`)
    Finish,
}

/// Phases for parsing a `While` loop
#[derive(Debug, Copy, Clone)]
pub enum WhilePhase {
    /// Start parsing (parse `While` condition)
    Start,
    /// Parse loop body (statement list pushed separately)
    Body,
    /// Finish the loop (parse `Wend`)
    Finish,
}

/// Phases for parsing a `Do` loop
#[derive(Debug, Copy, Clone)]
pub enum DoPhase {
    /// Start parsing (parse `Do` [While/Until condition])
    Start,
    /// Parse loop body (statement list pushed separately)
    Body,
    /// Finish the loop (parse `Loop` [While/Until condition])
    Finish,
}

/// Phases for parsing a `With` block
#[derive(Debug, Copy, Clone)]
pub enum WithPhase {
    /// Start parsing (parse `With` expression)
    Start,
    /// Parse `With` body (statement list pushed separately)
    Body,
    /// Finish the block (parse `End With`)
    Finish,
}

impl ControlFlowFrame {
    /// Get the type identifier for this frame.
    /// This allows matching on frame types without borrowing the entire frame.
    pub(crate) fn frame_type(&self) -> ControlFlowFrameType {
        match self {
            ControlFlowFrame::StatementList { .. } => ControlFlowFrameType::StatementList,
            ControlFlowFrame::IfStatement { .. } => ControlFlowFrameType::IfStatement,
            ControlFlowFrame::ForStatement { .. } => ControlFlowFrameType::ForStatement,
            ControlFlowFrame::SelectCase { .. } => ControlFlowFrameType::SelectCase,
            ControlFlowFrame::WhileStatement { .. } => ControlFlowFrameType::WhileStatement,
            ControlFlowFrame::DoStatement { .. } => ControlFlowFrameType::DoStatement,
            ControlFlowFrame::WithStatement { .. } => ControlFlowFrameType::WithStatement,
        }
    }
}
