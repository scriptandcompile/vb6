//! Random VB6 source code generator from an ANTLR4 grammar.
//!
//! Walks the grammar rules starting from a given start rule and makes random
//! choices at each alternative / quantifier to produce syntactically plausible
//! VB6 source text.
//!
//! Lexer rules (uppercase names) are handled specially: well-known lexer tokens
//! are emitted as sensible literal text rather than being expanded character by
//! character from their regex definitions, which would produce garbled output.

use std::collections::HashMap;

use rand::Rng;
use rand::RngExt;

use crate::g4_parser::{Alternative, CharRange, Element, Grammar, Rule};

/// Configuration for the generator.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Maximum recursion depth before forcing base-case choices.
    pub max_depth: usize,
    /// Maximum repetitions for `+` and `*` quantifiers.
    pub max_repeat: usize,
    /// Probability (0.0–1.0) of including an optional element (`?`).
    pub optional_prob: f64,
    /// Probability of producing a `*` repetition (per iteration).
    pub repeat_continue_prob: f64,
    /// Maximum output size in bytes. Generation stops expanding when reached.
    pub max_output_bytes: usize,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            max_depth: 6,
            max_repeat: 3,
            optional_prob: 0.5,
            repeat_continue_prob: 0.3,
            max_output_bytes: 512,
        }
    }
}

/// Build a table of fixed text overrides for well-known VB6 lexer tokens.
///
/// Instead of expanding e.g. `IDENTIFIER : LETTER LETTERORDIGIT*` down to
/// individual characters from large Unicode ranges, we emit sensible values.
fn build_lexer_overrides() -> HashMap<&'static str, Vec<&'static str>> {
    let mut m = HashMap::new();

    // Whitespace and structure
    m.insert("WS", vec![" "]);
    m.insert("NEWLINE", vec!["\r\n"]);
    m.insert("LINE_CONTINUATION", vec![" _\r\n"]);

    // Identifiers — pool of plausible VB6 names
    m.insert(
        "IDENTIFIER",
        vec![
            "x", "y", "z", "i", "j", "n", "s", "tmp", "val", "result", "count", "item", "obj",
            "frm", "ctl", "buf", "idx", "flag", "msg", "path", "MyVar", "MyFunc", "Counter",
            "Total", "Value1", "Item2",
        ],
    );

    // Literals
    m.insert(
        "STRINGLITERAL",
        vec![
            "\"hello\"",
            "\"\"",
            "\"test\"",
            "\"abc\"",
            "\"123\"",
            "\"Hello World\"",
        ],
    );
    m.insert(
        "INTEGERLITERAL",
        vec!["0", "1", "2", "5", "10", "42", "100", "255", "1000", "-1"],
    );
    m.insert(
        "DOUBLELITERAL",
        vec!["0.0", "1.0", "3.14", "2.5", "-1.5", "100.0"],
    );
    m.insert(
        "DATELITERAL",
        vec!["#1/1/2000#", "#12/31/1999#", "#6/15/2025#"],
    );
    m.insert("COLORLITERAL", vec!["&HFF", "&H0", "&HFF0000", "&HFFFFFF"]);
    m.insert("OCTALLITERAL", vec!["&O7", "&O77", "&O177"]);
    m.insert("FILENUMBER", vec!["#1", "#2", "#3"]);
    m.insert("GUID", vec!["{00000000-0000-0000-0000-000000000000}"]);
    m.insert("FRX_OFFSET", vec![":0000"]);

    // Symbols
    m.insert("AMPERSAND", vec!["&"]);
    m.insert("ASSIGN", vec![":="]);
    m.insert("AT", vec!["@"]);
    m.insert("COLON", vec![":"]);
    m.insert("COMMA", vec![","]);
    m.insert("DIV", vec!["/"]);
    m.insert("DOLLAR", vec!["$"]);
    m.insert("DOT", vec!["."]);
    m.insert("EQ", vec!["="]);
    m.insert("EXCLAMATIONMARK", vec!["!"]);
    m.insert("GEQ", vec![">="]);
    m.insert("GT", vec![">"]);
    m.insert("HASH", vec!["#"]);
    m.insert("LEQ", vec!["<="]);
    m.insert("LBRACE", vec!["{"]);
    m.insert("LPAREN", vec!["("]);
    m.insert("LT", vec!["<"]);
    m.insert("MINUS", vec!["-"]);
    m.insert("MINUS_EQ", vec!["-="]);
    m.insert("MULT", vec!["*"]);
    m.insert("NEQ", vec!["<>"]);
    m.insert("PERCENT", vec!["%"]);
    m.insert("PLUS", vec!["+"]);
    m.insert("PLUS_EQ", vec!["+="]);
    m.insert("POW", vec!["^"]);
    m.insert("RBRACE", vec!["}"]);
    m.insert("RPAREN", vec![")"]);
    m.insert("SEMICOLON", vec![";"]);
    m.insert("L_SQUARE_BRACKET", vec!["["]);
    m.insert("R_SQUARE_BRACKET", vec!["]"]);

    m
}

