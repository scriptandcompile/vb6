//! Pure VB6 `Like` pattern matching.
//!
//! Operates on plain strings only; no CST or
//! [`Interpreter`](crate::interpreter::Interpreter) required.

/// VB6 `Like` pattern match, case-insensitive (the interpreter's default
/// `Option Compare`). Supports `?`, `*`, `#`, and `[charlist]` / `[!charlist]`
/// classes with `a-z` ranges. A literal `[`, `?`, `*`, or `#` is matched by
/// enclosing it in brackets (e.g. `[[]`, `[?]`).
pub(crate) fn like_match(pattern: &str, text: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_matches_any_run_including_empty() {
        assert!(like_match("*", ""));
        assert!(like_match("*", "anything"));
        assert!(like_match("a*b", "ab"));
        assert!(like_match("a*b", "aXYZb"));
        assert!(!like_match("a*b", "ba"));
    }

    #[test]
    fn question_mark_matches_exactly_one_character() {
        assert!(like_match("?", "a"));
        assert!(!like_match("?", ""));
        assert!(like_match("a?c", "abc"));
        assert!(!like_match("a?c", "ac"));
    }

    #[test]
    fn hash_matches_only_ascii_digits() {
        assert!(like_match("###", "123"));
        assert!(!like_match("###", "12"));
        assert!(!like_match("#", "a"));
    }

    #[test]
    fn matching_is_case_insensitive() {
        assert!(like_match("HELLO", "hello"));
        assert!(like_match("[A-C]", "b"));
    }

    #[test]
    fn char_classes_support_ranges_and_negation() {
        assert!(like_match("[A-Za-z0-9]", "5"));
        assert!(like_match("[!abc]", "d"));
        assert!(!like_match("[!abc]", "a"));
        // The class closes at the FIRST `]`, so `[]]` is an empty class that
        // matches nothing (unlike classic VB6, which treats a leading `]`
        // as a literal member).
        assert!(!like_match("[]]", "]"));
    }

    #[test]
    fn unclosed_bracket_is_treated_as_a_literal() {
        assert!(like_match("[", "["));
        assert!(!like_match("[", "x"));
    }

    #[test]
    fn special_characters_are_literal_outside_classes() {
        assert!(!like_match("a?c", "?ac")); // `?` in text is not matched literally
        assert!(like_match("a.c", "a.c"));
        assert!(!like_match("a.c", "axc"));
    }

    #[test]
    fn patterns_requiring_backtracking_match() {
        // The memoized recursion must resolve overlapping `*` branches.
        assert!(like_match("*a*a*", "xaxa"));
        assert!(like_match("a*a*b", "abaXab"));
        assert!(!like_match("a*a*b", "abaXa"));
    }

    #[test]
    fn unicode_text_is_matched_char_by_char() {
        assert!(like_match("?é", "éé"));
        assert!(!like_match("?", "éé"));
    }
}
