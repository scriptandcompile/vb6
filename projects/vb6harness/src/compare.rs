//! Output comparison: golden lines vs. engine output lines.
//!
//! Comparison is per line. Lines are whitespace-trimmed (real VB6 pads numbers
//! when writing with `Print #`), and lines that parse as numbers are compared
//! within a tolerance so equivalent floating-point spellings match.

/// A single mismatching line.
#[derive(Debug, Clone)]
pub struct Diff {
    /// 1-based line number in the output.
    pub line: usize,
    pub expected: String,
    pub actual: String,
}

/// Compares golden output against engine output.
pub struct Comparer {
    tolerance: f64,
}

impl Comparer {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Return the differences between `expected` (golden) and `actual`
    /// (engine) output lines.
    pub fn compare(&self, expected: &[String], actual: &[String]) -> Vec<Diff> {
        let line_count = expected.len().max(actual.len());
        let mut diffs = Vec::new();
        for index in 0..line_count {
            let expected_line = expected.get(index).map(String::as_str).unwrap_or("");
            let actual_line = actual.get(index).map(String::as_str).unwrap_or("");
            if !self.lines_match(expected_line, actual_line) {
                diffs.push(Diff {
                    line: index + 1,
                    expected: expected_line.to_string(),
                    actual: actual_line.to_string(),
                });
            }
        }
        diffs
    }

    fn lines_match(&self, expected: &str, actual: &str) -> bool {
        let expected = expected.trim();
        let actual = actual.trim();
        if expected == actual {
            return true;
        }
        match (expected.parse::<f64>(), actual.parse::<f64>()) {
            (Ok(expected), Ok(actual)) => (expected - actual).abs() <= self.tolerance,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn trims_whitespace_and_padding() {
        let comparer = Comparer::new(1e-12);
        let diffs = comparer.compare(&lines(&["3", "hi"]), &lines(&[" 3 ", "hi"]));
        assert!(diffs.is_empty());
    }

    #[test]
    fn matches_equivalent_number_spellings() {
        let comparer = Comparer::new(1e-12);
        let diffs = comparer.compare(&lines(&["0.30000000000000004"]), &lines(&["0.3"]));
        assert!(diffs.is_empty());
    }

    #[test]
    fn detects_ordering_and_content_mismatches() {
        let comparer = Comparer::new(1e-12);
        let diffs = comparer.compare(&lines(&["1", "2", "3"]), &lines(&["1", "2"]));
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].line, 3);
        let diffs = comparer.compare(&lines(&["abc"]), &lines(&["abd"]));
        assert_eq!(diffs.len(), 1);
    }

    #[test]
    fn rejects_real_numeric_differences() {
        let comparer = Comparer::new(1e-12);
        let diffs = comparer.compare(&lines(&["5"]), &lines(&["6"]));
        assert_eq!(diffs.len(), 1);
    }
}
