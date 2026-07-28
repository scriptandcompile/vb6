//! Delta-debugging reducer for minimizing inputs that trigger Error nodes.
//!
//! Implements the classic ddmin algorithm: given source text that produces
//! Error nodes, repeatedly try removing chunks until no further reduction
//! is possible while the Error nodes remain.

use crate::checker::check_source;

/// Minimize `source` so that it still triggers at least one Error node,
/// but is as small as possible.
///
/// Returns `None` if the original source doesn't trigger Error nodes.
pub fn reduce(source: &str) -> Option<String> {
    // Verify the original triggers the bug.
    if !check_source(source).has_error {
        return None;
    }

    let mut current = source.to_string();

    // Phase 1: line-level reduction.
    current = reduce_lines(&current);

    // Phase 2: character-level reduction (token-boundary-aware).
    current = reduce_chars(&current);

    Some(current)
}

/// Try removing contiguous groups of lines using ddmin.
fn reduce_lines(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    if lines.len() <= 1 {
        return source.to_string();
    }

    let mut current: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    let mut granularity = 2usize;

    loop {
        let n = current.len();
        if granularity > n {
            break;
        }

        let chunk_size = n / granularity;
        if chunk_size == 0 {
            break;
        }

        let mut progress = false;

        // Try removing each chunk.
        let mut i = 0;
        while i < granularity {
            let start = i * chunk_size;
            let end = if i == granularity - 1 {
                n
            } else {
                start + chunk_size
            };

            let candidate: Vec<String> = current[..start]
                .iter()
                .chain(current[end..].iter())
                .cloned()
                .collect();

            let candidate_str = candidate.join("\n");

            if check_source(&candidate_str).has_error {
                current = candidate;
                progress = true;
                // Don't increment i – we removed a chunk so indices shifted.
                // But we do need to recompute granularity parameters.
                break;
            }

            i += 1;
        }

        if progress {
            // Restart with same granularity on the now-smaller input.
            granularity = 2;
            continue;
        }

        granularity *= 2;
        if granularity > current.len() {
            break;
        }
    }

    // Also try removing individual lines one at a time, repeatedly.
    let mut changed = true;
    while changed {
        changed = false;
        let n = current.len();
        let mut i = 0;
        while i < n && current.len() > 1 {
            let mut candidate = current.clone();
            candidate.remove(i);
            let candidate_str = candidate.join("\n");
            if check_source(&candidate_str).has_error {
                current = candidate;
                changed = true;
                // Don't increment i.
            } else {
                i += 1;
            }
            // Re-check length since we might have removed elements.
            if i >= current.len() {
                break;
            }
        }
    }

    current.join("\n")
}

/// Try removing individual characters.
fn reduce_chars(source: &str) -> String {
    let mut chars: Vec<char> = source.chars().collect();

    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i < chars.len() {
            let mut candidate = chars.clone();
            candidate.remove(i);
            let candidate_str: String = candidate.iter().collect();
            if check_source(&candidate_str).has_error {
                chars = candidate;
                changed = true;
                // Don't increment i.
            } else {
                i += 1;
            }
        }
    }

    chars.into_iter().collect()
}
