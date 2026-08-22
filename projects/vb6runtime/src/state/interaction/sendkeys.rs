//! `SendKeys` request model shared by every interaction backend.
//!
//! The raw VB6 keystroke string arrives in VB6's compact notation —
//! `+`/`^`/`%` prefixes for Shift/Ctrl/Alt, `{ENTER}`-style brace names,
//! `{RIGHT 5}` repeats — which backends cannot reasonably each re-implement,
//! so [`SendKeysRequest::parse`] decodes it once into a flat list of
//! [`Keystroke`]s (one physical key press plus its held modifiers).
//! Unrecognized names, unbalanced braces or groups, and bad repeat counts
//! raise VB6 error 5 ("Invalid procedure call or argument"), matching the
//! trappable error real VB6 raises for malformed key strings.

use std::fmt;

use crate::error::{err_number, VBError, VBResult};

/// One key of a `SendKeys` sequence after decoding: either a character to
/// type or a named non-printing key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendKey {
    /// Type this character (already resolved through VB6's notation; the
    /// backend maps it onto the platform keyboard).
    Char(char),
    /// `{BACKSPACE}`, `{BS}`, `{BKSP}`.
    Backspace,
    /// `{BREAK}`.
    Break,
    /// `{CAPSLOCK}`.
    CapsLock,
    /// `{DELETE}`, `{DEL}`.
    Delete,
    /// `{DOWN}`.
    Down,
    /// `{END}`.
    End,
    /// `{ENTER}` and `~`.
    Enter,
    /// `{ESC}`, `{ESCAPE}`.
    Esc,
    /// `{HELP}`.
    Help,
    /// `{HOME}`.
    Home,
    /// `{INSERT}`, `{INS}`.
    Insert,
    /// `{LEFT}`.
    Left,
    /// `{NUMLOCK}`.
    NumLock,
    /// `{PGDN}`.
    PageDown,
    /// `{PGUP}`.
    PageUp,
    /// `{PRTSC}`.
    PrintScreen,
    /// `{RIGHT}`.
    Right,
    /// `{SCROLLLOCK}`.
    ScrollLock,
    /// `{TAB}`.
    Tab,
    /// `{UP}`.
    Up,
    /// `{F1}` through `{F16}`.
    Function(u8),
}

impl SendKey {
    /// Decode a brace name (case-insensitive, VB6's table).
    ///
    /// Single characters reserved by the notation (`{+}`, `{%}`, `{~}`,
    /// `{{}`, `{}}`, ...) decode to their literal [`SendKey::Char`] form.
    /// Anything else is an invalid key name.
    fn from_brace_name(name: &str) -> VBResult<Self> {
        let upper = name.to_ascii_uppercase();
        let key = match upper.as_str() {
            "BACKSPACE" | "BS" | "BKSP" => Self::Backspace,
            "BREAK" => Self::Break,
            "CAPSLOCK" => Self::CapsLock,
            "DELETE" | "DEL" => Self::Delete,
            "DOWN" => Self::Down,
            "END" => Self::End,
            "ENTER" => Self::Enter,
            "ESC" | "ESCAPE" => Self::Esc,
            "HELP" => Self::Help,
            "HOME" => Self::Home,
            "INSERT" | "INS" => Self::Insert,
            "LEFT" => Self::Left,
            "NUMLOCK" => Self::NumLock,
            "PGDN" => Self::PageDown,
            "PGUP" => Self::PageUp,
            "PRTSC" => Self::PrintScreen,
            "RIGHT" => Self::Right,
            "SCROLLLOCK" => Self::ScrollLock,
            "TAB" => Self::Tab,
            "UP" => Self::Up,
            other if other.len() >= 2 && other.starts_with('F') => {
                let number: u8 = other[1..]
                    .parse()
                    .map_err(|_| invalid_keys(format!("unknown key name {{{name}}}")))?;
                if !(1..=16).contains(&number) {
                    return Err(invalid_keys(format!(
                        "function key {{{name}}} is out of range (F1-F16)"
                    )));
                }
                Self::Function(number)
            }
            // Escaped metacharacters: {+} {%} {^} {~} {{} {}.
            other if other.chars().count() == 1 => {
                Self::Char(other.chars().next().expect("non-empty"))
            }
            _ => return Err(invalid_keys(format!("unknown key name {{{name}}}"))),
        };
        Ok(key)
    }