/// Generates random source text from a grammar.
pub struct Generator<'g, R> {
    grammar: &'g Grammar,
    config: GeneratorConfig,
    rng: R,
    depth: usize,
    output: String,
    lexer_overrides: HashMap<&'static str, Vec<&'static str>>,
}

impl<'g, R: Rng> Generator<'g, R> {
    pub fn new(grammar: &'g Grammar, config: GeneratorConfig, rng: R) -> Self {
        Self {
            grammar,
            config,
            rng,
            depth: 0,
            output: String::new(),
            lexer_overrides: build_lexer_overrides(),
        }
    }

    /// Generate source text starting from the given rule name.
    pub fn generate(mut self, start_rule: &str) -> String {
        self.expand_rule(start_rule);
        self.output
    }

    fn at_limit(&self) -> bool {
        self.output.len() >= self.config.max_output_bytes
    }

    fn expand_rule(&mut self, name: &str) {
        if self.at_limit() {
            return;
        }

        // Check for a lexer override first.
        if let Some(options) = self.lexer_overrides.get(name) {
            let idx = self.rng.random_range(0..options.len());
            self.output.push_str(options[idx]);
            return;
        }

        let Some(rule) = self.grammar.rules.get(name) else {
            return;
        };

        // Skip hidden-channel rules.
        if rule.is_hidden {
            return;
        }

        // For lexer keyword rules, try to emit the keyword text directly
        // rather than expanding case-insensitive fragment rules char-by-char.
        if rule.is_lexer_rule() && !rule.is_fragment {
            if let Some(text) = try_emit_keyword(rule, &self.grammar.rules) {
                self.output.push_str(&text);
                return;
            }
        }

        self.depth += 1;
        let alt = self.choose_alternative(rule);
        let alt = alt.clone();
        self.expand_alternative(&alt);
        self.depth -= 1;
    }

