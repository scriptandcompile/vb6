//! Check generated VB6 source for Error nodes in the CST.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use vb6parse::parsers::cst::{ConcreteSyntaxTree, CstNode};
use vb6parse::SyntaxKind;

/// Details about an Error node found in the CST.
#[derive(Debug, Clone)]
pub struct ErrorDetail {
    /// The text content of the Error node.
    pub text: String,
    /// A breadcrumb path of ancestor kinds leading to this node.
    pub path: Vec<SyntaxKind>,
}

/// Result of checking a source string for Error nodes.
#[derive(Debug)]
pub struct CheckResult {
    /// Whether any Error nodes were found.
    pub has_error: bool,
    /// Details of each Error node found.
    pub errors: Vec<ErrorDetail>,
    /// Parse failures reported by vb6parse.
    pub failure_count: usize,
    /// Whether the parse itself succeeded (produced a CST).
    pub parse_succeeded: bool,
    /// Whether the check timed out.
    pub timed_out: bool,
}

/// Maximum time to spend parsing a single source.
const PARSE_TIMEOUT: Duration = Duration::from_secs(2);

/// Parse the source with vb6parse and check for Error nodes.
/// Times out after `PARSE_TIMEOUT` to handle pathological inputs.
pub fn check_source(source: &str) -> CheckResult {
    let source = source.to_string();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = check_source_inner(&source);
        let _ = tx.send(result);
    });

    match rx.recv_timeout(PARSE_TIMEOUT) {
        Ok(result) => result,
        Err(_) => CheckResult {
            has_error: false,
            errors: vec![],
            failure_count: 0,
            parse_succeeded: false,
            timed_out: true,
        },
    }
}

fn check_source_inner(source: &str) -> CheckResult {
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("generated.bas", source).unpack();

    let Some(cst) = cst_opt else {
        return CheckResult {
            has_error: false,
            errors: vec![],
            failure_count: failures.len(),
            parse_succeeded: false,
            timed_out: false,
        };
    };

    let serializable = cst.to_serializable();
    let root = &serializable.root;

    let mut errors = Vec::new();
    let mut path = Vec::new();
    find_errors(root, &mut path, &mut errors);

    CheckResult {
        has_error: !errors.is_empty(),
        errors,
        failure_count: failures.len(),
        parse_succeeded: true,
        timed_out: false,
    }
}

fn find_errors(node: &CstNode, path: &mut Vec<SyntaxKind>, errors: &mut Vec<ErrorDetail>) {
    if node.kind().is_error_recovery() {
        errors.push(ErrorDetail {
            text: node.text().to_string(),
            path: path.clone(),
        });
    }

    path.push(node.kind());
    for child in node.children() {
        find_errors(child, path, errors);
    }
    path.pop();
}