    /// Short human-readable name for diagnostics.
    pub fn name(self) -> String {
        match self {
            Self::Char(c) => c.to_string(),
            Self::Function(n) => format!("F{n}"),
            Self::Backspace => "BACKSPACE".into(),
            Self::Break => "BREAK".into(),
            Self::CapsLock => "CAPSLOCK".into(),
            Self::Delete => "DELETE".into(),
            Self::Down => "DOWN".into(),
            Self::End => "END".into(),
            Self::Enter => "ENTER".into(),
            Self::Esc => "ESC".into(),
            Self::Help => "HELP".into(),
            Self::Home => "HOME".into(),
            Self::Insert => "INSERT".into(),
            Self::Left => "LEFT".into(),
            Self::NumLock => "NUMLOCK".into(),
            Self::PageDown => "PGDN".into(),
            Self::PageUp => "PGUP".into(),
            Self::PrintScreen => "PRTSC".into(),
            Self::Right => "RIGHT".into(),
            Self::ScrollLock => "SCROLLLOCK".into(),
            Self::Tab => "TAB".into(),
            Self::Up => "UP".into(),
        }
    }
}

/// One decoded keystroke: a key plus the modifiers held down while pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keystroke {
    /// The key being pressed.
    pub key: SendKey,
    /// Whether Shift is held (`+` prefix).
    pub shift: bool,
    /// Whether Ctrl is held (`^` prefix).
    pub ctrl: bool,
    /// Whether Alt is held (`%` prefix).
    pub alt: bool,
}

impl Keystroke {
    /// An unmodified keystroke of `key`.
    pub fn new(key: SendKey) -> Self {
        Self {
            key,
            shift: false,
            ctrl: false,
            alt: false,
        }
    }
}

/// Modifier state accumulated while decoding a `SendKeys` string.
#[derive(Debug, Clone, Copy, Default)]
struct Modifiers {
    shift: bool,
    ctrl: bool,
    alt: bool,
}

impl Modifiers {
    /// Apply these modifiers to `key`.
    fn apply(self, key: SendKey) -> Keystroke {
        Keystroke {
            key,
            shift: self.shift,
            ctrl: self.ctrl,
            alt: self.alt,
        }
    }

    /// Turn on the modifier named by `marker` (`+`, `^`, `%`).
    fn enable(mut self, marker: char) -> Self {
        match marker {
            '+' => self.shift = true,
            '^' => self.ctrl = true,
            '%' => self.alt = true,
            _ => unreachable!("only + ^ % are modifiers"),
        }
        self
    }
}

/// A fully decoded `SendKeys` invocation.
///
/// Backends receive this instead of the raw string: [`strokes`](Self::strokes)
/// holds the expanded keystroke sequence ready for synthesis, while
/// [`keys`](Self::keys) preserves the original text for logging and hosts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendKeysRequest {
    /// The original keystroke string, verbatim.
    pub keys: String,
    /// Whether to wait until the keystrokes have been processed before
    /// continuing (`wait` argument). Platforms whose delivery is already
    /// synchronous treat this as a hint.
    pub wait: bool,
    /// The decoded keystrokes, in delivery order (repeats already expanded).
    pub strokes: Vec<Keystroke>,
}

impl SendKeysRequest {
    /// Decode a VB6 `SendKeys` string into a request for the active window.
    ///
    /// Malformed input — unknown `{names}`, unbalanced braces or modifier
    /// groups, repeat counts below 1 — fails with VB6 error 5.
    pub fn parse(keys: impl Into<String>, wait: bool) -> VBResult<Self> {
        let keys = keys.into();
        let strokes = parse_keys(&keys)?;
        Ok(Self {
            keys,
            wait,
            strokes,
        })
    }
}

/// Decode the full keystroke string.
fn parse_keys(keys: &str) -> VBResult<Vec<Keystroke>> {
    let mut strokes = Vec::new();
    let mut chars = keys.chars().peekable();
    parse_items(&mut strokes, &mut chars, Modifiers::default(), false)?;
    Ok(strokes)
}

/// Decode items until end-of-string or, inside a modifier group, the closing
/// parenthesis (which the caller consumes).
///
/// `in_group` distinguishes a `)` closing a `+(...)` group from a literal
/// parenthesis a program wants typed; bare parentheses outside groups are
/// ordinary characters.
fn parse_items(
    strokes: &mut Vec<Keystroke>,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    modifiers: Modifiers,
    in_group: bool,
) -> VBResult<()> {
    while let Some(&ch) = chars.peek() {
        match ch {
            ')' if in_group => return Ok(()),
            '+' | '^' | '%' => {
                chars.next();
                let inner = modifiers.enable(ch);
                match chars.peek() {
                    // Group: every item inside carries this modifier.
                    Some('(') => {
                        chars.next();
                        parse_items(strokes, chars, inner, true)?;
                        match chars.next() {
                            Some(')') => {}
                            _ => {
                                return Err(invalid_keys(
                                    "modifier group opened with \"(\" was never closed",
                                ))
                            }
                        }
                    }
                    // Single item: only the next keystroke is modified.
                    Some(_) => parse_single_item(strokes, chars, inner)?,
                    // Nothing follows; type the marker itself (WSH parity).
                    None => strokes.push(modifiers.apply(SendKey::Char(ch))),
                }
            }
            '{' => {
                chars.next();
                parse_brace_group(strokes, chars, modifiers)?;
            }
            '~' => {
                chars.next();
                strokes.push(modifiers.apply(SendKey::Enter));
            }
            other => {
                chars.next();
                strokes.push(modifiers.apply(SendKey::Char(other)));
            }
        }
    }
    Ok(())
}