    fn choose_alternative<'r>(&mut self, rule: &'r Rule) -> &'r Alternative {
        let alts = &rule.alternatives;
        if alts.len() == 1 {
            return &alts[0];
        }

        if self.depth >= self.config.max_depth {
            return alts
                .iter()
                .min_by_key(|a| a.elements.len())
                .unwrap_or(&alts[0]);
        }

        let idx = self.rng.random_range(0..alts.len());
        &alts[idx]
    }

    fn expand_alternative(&mut self, alt: &Alternative) {
        for elem in &alt.elements {
            self.expand_element(elem);
        }
    }

    fn expand_element(&mut self, elem: &Element) {
        if self.at_limit() {
            return;
        }

        match elem {
            Element::RuleRef(name) => self.expand_rule(name),
            Element::StringLiteral(s) => self.output.push_str(s),
            Element::CharClass { ranges, negated } => {
                self.emit_char_from_class(ranges, *negated);
            }
            Element::Range(start, end) => {
                let s = *start as u32;
                let e = *end as u32;
                if s <= e {
                    let c = self.rng.random_range(s..=e);
                    if let Some(ch) = char::from_u32(c) {
                        self.output.push(ch);
                    }
                }
            }
            Element::Group(alts) => {
                if alts.is_empty() {
                    return;
                }
                let idx = if self.depth >= self.config.max_depth {
                    alts.iter()
                        .enumerate()
                        .min_by_key(|(_, a)| a.elements.len())
                        .map(|(i, _)| i)
                        .unwrap_or(0)
                } else {
                    self.rng.random_range(0..alts.len())
                };
                let alt = alts[idx].clone();
                self.expand_alternative(&alt);
            }
            Element::Optional(inner) => {
                let p: f64 = self.rng.random();
                if self.depth < self.config.max_depth && p < self.config.optional_prob {
                    self.expand_element(inner);
                }
            }
            Element::ZeroOrMore(inner) => {
                if self.depth < self.config.max_depth {
                    let mut count = 0;
                    while count < self.config.max_repeat {
                        let p: f64 = self.rng.random();
                        if p >= self.config.repeat_continue_prob {
                            break;
                        }
                        self.expand_element(inner);
                        count += 1;
                    }
                }
            }
            Element::OneOrMore(inner) => {
                self.expand_element(inner);
                if self.depth < self.config.max_depth {
                    let mut count = 1;
                    while count < self.config.max_repeat {
                        let p: f64 = self.rng.random();
                        if p >= self.config.repeat_continue_prob {
                            break;
                        }
                        self.expand_element(inner);
                        count += 1;
                    }
                }
            }
            Element::Not(_inner) => {
                self.output.push('x');
            }
            Element::Wildcard => {
                let c = self.rng.random_range(0x20u8..0x7Fu8) as char;
                self.output.push(c);
            }
            Element::Eof => {}
        }
    }

    fn emit_char_from_class(&mut self, ranges: &[CharRange], negated: bool) {
        if negated {
            for _ in 0..100 {
                let c = self.rng.random_range(0x20u8..0x7Fu8) as char;
                if !char_in_ranges(c, ranges) {
                    self.output.push(c);
                    return;
                }
            }
            self.output.push('x');
        } else if ranges.is_empty() {
            self.output.push('a');
        } else {
            let chars = collect_chars_from_ranges(ranges);
            if chars.is_empty() {
                self.output.push('a');
            } else {
                let idx = self.rng.random_range(0..chars.len());
                self.output.push(chars[idx]);
            }
        }
    }
}

/// If a lexer rule is a simple keyword (built from case-insensitive letter
/// fragments and string literals), return its lowercase text.
fn try_emit_keyword(rule: &Rule, all_rules: &HashMap<String, Rule>) -> Option<String> {
    if rule.alternatives.is_empty() {
        return None;
    }

    let alt = &rule.alternatives[0];
    let mut text = String::new();

    for elem in &alt.elements {
        match elem {
            Element::StringLiteral(s) => text.push_str(s),
            Element::RuleRef(frag_name) => {
                if let Some(frag) = all_rules.get(frag_name) {
                    if frag.is_fragment {
                        if let Some(Element::StringLiteral(ch)) =
                            frag.alternatives.first().and_then(|a| a.elements.first())
                        {
                            text.push_str(ch);
                            continue;
                        }
                    }
                }
                return None;
            }
            _ => return None,
        }
    }

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn char_in_ranges(c: char, ranges: &[CharRange]) -> bool {
    for r in ranges {
        match r {
            CharRange::Single(ch) => {
                if c == *ch {
                    return true;
                }
            }
            CharRange::Range(start, end) => {
                if c >= *start && c <= *end {
                    return true;
                }
            }
        }
    }
    false
}

fn collect_chars_from_ranges(ranges: &[CharRange]) -> Vec<char> {
    let mut chars = Vec::new();
    for r in ranges {
        match r {
            CharRange::Single(c) => chars.push(*c),
            CharRange::Range(start, end) => {
                let s = *start as u32;
                let e = *end as u32;
                let limit = s + 256;
                let e = e.min(limit);
                for code in s..=e {
                    if let Some(c) = char::from_u32(code) {
                        chars.push(c);
                    }
                }
            }
        }
    }
    chars
}
