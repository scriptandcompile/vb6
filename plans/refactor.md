# Statement List Parser Refactor Plan

## Problem Statement

The test `chess_brain_vb_eval_module_load` fails with a stack overflow when parsing `Eval.bas`, a real-world VB6 file from the ChessBrainVB project. 

### Stack Overflow Root Cause

**File:** `vb6parse/tests/data/ChessBrainVB/ChessBrainVB_V4_03a/Source/Modules/Eval.bas`
- **Total lines:** 3,687
- **Maximum control flow nesting depth:** 285 levels (in the `Eval()` function)
- **Control flow statements:** 809 If statements, multiple For/Select statements

The parser fails due to **mutual recursion** between control flow statement parsers and `parse_statement_list()`:

```
parse_if_statement() 
  → parse_statement_list() [line 110 in if_statements.rs]
    → parse_if_statement() [line 2204 in mod.rs]
      → parse_statement_list()
        → parse_if_statement()
          → ... (285 recursive calls)
            → STACK OVERFLOW
```

### Why Current Implementation Fails

The comment in [if_statements.rs:108](vb6parse/src/parsers/cst/if_statements.rs#L108-L117) is **incorrect**:

```rust
// Parse If body - the recursive call here is now safe because
// parse_statement_list handles control flow iteratively
self.parse_statement_list(|parser| { ... });
```

While `parse_statement_list()` itself doesn't directly recurse (it uses a while loop), it **calls control flow statement parsers** (`parse_if_statement`, `parse_for_statement`, etc.) which **call back to `parse_statement_list()`**, creating mutual recursion.

**Current call chain:**
1. `parse_statement_list()` is iterative (uses `while !self.is_at_end()`)
2. Within the loop, it calls `self.parse_if_statement()` 
3. `parse_if_statement()` calls `self.parse_statement_list()` for the If body
4. This creates a function call stack frame for each nesting level
5. At depth 285, the stack overflows (typical stack size ~8MB)

### Similar Issues in Other Parsers

The same mutual recursion pattern exists in:
- [for_statements.rs](vb6parse/src/parsers/cst/for_statements.rs) - `parse_for_statement()` calls `parse_statement_list()`
- [loop_statements.rs](vb6parse/src/parsers/cst/loop_statements.rs) - `parse_do_statement()`, `parse_while_statement()` 
- [select_statements.rs](vb6parse/src/parsers/cst/select_statements.rs) - `parse_select_case_statement()`
- [with_statements.rs](vb6parse/src/parsers/cst/with_statements.rs) - `parse_with_statement()`

## Solution: Eliminate Mutual Recursion with State Machine

### Strategy Overview

Replace the mutual recursion pattern with a **state machine** that uses an explicit stack to track parsing state, with **modular handler functions** for each control flow type. This completely eliminates function call recursion while maintaining clean code organization.

**Key insight:** Instead of relying on the function call stack, maintain our own stack of parsing states that can be processed iteratively. Each control flow type has its own handler function that processes state transitions without making recursive calls.

### Design Approach

#### 1. Add State Tracking to Parser

```rust
// In src/parsers/cst/mod.rs

/// Maximum depth for nested control flow statements
const MAX_STATEMENT_DEPTH: usize = 500;

/// Type alias for stop condition closures
type StopConditionFn = Box<dyn Fn(&Parser) -> bool>;

/// Parsing state for control flow statements
enum ControlFlowFrame {
    /// Parsing a statement list
    StatementList {
        stop_condition: StopConditionFn,
        depth: usize,
    },
    
    /// Parsing an If statement
    IfStatement {
        phase: IfPhase,
        depth: usize,
    },
    
    /// Parsing a For loop
    ForStatement {
        phase: ForPhase,
        is_for_each: bool,
        depth: usize,
    },
    
    /// Parsing a Select Case
    SelectCase {
        phase: SelectPhase,
        depth: usize,
    },
    
    /// Parsing a While loop
    WhileStatement {
        phase: WhilePhase,
        depth: usize,
    },
    
    /// Parsing a Do loop
    DoStatement {
        phase: DoPhase,
        depth: usize,
    },
    
    /// Parsing a With block
    WithStatement {
        phase: WithPhase,
        depth: usize,
    },
}

/// Phases for parsing an If statement
enum IfPhase {
    /// Start parsing the If statement (parse condition and Then keyword)
    Start,
    /// Parse the Then body (statement list pushed separately)
    ThenBody,
    /// Check for and parse ElseIf condition
    CheckElseIf,
    /// Parse ElseIf body (statement list pushed separately)
    ElseIfBody,
    /// Check for and parse Else
    CheckElse,
    /// Parse Else body (statement list pushed separately)
    ElseBody,
    /// Finish the If statement (parse End If)
    Finish,
}

/// Phases for parsing a For loop
enum ForPhase {
    /// Start parsing (parse For variable = start To end [Step step])
    Start,
    /// Parse loop body (statement list pushed separately)
    Body,
    /// Finish the loop (parse Next)
    Finish,
}

/// Phases for parsing a Select Case
enum SelectPhase {
    /// Start parsing (parse Select Case expression)
    Start,
    /// Parse Case clause
    CaseClause,
    /// Parse Case body (statement list pushed separately)
    CaseBody,
    /// Check for more Case clauses or Case Else
    CheckNextCase,
    /// Parse Case Else
    CaseElse,
    /// Parse Case Else body (statement list pushed separately)
    CaseElseBody,
    /// Finish (parse End Select)
    Finish,
}

/// Phases for parsing a While loop
enum WhilePhase {
    /// Start parsing (parse While condition)
    Start,
    /// Parse loop body (statement list pushed separately)
    Body,
    /// Finish the loop (parse Wend)
    Finish,
}

/// Phases for parsing a Do loop
enum DoPhase {
    /// Start parsing (parse Do [While/Until condition])
    Start,
    /// Parse loop body (statement list pushed separately)
    Body,
    /// Finish the loop (parse Loop [While/Until condition])
    Finish,
}

/// Phases for parsing a With block
enum WithPhase {
    /// Start parsing (parse With expression)
    Start,
    /// Parse With body (statement list pushed separately)
    Body,
    /// Finish the block (parse End With)
    Finish,
}
```

#### 2. Create Modular Handler Functions

Create separate handler functions for each control flow type. These functions process state transitions without making recursive calls:

```rust
impl<'a> Parser<'a> {
    /// Handle If statement state transitions
    /// Returns true if processing should continue, false if frame should be popped
    fn handle_if_statement_frame(
        &mut self,
        phase: &mut IfPhase,
        depth: usize,
        frame_stack: &mut Vec<ControlFlowFrame>,
    ) -> bool {
        match phase {
            IfPhase::Start => {
                // Parse If condition and Then keyword
                self.builder.start_node(SyntaxKind::IfStatement.to_raw());
                self.consume_whitespace();
                self.consume_token(); // If
                self.consume_whitespace();
                self.parse_expression();
                self.consume_whitespace();
                
                if self.at_token(Token::ThenKeyword) {
                    self.consume_token();
                }
                self.consume_whitespace();
                
                // Check if single-line or multi-line
                let is_single_line = !self.at_token(Token::Newline);
                
                if is_single_line {
                    // Single-line: parse inline without frame stack
                    self.parse_single_line_if_body();
                    self.builder.finish_node(); // IfStatement
                    return false; // Pop this frame
                } else {
                    // Multi-line: transition to ThenBody phase
                    if self.at_token(Token::Newline) {
                        self.consume_token();
                    }
                    
                    *phase = IfPhase::ThenBody;
                    
                    // Push statement list frame for Then body
                    frame_stack.push(ControlFlowFrame::StatementList {
                        stop_condition: Box::new(|parser| {
                            parser.at_token(Token::ElseIfKeyword)
                                || parser.at_token(Token::ElseKeyword)
                                || (parser.at_token(Token::EndKeyword)
                                    && parser.peek_next_keyword() == Some(Token::IfKeyword))
                        }),
                        depth: depth + 1,
                    });
                    return true; // Continue processing
                }
            }
            
            IfPhase::ThenBody => {
                // Statement list for Then body just finished
                *phase = IfPhase::CheckElseIf;
                true
            }
            
            IfPhase::CheckElseIf => {
                // Check for ElseIf or move to next phase
                if self.at_token(Token::ElseIfKeyword) {
                    // Start ElseIf clause
                    self.builder.start_node(SyntaxKind::ElseIfClause.to_raw());
                    self.consume_token(); // ElseIf
                    self.consume_whitespace();
                    self.parse_expression();
                    self.consume_whitespace();
                    
                    if self.at_token(Token::ThenKeyword) {
                        self.consume_token();
                    }
                    self.consume_whitespace();
                    
                    if self.at_token(Token::Newline) {
                        self.consume_token();
                    }
                    
                    // Push statement list for ElseIf body
                    *phase = IfPhase::ElseIfBody;
                    frame_stack.push(ControlFlowFrame::StatementList {
                        stop_condition: Box::new(|parser| {
                            parser.at_token(Token::ElseIfKeyword)
                                || parser.at_token(Token::ElseKeyword)
                                || (parser.at_token(Token::EndKeyword)
                                    && parser.peek_next_keyword() == Some(Token::IfKeyword))
                        }),
                        depth: depth + 1,
                    });
                } else {
                    // No ElseIf, check for Else
                    *phase = IfPhase::CheckElse;
                }
                true
            }
            
            IfPhase::ElseIfBody => {
                // ElseIf body finished
                self.builder.finish_node(); // ElseIfClause
                *phase = IfPhase::CheckElseIf; // Check for more ElseIf
                true
            }
            
            IfPhase::CheckElse => {
                // Check for Else clause
                if self.at_token(Token::ElseKeyword) {
                    self.builder.start_node(SyntaxKind::ElseClause.to_raw());
                    self.consume_token(); // Else
                    self.consume_whitespace();
                    
                    if self.at_token(Token::Newline) {
                        self.consume_token();
                    }
                    
                    // Push statement list for Else body
                    *phase = IfPhase::ElseBody;
                    frame_stack.push(ControlFlowFrame::StatementList {
                        stop_condition: Box::new(|parser| {
                            parser.at_token(Token::EndKeyword)
                                && parser.peek_next_keyword() == Some(Token::IfKeyword)
                        }),
                        depth: depth + 1,
                    });
                } else {
                    // No Else, finish the If statement
                    *phase = IfPhase::Finish;
                }
                true
            }
            
            IfPhase::ElseBody => {
                // Else body finished
                self.builder.finish_node(); // ElseClause
                *phase = IfPhase::Finish;
                true
            }
            
            IfPhase::Finish => {
                // Consume "End If"
                if self.at_token(Token::EndKeyword) {
                    self.consume_token();
                    self.consume_whitespace();
                    self.consume_token(); // If
                    self.consume_until_after(Token::Newline);
                }
                
                self.builder.finish_node(); // IfStatement
                false // Pop this frame
            }
        }
    }
    
    /// Handle For statement state transitions
    fn handle_for_statement_frame(
        &mut self,
        phase: &mut ForPhase,
        is_for_each: bool,
        depth: usize,
        frame_stack: &mut Vec<ControlFlowFrame>,
    ) -> bool {
        match phase {
            ForPhase::Start => {
                // Parse For/For Each header
                self.builder.start_node(
                    if is_for_each {
                        SyntaxKind::ForEachStatement.to_raw()
                    } else {
                        SyntaxKind::ForStatement.to_raw()
                    }
                );
                
                self.consume_whitespace();
                self.consume_token(); // For
                self.consume_whitespace();
                
                if is_for_each {
                    if self.at_token(Token::EachKeyword) {
                        self.consume_token();
                    }
                    // Parse: variable In collection
                    // ... parsing logic ...
                } else {
                    // Parse: variable = start To end [Step step]
                    // ... parsing logic ...
                }
                
                self.consume_until_after(Token::Newline);
                
                // Push statement list for loop body
                *phase = ForPhase::Body;
                frame_stack.push(ControlFlowFrame::StatementList {
                    stop_condition: Box::new(|parser| {
                        parser.at_token(Token::NextKeyword)
                    }),
                    depth: depth + 1,
                });
                true
            }
            
            ForPhase::Body => {
                // Loop body finished
                *phase = ForPhase::Finish;
                true
            }
            
            ForPhase::Finish => {
                // Consume Next
                if self.at_token(Token::NextKeyword) {
                    self.consume_token();
                    // Optionally parse variable name after Next
                    self.consume_until_after(Token::Newline);
                }
                
                self.builder.finish_node(); // ForStatement or ForEachStatement
                false // Pop this frame
            }
        }
    }
    
    /// Handle Select Case statement state transitions
    fn handle_select_case_frame(
        &mut self,
        phase: &mut SelectPhase,
        depth: usize,
        frame_stack: &mut Vec<ControlFlowFrame>,
    ) -> bool {
        // Similar pattern: Start -> CaseClause -> CaseBody -> CheckNextCase -> ...
        // Implementation details omitted for brevity
        todo!("Implement Select Case handler")
    }
    
    /// Handle While statement state transitions
    fn handle_while_statement_frame(
        &mut self,
        phase: &mut WhilePhase,
        depth: usize,
        frame_stack: &mut Vec<ControlFlowFrame>,
    ) -> bool {
        // Start -> Body -> Finish
        todo!("Implement While handler")
    }
    
    /// Handle Do statement state transitions
    fn handle_do_statement_frame(
        &mut self,
        phase: &mut DoPhase,
        depth: usize,
        frame_stack: &mut Vec<ControlFlowFrame>,
    ) -> bool {
        // Start -> Body -> Finish
        todo!("Implement Do handler")
    }
    
    /// Handle With statement state transitions
    fn handle_with_statement_frame(
        &mut self,
        phase: &mut WithPhase,
        depth: usize,
        frame_stack: &mut Vec<ControlFlowFrame>,
    ) -> bool {
        // Start -> Body -> Finish
        todo!("Implement With handler")
    }
}
```

#### 3. Refactor parse_statement_list to Use State Machine

Transform `parse_statement_list()` to use the handler functions:

```rust
// NEW (fully iterative with modular handlers)
pub(crate) fn parse_statement_list<F>(&mut self, stop_conditions: F)
where F: Fn(&Parser) -> bool
{
    let mut frame_stack: Vec<ControlFlowFrame> = Vec::new();
    
    // Push initial statement list frame
    frame_stack.push(ControlFlowFrame::StatementList {
        stop_condition: Box::new(stop_conditions),
        depth: 0,
    });
    
    self.builder.start_node(SyntaxKind::StatementList.to_raw());
    
    while let Some(current_frame) = frame_stack.last_mut() {
        // Check depth limit
        if frame_stack.len() > MAX_STATEMENT_DEPTH {
            self.record_error(
                ParseError::NestingTooDeep {
                    depth: frame_stack.len(),
                    max_depth: MAX_STATEMENT_DEPTH,
                }
            );
            break;
        }
        
        match current_frame {
            ControlFlowFrame::StatementList { stop_condition, depth } => {
                if (stop_condition)(self) || self.is_at_end() {
                    frame_stack.pop();
                    self.builder.finish_node(); // StatementList
                    continue;
                }
                
                // Detect and start control flow statements
                if self.is_control_flow_keyword() {
                    match self.current_token() {
                        Some(Token::IfKeyword) => {
                            frame_stack.push(ControlFlowFrame::IfStatement {
                                phase: IfPhase::Start,
                                depth: *depth + 1,
                            });
                        }
                        Some(Token::ForKeyword) => {
                            // Check if For Each
                            let is_for_each = self.peek_next_keyword() == Some(Token::EachKeyword);
                            frame_stack.push(ControlFlowFrame::ForStatement {
                                phase: ForPhase::Start,
                                is_for_each,
                                depth: *depth + 1,
                            });
                        }
                        Some(Token::SelectKeyword) => {
                            frame_stack.push(ControlFlowFrame::SelectCase {
                                phase: SelectPhase::Start,
                                depth: *depth + 1,
                            });
                        }
                        Some(Token::WhileKeyword) => {
                            frame_stack.push(ControlFlowFrame::WhileStatement {
                                phase: WhilePhase::Start,
                                depth: *depth + 1,
                            });
                        }
                        Some(Token::DoKeyword) => {
                            frame_stack.push(ControlFlowFrame::DoStatement {
                                phase: DoPhase::Start,
                                depth: *depth + 1,
                            });
                        }
                        Some(Token::WithKeyword) => {
                            frame_stack.push(ControlFlowFrame::WithStatement {
                                phase: WithPhase::Start,
                                depth: *depth + 1,
                            });
                        }
                        _ => {
                            // Other control flow (GoTo, GoSub, etc.) - non-nesting
                            self.parse_control_flow_statement();
                        }
                    }
                    continue;
                }
                
                // Try built-in library statements
                if self.is_library_statement_keyword() {
                    self.parse_library_statement();
                    continue;
                }
                
                // Try array statements
                if self.is_variable_declaration_keyword() {
                    self.parse_array_statement();
                    continue;
                }
                
                // Try other statement types
                if self.is_statement_keyword() {
                    self.parse_statement();
                    continue;
                }
                
                // Handle other constructs
                match self.current_token() {
                    Some(
                        Token::Whitespace
                        | Token::Newline
                        | Token::EndOfLineComment
                        | Token::RemComment,
                    ) => {
                        self.consume_token();
                    }
                    _ => {
                        // Labels, assignments, procedure calls, etc.
                        if self.is_at_label() {
                            self.parse_label_statement();
                        } else if self.at_token(Token::LetKeyword) {
                            self.parse_let_statement();
                        } else if self.is_at_assignment() {
                            self.parse_assignment_statement();
                        } else if self.is_at_procedure_call() {
                            self.parse_procedure_call();
                        } else {
                            self.consume_token_as_unknown();
                        }
                    }
                }
            }
            
            ControlFlowFrame::IfStatement { phase, depth } => {
                let should_continue = self.handle_if_statement_frame(
                    phase,
                    *depth,
                    &mut frame_stack,
                );
                
                if !should_continue {
                    frame_stack.pop();
                }
            }
            
            ControlFlowFrame::ForStatement { phase, is_for_each, depth } => {
                let should_continue = self.handle_for_statement_frame(
                    phase,
                    *is_for_each,
                    *depth,
                    &mut frame_stack,
                );
                
                if !should_continue {
                    frame_stack.pop();
                }
            }
            
            ControlFlowFrame::SelectCase { phase, depth } => {
                let should_continue = self.handle_select_case_frame(
                    phase,
                    *depth,
                    &mut frame_stack,
                );
                
                if !should_continue {
                    frame_stack.pop();
                }
            }
            
            ControlFlowFrame::WhileStatement { phase, depth } => {
                let should_continue = self.handle_while_statement_frame(
                    phase,
                    *depth,
                    &mut frame_stack,
                );
                
                if !should_continue {
                    frame_stack.pop();
                }
            }
            
            ControlFlowFrame::DoStatement { phase, depth } => {
                let should_continue = self.handle_do_statement_frame(
                    phase,
                    *depth,
                    &mut frame_stack,
                );
                
                if !should_continue {
                    frame_stack.pop();
                }
            }
            
            ControlFlowFrame::WithStatement { phase, depth } => {
                let should_continue = self.handle_with_statement_frame(
                    phase,
                    *depth,
                    &mut frame_stack,
                );
                
                if !should_continue {
                    frame_stack.pop();
                }
            }
        }
    }
}
```

#### 4. Update Old Control Flow Parsers

The existing `parse_if_statement()`, `parse_for_statement()`, etc. should become thin wrappers:

```rust
pub(crate) fn parse_if_statement(&mut self) {
    // Delegate to the state machine
    self.parse_statement_list(|_| false); // Never stop - handled by If frame
    // Note: This simplified wrapper may need adjustment based on actual usage
}
```

Alternatively, mark them as deprecated and update all call sites to use the new approach.

#### 5. Add Depth Limit Error Handling

```rust
// In src/parsers/mod.rs or src/error.rs

#[derive(Debug, Clone)]
pub enum ParseError {
    // ... existing errors ...
    
    /// Control flow nesting exceeds maximum depth
    NestingTooDeep {
        depth: usize,
        max_depth: usize,
    },
}
```

When depth limit is exceeded:
1. Record the error in the failures list
2. Pop all frames and stop parsing the current statement list
3. Consume tokens until recovery point (Sub/Function/End of nested block)
4. Continue parsing to gather more errors

### Benefits of the Modular Approach

1. **Clean separation of concerns** - Each control flow type has its own handler function
2. **Easier to test** - Individual handlers can be unit tested
3. **Better maintainability** - Logic for each control flow type is isolated
4. **Easier to debug** - Stack traces point to specific handler functions
5. **Extensible** - New control flow types can be added by creating new frame variants and handlers
6. **No mutual recursion** - All handlers use the frame stack, never call back to `parse_statement_list()`

## Implementation Plan

### Phase 1: Infrastructure (1-2 days)

**Files to modify:**
- `vb6parse/src/parsers/cst/mod.rs`

**Tasks:**
- [ ] Define `ControlFlowFrame` enum with all control flow types
- [ ] Define phase enums: `IfPhase`, `ForPhase`, `SelectPhase`, `WhilePhase`, `DoPhase`, `WithPhase`
- [ ] Add `MAX_STATEMENT_DEPTH` constant (500)
- [ ] Add `ParseError::NestingTooDeep` variant
- [ ] Add skeleton handler functions (return `todo!()` for now)
- [ ] Update `parse_statement_list()` to create frame stack and dispatch to handlers

**Testing:**
- [ ] Ensure existing tests still compile (handlers will panic with todo!())
- [ ] Add unit test for depth limit error creation

### Phase 2: Implement If Statement Handler (2-3 days)

**Files to modify:**
- `vb6parse/src/parsers/cst/mod.rs` - `handle_if_statement_frame()`
- `vb6parse/src/parsers/cst/if_statements.rs` - mark as deprecated or update docs

**Tasks:**
- [ ] Implement `handle_if_statement_frame()` with all phases:
  - [ ] `IfPhase::Start` - parse condition and Then keyword
  - [ ] `IfPhase::ThenBody` - handle Then body completion
  - [ ] `IfPhase::CheckElseIf` - check for ElseIf clauses
  - [ ] `IfPhase::ElseIfBody` - handle ElseIf body completion
  - [ ] `IfPhase::CheckElse` - check for Else clause
  - [ ] `IfPhase::ElseBody` - handle Else body completion
  - [ ] `IfPhase::Finish` - consume End If and finish node
- [ ] Handle single-line If statements in Start phase
- [ ] Implement depth checking
- [ ] Add helper method `parse_single_line_if_body()` if needed
- [ ] Update detection logic in `parse_statement_list()` to push If frames

**Testing:**
- [ ] Run `tests/edge_cases/recursion_limits.rs::deeply_nested_if_statements`
- [ ] Run all existing If statement tests in `tests/cst/`
- [ ] Test single-line If statements
- [ ] Test multi-line If with ElseIf and Else
- [ ] Test depth limit: create test with 501 nested Ifs, verify error

### Phase 3: Implement For Loop Handler (2 days)

**Files to modify:**
- `vb6parse/src/parsers/cst/mod.rs` - `handle_for_statement_frame()`
- `vb6parse/src/parsers/cst/for_statements.rs`

**Tasks:**
- [ ] Implement `handle_for_statement_frame()` with phases:
  - [ ] `ForPhase::Start` - parse For...To or For Each header
  - [ ] `ForPhase::Body` - handle loop body completion
  - [ ] `ForPhase::Finish` - consume Next and finish node
- [ ] Handle both `For...To...Next` and `For Each...In...Next` variants
- [ ] Update detection logic to distinguish For vs For Each
- [ ] Update `parse_statement_list()` to push For frames

**Testing:**
- [ ] Run `tests/edge_cases/recursion_limits.rs::deeply_nested_for_loops`
- [ ] Run all For loop tests
- [ ] Test For Each loops
- [ ] Test nested For loops with different variable names

### Phase 4: Implement Select Case Handler (2 days)

**Files to modify:**
- `vb6parse/src/parsers/cst/mod.rs` - `handle_select_case_frame()`
- `vb6parse/src/parsers/cst/select_statements.rs`

**Tasks:**
- [ ] Implement `handle_select_case_frame()` with phases:
  - [ ] `SelectPhase::Start` - parse Select Case expression
  - [ ] `SelectPhase::CaseClause` - start parsing a Case clause
  - [ ] `SelectPhase::CaseBody` - handle Case body completion
  - [ ] `SelectPhase::CheckNextCase` - check for more Case clauses
  - [ ] `SelectPhase::CaseElse` - start Case Else clause
  - [ ] `SelectPhase::CaseElseBody` - handle Case Else body completion
  - [ ] `SelectPhase::Finish` - consume End Select and finish node
- [ ] Handle multiple Case expressions
- [ ] Handle Case Else
- [ ] Update `parse_statement_list()` to push Select frames

**Testing:**
- [ ] Run `tests/edge_cases/recursion_limits.rs::deeply_nested_select_case`
- [ ] Run all Select Case tests
- [ ] Test Select with multiple Case clauses
- [ ] Test Select with Case Else

### Phase 5: Implement Loop Handlers (1-2 days)

**Files to modify:**
- `vb6parse/src/parsers/cst/mod.rs` - `handle_while_statement_frame()`, `handle_do_statement_frame()`
- `vb6parse/src/parsers/cst/loop_statements.rs`

**Tasks:**
- [ ] Implement `handle_while_statement_frame()` with phases:
  - [ ] `WhilePhase::Start` - parse While condition
  - [ ] `WhilePhase::Body` - handle body completion
  - [ ] `WhilePhase::Finish` - consume Wend and finish node
- [ ] Implement `handle_do_statement_frame()` with phases:
  - [ ] `DoPhase::Start` - parse Do [While/Until condition]
  - [ ] `DoPhase::Body` - handle body completion
  - [ ] `DoPhase::Finish` - parse Loop [While/Until condition] and finish
- [ ] Handle both Do While/Until at start and Loop While/Until at end
- [ ] Update `parse_statement_list()` to push While and Do frames

**Testing:**
- [ ] Run all While loop tests
- [ ] Run all Do loop tests
- [ ] Test Do While, Do Until, Do...Loop While, Do...Loop Until variants

### Phase 6: Implement With Statement Handler (1 day)

**Files to modify:**
- `vb6parse/src/parsers/cst/mod.rs` - `handle_with_statement_frame()`
- `vb6parse/src/parsers/cst/with_statements.rs`

**Tasks:**
- [ ] Implement `handle_with_statement_frame()` with phases:
  - [ ] `WithPhase::Start` - parse With expression
  - [ ] `WithPhase::Body` - handle body completion
  - [ ] `WithPhase::Finish` - consume End With and finish node
- [ ] Update `parse_statement_list()` to push With frames

**Testing:**
- [ ] Run `tests/edge_cases/recursion_limits.rs::deeply_nested_with_blocks`
- [ ] Run all With statement tests
- [ ] Test nested With blocks

### Phase 7: Refactor Old Control Flow Parsers (1 day)

**Files to modify:**
- `vb6parse/src/parsers/cst/if_statements.rs`
- `vb6parse/src/parsers/cst/for_statements.rs`
- `vb6parse/src/parsers/cst/select_statements.rs`
- `vb6parse/src/parsers/cst/loop_statements.rs`
- `vb6parse/src/parsers/cst/with_statements.rs`

**Tasks:**
- [ ] Mark `parse_if_statement()` as deprecated or convert to wrapper
- [ ] Mark `parse_for_statement()` and `parse_for_each_statement()` as deprecated
- [ ] Mark `parse_select_case_statement()` as deprecated
- [ ] Mark `parse_while_statement()` and `parse_do_statement()` as deprecated
- [ ] Mark `parse_with_statement()` as deprecated
- [ ] Update any external call sites (if any exist outside `parse_statement_list()`)
- [ ] Add documentation explaining the new architecture

**Note:** If these functions are only called from `parse_statement_list()`, they can be removed entirely after verifying all tests pass.

### Phase 8: Integration Testing (1-2 days)

**Tasks:**
- [ ] Run full test suite: `cargo test --package vb6parse`
- [ ] **Test with Eval.bas:** `cargo test --package vb6parse --test module -- chess_brain_vb::chess_brain_vb_eval_module_load --exact`
- [ ] Run all recursion limit tests with increased depths (600, 1000)
- [ ] Test combined nesting scenarios (If inside For inside Select, etc.)
- [ ] Verify error messages are clear and helpful
- [ ] Performance testing: compare parse times before/after refactor
- [ ] Test edge cases: incomplete statements, syntax errors in deeply nested code

### Phase 9: Documentation & Cleanup (1 day)

**Tasks:**
- [ ] Update `recursion.md` to mark as resolved
- [ ] Remove incorrect comments about "safe recursion" in old parsers
- [ ] Document the state machine approach in `mod.rs` module docs
- [ ] Add inline comments explaining the frame stack pattern
- [ ] Update CHANGELOG.md with breaking changes (if any)
- [ ] Create example showing depth limit behavior
- [ ] Document in README if this is a notable improvement

**Total Estimate:** 12-16 days

## Alternative: Temporary Depth Limiting

If the full refactor is not immediately feasible, implement a **temporary depth limit** as a stopgap:

### Quick Fix Approach

```rust
// Add to Parser struct
pub(crate) struct Parser<'a> {
    // ... existing fields ...
    statement_depth: usize,
}

// Create RAII depth guard
struct DepthGuard<'a> {
    parser: &'a mut Parser<'a>,
}

impl<'a> DepthGuard<'a> {
    fn new(parser: &'a mut Parser<'a>) -> Option<Self> {
        const MAX_DEPTH: usize = 200; // Conservative limit
        
        if parser.statement_depth >= MAX_DEPTH {
            parser.record_error(ParseError::NestingTooDeep {
                depth: parser.statement_depth,
                max_depth: MAX_DEPTH,
            });
            return None;
        }
        
        parser.statement_depth += 1;
        Some(DepthGuard { parser })
    }
}

impl<'a> Drop for DepthGuard<'a> {
    fn drop(&mut self) {
        self.parser.statement_depth -= 1;
    }
}

// Use in control flow parsers
pub(crate) fn parse_if_statement(&mut self) {
    let _guard = match DepthGuard::new(self) {
        Some(g) => g,
        None => return, // Depth exceeded, bail out
    };
    
    // ... existing If parsing logic ...
}
```

**Effort:** 1 day  
**Tradeoff:** 
- ✅ Prevents stack overflow crashes
- ✅ Provides clear error messages
- ❌ Won't parse Eval.bas correctly (depth 285 exceeds limit of 200)
- ❌ Still has mutual recursion, just limited

## Recommendation

**Implement the modular state machine refactor.** 

**Rationale:**
1. **Real-world VB6 code has deep nesting** - The ChessBrainVB example shows that depth 285 exists in production code
2. **Temporary fix is insufficient** - A depth limit of 200-300 still carries crash risk and won't handle all real code
3. **Parser robustness** - The refactor eliminates an entire class of stack overflow bugs
4. **Maintainability** - Modular handler functions are easier to understand and modify than a monolithic state machine
5. **Testability** - Individual handlers can be unit tested in isolation
6. **Performance** - Eliminating function call overhead may improve parsing speed
7. **Future-proof** - Handles arbitrarily deep nesting (up to available heap memory)

The modular approach (using handler functions) is preferred over inlining all logic into `parse_statement_list()` because:
- **Smaller functions** - Each handler is ~50-150 lines instead of one 1000+ line function
- **Easier debugging** - Stack traces show which handler is executing
- **Better code organization** - Each control flow type is self-contained
- **Easier to review** - PRs can be reviewed per control flow type

The temporary depth limiting could be implemented first (1 day) to unblock testing while the full refactor is in progress.

## Success Criteria

✅ **The refactor is complete when:**

1. Test `chess_brain_vb_eval_module_load` passes (parses Eval.bas successfully)
2. All tests in `tests/edge_cases/recursion_limits.rs` pass
3. Full test suite passes: `cargo test --package vb6parse` 
4. No mutual recursion between control flow parsers and `parse_statement_list()`
5. Stack usage is constant regardless of nesting depth
6. Clear error message when depth exceeds `MAX_STATEMENT_DEPTH`
7. All handler functions are implemented and documented
8. Old control flow parser functions are deprecated or removed
9. Code review passes - handlers are clean and maintainable
10. Documentation is updated to reflect the new architecture

## Architecture Overview

### Before: Mutual Recursion (Current)

```
parse_statement_list()              [uses function call stack]
  └─> match token
        If => parse_if_statement()
                 └─> parse_statement_list()  [RECURSION!]
                       └─> match token
                             If => parse_if_statement()
                                      └─> ... (285 times) => STACK OVERFLOW
```

### After: State Machine with Handlers (Proposed)

```
parse_statement_list()              [uses explicit frame stack]
  └─> while let Some(frame)
        match frame
          StatementList => detect If, push IfStatement frame
          IfStatement => handle_if_statement_frame()
                           └─> match phase
                                 Start => push StatementList frame
                                 ThenBody => check for ElseIf
                                 CheckElseIf => push StatementList frame
                                 ... (no recursion, all stack based)
```

**Key differences:**
- Function calls replaced with frame stack operations
- Each handler processes one phase per iteration
- State transitions by modifying frame or pushing new frames
- Depth limited by frame stack size (heap-allocated, can grow to millions)

## References

- [recursion.md](recursion.md) - Original recursion analysis
- [if_statements.rs:108](src/parsers/cst/if_statements.rs#L108) - Incorrect "safe recursion" comment
- [mod.rs:2191](src/parsers/cst/mod.rs#L2191) - Current `parse_statement_list()` implementation
- [tests/edge_cases/recursion_limits.rs](tests/edge_cases/recursion_limits.rs) - Recursion limit test cases
- [Pratt Parser](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) - For reference on expression parsing (separate issue)

## Notes

- **Control parsing is already iterative** - Form/control parsing was previously refactored to use explicit stacks ([mod.rs:1548](src/parsers/cst/mod.rs#L1548)), so there's precedent for this approach
- **Property groups are iterative** - Property group parsing also uses explicit stacks, proving the pattern works well
- **Modular handlers vs inline** - We chose modular handler functions over inlining all logic into `parse_statement_list()` to maintain code readability and testability. Each handler is ~50-150 lines, making the codebase easier to navigate and modify.
- **Handler function signature** - All handlers return `bool` to indicate whether the frame should remain on stack (true) or be popped (false). This provides a consistent pattern across all control flow types.
- **Expression parsing is separate** - Expression recursion (Strategy 3 in recursion.md) is a separate issue and not addressed by this refactor
- **Backwards compatibility** - Old control flow parser functions can remain as thin wrappers for backwards compatibility, or be deprecated if they're only called internally