/// Decode exactly one item — a brace group or a single character — applying
/// `modifiers` to every keystroke it produces.
///
/// Used after a `+`/`^`/`%` marker so that, say, `+{RIGHT 3}` shifts all
/// three repeats while `+ab` shifts only the `a`. Markers stack (`%+x` is
/// Alt+Shift+x); a marker with nothing left to modify types itself.
fn parse_single_item(
    strokes: &mut Vec<Keystroke>,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    modifiers: Modifiers,
) -> VBResult<()> {
    match chars.peek().copied() {
        Some('{') => {
            chars.next();
            parse_brace_group(strokes, chars, modifiers)
        }
        Some('~') => {
            chars.next();
            strokes.push(modifiers.apply(SendKey::Enter));
            Ok(())
        }
        Some(marker @ ('+' | '^' | '%')) => {
            // Stacked modifier: fold it in and keep looking for the key it
            // belongs to.
            chars.next();
            let inner = modifiers.enable(marker);
            match chars.peek().copied() {
                Some('(') => {
                    chars.next();
                    parse_items(strokes, chars, inner, true)?;
                    match chars.next() {
                        Some(')') => Ok(()),
                        _ => Err(invalid_keys(
                            "modifier group opened with \"(\" was never closed",
                        )),
                    }
                }
                Some(_) => parse_single_item(strokes, chars, inner),
                None => {
                    strokes.push(modifiers.apply(SendKey::Char(marker)));
                    Ok(())
                }
            }
        }
        Some(ch) => {
            chars.next();
            strokes.push(modifiers.apply(SendKey::Char(ch)));
            Ok(())
        }
        None => Ok(()),
    }
}

/// Decode a `{...}` group (the `{` already consumed) and append its
/// keystrokes, honoring the optional `{name count}` repeat suffix.
fn parse_brace_group(
    strokes: &mut Vec<Keystroke>,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    modifiers: Modifiers,
) -> VBResult<()> {
    let mut content = String::new();
    loop {
        match chars.next() {
            Some('}') => break,
            Some(ch) => content.push(ch),
            None => return Err(invalid_keys("braces must balance: missing \"}\"")),
        }
    }
    if content.is_empty() {
        // `{}}` is VB6's two-character escape for a literal `}`: the first
        // `}` closed an empty group, and the second one supplies the key.
        if chars.peek() == Some(&'}') {
            chars.next();
            strokes.push(modifiers.apply(SendKey::Char('}')));
            return Ok(());
        }
        return Err(invalid_keys("empty braces {} name no key"));
    }

    // `{name count}` form: a trailing whitespace-separated integer asks for
    // repetition. A final token that is not a number is part of the name.
    let (name, repeat) = match content.rsplit_once(char::is_whitespace) {
        Some((name, count)) => match count.trim().parse::<u32>() {
            Ok(count) => (name, count),
            Err(_) => (content.as_str(), 1),
        },
        None => (content.as_str(), 1),
    };
    if !(1..=65_535).contains(&repeat) {
        return Err(invalid_keys(format!(
            "repeat count for {{{name} {repeat}}} is out of range"
        )));
    }

    let key = SendKey::from_brace_name(name)?;
    for _ in 0..repeat {
        strokes.push(modifiers.apply(key));
    }
    Ok(())
}

/// Build the error 5 raised for malformed keystroke strings.
fn invalid_keys(detail: impl fmt::Display) -> VBError {
    VBError::with_description(
        err_number::INVALID_PROCEDURE_CALL,
        format!("Invalid procedure call or argument: invalid SendKeys string: {detail}"),
    )
}

/// One recorded `SendKeys` invocation — what was requested, kept so hosts
/// and tests can assert on the keystrokes a program attempted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendKeysRecord {
    /// The requested keystroke string, verbatim.
    pub keys: String,
    /// The requested `wait` flag.
    pub wait: bool,
}

