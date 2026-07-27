//! Check generated VB6 source for Unknown tokens in the CST.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use vb6parse::parsers::cst::{ConcreteSyntaxTree, CstNode};
use vb6parse::SyntaxKind;

/// Details about an Unknown token found in the CST.
#[derive(Debug, Clone)]
pub struct UnknownDetail {
    /// The text content of the Unknown node.
    pub text: String,
    /// A breadcrumb path of ancestor kinds leading to this node.
    pub path: Vec<SyntaxKind>,
}

/// Result of checking a source string for Unknown tokens.
#[derive(Debug)]
pub struct CheckResult {
    /// Whether any Unknown tokens were found.
    pub has_unknown: bool,
    /// Details of each Unknown token found.
    pub unknowns: Vec<UnknownDetail>,
    /// Parse failures reported by vb6parse.
    pub failure_count: usize,
    /// Whether the parse itself succeeded (produced a CST).
    pub parse_succeeded: bool,
    /// Whether the check timed out.
    pub timed_out: bool,
}

/// Maximum time to spend parsing a single source.
const PARSE_TIMEOUT: Duration = Duration::from_secs(2);

/// Parse the source with vb6parse and check for Unknown tokens.
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
            has_unknown: false,
            unknowns: vec![],
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
            has_unknown: false,
            unknowns: vec![],
            failure_count: failures.len(),
            parse_succeeded: false,
            timed_out: false,
        };
    };

    let serializable = cst.to_serializable();
    let root = &serializable.root;

    let mut unknowns = Vec::new();
    let mut path = Vec::new();
    find_unknowns(root, &mut path, &mut unknowns);

    CheckResult {
        has_unknown: !unknowns.is_empty(),
        unknowns,
        failure_count: failures.len(),
        parse_succeeded: true,
        timed_out: false,
    }
}

fn find_unknowns(node: &CstNode, path: &mut Vec<SyntaxKind>, unknowns: &mut Vec<UnknownDetail>) {
    if node.kind() == SyntaxKind::Unknown {
        unknowns.push(UnknownDetail {
            text: node.text().to_string(),
            path: path.clone(),
        });
    }

    path.push(node.kind());
    for child in node.children() {
        find_unknowns(child, path, unknowns);
    }
    path.pop();
}