impl SendKeysRecord {
    /// Capture the relevant parts of `request`.
    pub fn of(request: &SendKeysRequest) -> Self {
        Self {
            keys: request.keys.clone(),
            wait: request.wait,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(keys: &str) -> Vec<Keystroke> {
        SendKeysRequest::parse(keys, false).unwrap().strokes
    }

    fn plain(key: SendKey) -> Keystroke {
        Keystroke::new(key)
    }

    #[test]
    fn plain_characters_are_typed_verbatim() {
        assert_eq!(
            parse("Hello, World"),
            "Hello, World"
                .chars()
                .map(|c| plain(SendKey::Char(c)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn tilde_is_enter() {
        assert_eq!(parse("~"), vec![plain(SendKey::Enter)]);
        assert_eq!(
            parse("a~b"),
            vec![
                plain(SendKey::Char('a')),
                plain(SendKey::Enter),
                plain(SendKey::Char('b')),
            ]
        );
    }

    #[test]
    fn brace_names_decode_case_insensitively() {
        assert_eq!(parse("{enter}"), vec![plain(SendKey::Enter)]);
        assert_eq!(parse("{ENTER}"), vec![plain(SendKey::Enter)]);
        assert_eq!(parse("{Del}"), vec![plain(SendKey::Delete)]);
        assert_eq!(parse("{INS}"), vec![plain(SendKey::Insert)]);
        assert_eq!(parse("{bksp}"), vec![plain(SendKey::Backspace)]);
        assert_eq!(parse("{escape}"), vec![plain(SendKey::Esc)]);
    }

    #[test]
    fn function_keys_decode() {
        assert_eq!(parse("{F1}"), vec![plain(SendKey::Function(1))]);
        assert_eq!(parse("{f16}"), vec![plain(SendKey::Function(16))]);
    }

    #[test]
    fn function_key_out_of_range_is_error_5() {
        assert_eq!(
            SendKeysRequest::parse("{F17}", false).unwrap_err().number,
            5
        );
        assert_eq!(SendKeysRequest::parse("{F0}", false).unwrap_err().number, 5);
    }

    #[test]
    fn modifiers_apply_to_the_next_single_item_only() {
        assert_eq!(
            parse("^c"),
            vec![Keystroke {
                key: SendKey::Char('c'),
                shift: false,
                ctrl: true,
                alt: false,
            }]
        );
        // Only the `a` is shifted; `bc` follows unmodified.
        assert_eq!(
            parse("+abc"),
            vec![
                Keystroke {
                    key: SendKey::Char('a'),
                    shift: true,
                    ctrl: false,
                    alt: false
                },
                plain(SendKey::Char('b')),
                plain(SendKey::Char('c')),
            ]
        );
    }

    #[test]
    fn modifier_groups_apply_to_every_member() {
        assert_eq!(
            parse("^(ec)"),
            vec![
                Keystroke {
                    key: SendKey::Char('e'),
                    shift: false,
                    ctrl: true,
                    alt: false
                },
                Keystroke {
                    key: SendKey::Char('c'),
                    shift: false,
                    ctrl: true,
                    alt: false
                },
            ]
        );
        assert_eq!(
            parse("%(FA)"),
            vec![
                Keystroke {
                    key: SendKey::Char('F'),
                    shift: false,
                    ctrl: false,
                    alt: true
                },
                Keystroke {
                    key: SendKey::Char('A'),
                    shift: false,
                    ctrl: false,
                    alt: true
                },
            ]
        );
    }

    #[test]
    fn modifiers_combine_and_stack() {
        // %+x = Alt and Shift both held for x; ^%{DEL} stacks two markers.
        assert_eq!(
            parse("%+x"),
            vec![Keystroke {
                key: SendKey::Char('x'),
                shift: true,
                ctrl: false,
                alt: true,
            }]
        );
        assert_eq!(
            parse("^%{DELETE}"),
            vec![Keystroke {
                key: SendKey::Delete,
                shift: false,
                ctrl: true,
                alt: true,
            }]
        );
    }

    #[test]
    fn modifiers_apply_to_modified_brace_groups() {
        assert_eq!(
            parse("+{F1}"),
            vec![Keystroke {
                key: SendKey::Function(1),
                shift: true,
                ctrl: false,
                alt: false,
            }]
        );
        // A modified group may contain further braces and tildes.
        assert_eq!(
            parse("%~"),
            vec![Keystroke {
                key: SendKey::Enter,
                shift: false,
                ctrl: false,
                alt: true,
            }]
        );
    }

    #[test]
    fn modifier_groups_may_nest() {
        // Inner group members inherit both markers.
        assert_eq!(
            parse("^(%(a))"),
            vec![Keystroke {
                key: SendKey::Char('a'),
                shift: false,
                ctrl: true,
                alt: true,
            }]
        );
    }

    #[test]
    fn repeats_expand_in_place() {
        assert_eq!(parse("{RIGHT 10}"), vec![plain(SendKey::Right); 10]);
        assert_eq!(
            parse("{TAB 5}a"),
            vec![
                plain(SendKey::Tab),
                plain(SendKey::Tab),
                plain(SendKey::Tab),
                plain(SendKey::Tab),
                plain(SendKey::Tab),
                plain(SendKey::Char('a')),
            ]
        );
    }

    #[test]
    fn modified_repeats_apply_to_each_copy() {
        assert_eq!(
            parse("+{RIGHT 3}"),
            vec![
                Keystroke {
                    key: SendKey::Right,
                    shift: true,
                    ctrl: false,
                    alt: false
                };
                3
            ]
        );
    }

    #[test]
    fn escaped_metacharacters_type_themselves() {
        assert_eq!(parse("{+}"), vec![plain(SendKey::Char('+'))]);
        assert_eq!(parse("{^}"), vec![plain(SendKey::Char('^'))]);
        assert_eq!(parse("{%}"), vec![plain(SendKey::Char('%'))]);
        assert_eq!(parse("{~}"), vec![plain(SendKey::Char('~'))]);
        assert_eq!(parse("{{}"), vec![plain(SendKey::Char('{'))]);
        assert_eq!(parse("{}}"), vec![plain(SendKey::Char('}'))]);
    }

    #[test]
    fn bare_parentheses_are_literal_outside_modifier_groups() {
        assert_eq!(
            parse("f(x)"),
            vec![
                plain(SendKey::Char('f')),
                plain(SendKey::Char('(')),
                plain(SendKey::Char('x')),
                plain(SendKey::Char(')')),
            ]
        );
    }

    #[test]
    fn stray_trailing_modifier_types_itself() {
        assert_eq!(
            parse("a+"),
            vec![plain(SendKey::Char('a')), plain(SendKey::Char('+')),]
        );
    }

    #[test]
    fn empty_string_sends_nothing() {
        assert_eq!(parse(""), Vec::new());
    }

    #[test]
    fn unknown_names_are_error_5() {
        for bad in ["{FOO}", "{}", "{ 5 }"] {
            let err = SendKeysRequest::parse(bad, false).unwrap_err();
            assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL, "{bad}");
        }
    }

    #[test]
    fn unbalanced_braces_are_error_5() {
        let err = SendKeysRequest::parse("{ENTER", false).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(err.description.contains("balance"), "{}", err.description);
    }

    #[test]
    fn unclosed_modifier_group_is_error_5() {
        let err = SendKeysRequest::parse("^(ab", false).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(err.description.contains("\"(\""), "{}", err.description);
    }

    #[test]
    fn zero_repeat_count_is_error_5() {
        let err = SendKeysRequest::parse("{TAB 0}", false).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn oversized_repeat_count_is_error_5() {
        assert_eq!(
            SendKeysRequest::parse("{TAB 65536}", false)
                .unwrap_err()
                .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn non_numeric_trailing_token_stays_part_of_the_name() {
        // "{NOPE 12x}" is a single (unknown) name, not a repeat.
        let err = SendKeysRequest::parse("{NOPE 12x}", false).unwrap_err();
        assert!(err.description.contains("NOPE 12x"), "{}", err.description);
    }

    #[test]
    fn wait_flag_is_preserved() {
        assert!(!SendKeysRequest::parse("hi", false).unwrap().wait);
        assert!(SendKeysRequest::parse("hi", true).unwrap().wait);
    }

    #[test]
    fn original_string_is_preserved_verbatim() {
        let request = SendKeysRequest::parse("Username{TAB}Password{ENTER}", true).unwrap();
        assert_eq!(request.keys, "Username{TAB}Password{ENTER}");
    }

    #[test]
    fn record_captures_relevant_parts() {
        let request = SendKeysRequest::parse("hi{TAB}", true).unwrap();
        let record = SendKeysRecord::of(&request);
        assert_eq!(record.keys, "hi{TAB}");
        assert!(record.wait);
    }

    #[test]
    fn key_names_round_trip_for_diagnostics() {
        assert_eq!(SendKey::Enter.name(), "ENTER");
        assert_eq!(SendKey::Function(12).name(), "F12");
        assert_eq!(SendKey::Char('x').name(), "x");
    }
}
