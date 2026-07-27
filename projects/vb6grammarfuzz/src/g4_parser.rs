//! ANTLR4 .g4 grammar file parser.
//!
//! Parses an ANTLR4 grammar file into an internal representation that can
//! be used for random source generation.

use std::collections::HashMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Grammar IR
// ---------------------------------------------------------------------------

/// A parsed ANTLR4 grammar.
#[derive(Debug, Clone)]
pub struct Grammar {
    #[allow(dead_code)]
    pub name: String,
    pub rules: HashMap<String, Rule>,
    /// Rule names in definition order (for deterministic iteration).
    #[allow(dead_code)]
    pub rule_order: Vec<String>,
}

/// A single grammar rule (parser rule or lexer rule).
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub is_fragment: bool,
    /// Hidden-channel rule (COMMENT, LINE_CONTINUATION, etc.)
    pub is_hidden: bool,
    pub alternatives: Vec<Alternative>,
}

impl Rule {
    /// True if this is a lexer rule (name starts with uppercase).
    pub fn is_lexer_rule(&self) -> bool {
        self.name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
    }
}

/// One alternative inside a rule (separated by `|`).
#[derive(Debug, Clone)]
pub struct Alternative {
    pub elements: Vec<Element>,
    #[allow(dead_code)]
    pub label: Option<String>,
}

/// A single element inside an alternative.
#[derive(Debug, Clone)]
pub enum Element {
    /// Reference to another rule by name.
    RuleRef(String),
    /// A literal string, e.g. `'End'`.
    StringLiteral(String),
    /// A character class, e.g. `[a-zA-Z]`.
    CharClass {
        ranges: Vec<CharRange>,
        negated: bool,
    },
    /// A group of alternatives `( a | b )`.
    Group(Vec<Alternative>),
    /// Optional: `e?`
    Optional(Box<Element>),
    /// Zero-or-more: `e*`
    ZeroOrMore(Box<Element>),
    /// One-or-more: `e+`
    OneOrMore(Box<Element>),
    /// Negation: `~e` (complement set).
    Not(Box<Element>),
    /// Character range: `'a'..'z'`
    Range(char, char),
    /// Wildcard `.`
    Wildcard,
    /// End of file marker.
    Eof,
}

/// A range inside a character class.
#[derive(Debug, Clone)]
pub enum CharRange {
    Single(char),
    Range(char, char),
}

// ---------------------------------------------------------------------------
// Tokenizer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Ident(String),
    StringLit(String),
    CharClass { body: String, negated: bool },
    Colon,
    Semicolon,
    Pipe,
    LParen,
    RParen,
    Question,
    Star,
    Plus,
    Tilde,
    Dot,
    DotDot,
    Hash,
    Arrow,
    Eof,
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tok::Ident(s) => write!(f, "Ident({s})"),
            Tok::StringLit(s) => write!(f, "Str({s})"),
            Tok::CharClass { body, negated } => {
                write!(f, "CharClass(neg={negated}, {body})")
            }
            Tok::Colon => write!(f, ":"),
            Tok::Semicolon => write!(f, ";"),
            Tok::Pipe => write!(f, "|"),
            Tok::LParen => write!(f, "("),
            Tok::RParen => write!(f, ")"),
            Tok::Question => write!(f, "?"),
            Tok::Star => write!(f, "*"),
            Tok::Plus => write!(f, "+"),
            Tok::Tilde => write!(f, "~"),
            Tok::Dot => write!(f, "."),
            Tok::DotDot => write!(f, ".."),
            Tok::Hash => write!(f, "#"),
            Tok::Arrow => write!(f, "->"),
            Tok::Eof => write!(f, "EOF"),
        }
    }
}

struct Tokenizer {
    chars: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // skip whitespace
            while self.peek().is_some_and(|c| c.is_ascii_whitespace()) {
                self.advance();
            }
            // skip line comment
            if self.peek() == Some('/') && self.chars.get(self.pos + 1) == Some(&'/') {
                while self.peek().is_some_and(|c| c != '\n') {
                    self.advance();
                }
                continue;
            }
            // skip block comment
            if self.peek() == Some('/') && self.chars.get(self.pos + 1) == Some(&'*') {
                self.advance();
                self.advance();
                loop {
                    if self.peek().is_none() {
                        break;
                    }
                    if self.peek() == Some('*') && self.chars.get(self.pos + 1) == Some(&'/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn next_token(&mut self) -> Tok {
        self.skip_whitespace_and_comments();

        let Some(c) = self.peek() else {
            return Tok::Eof;
        };

        match c {
            ':' => {
                self.advance();
                Tok::Colon
            }
            ';' => {
                self.advance();
                Tok::Semicolon
            }
            '|' => {
                self.advance();
                Tok::Pipe
            }
            '(' => {
                self.advance();
                Tok::LParen
            }
            ')' => {
                self.advance();
                Tok::RParen
            }
            '?' => {
                self.advance();
                Tok::Question
            }
            '*' => {
                self.advance();
                Tok::Star
            }
            '+' => {
                self.advance();
                Tok::Plus
            }
            '~' => {
                self.advance();
                Tok::Tilde
            }
            '#' => {
                self.advance();
                Tok::Hash
            }
            '-' if self.chars.get(self.pos + 1) == Some(&'>') => {
                self.advance();
                self.advance();
                Tok::Arrow
            }
            '.' if self.chars.get(self.pos + 1) == Some(&'.') => {
                self.advance();
                self.advance();
                Tok::DotDot
            }
            '.' => {
                self.advance();
                Tok::Dot
            }
            '\'' => {
                self.advance(); // opening quote
                let mut s = String::new();
                loop {
                    match self.peek() {
                        None => break,
                        Some('\\') => {
                            // ANTLR4 escape: \' → literal quote, \\ → backslash, etc.
                            self.advance(); // consume backslash
                            if let Some(escaped) = self.advance() {
                                let ch = match escaped {
                                    'n' => '\n',
                                    'r' => '\r',
                                    't' => '\t',
                                    other => other, // \' → ', \\ → \, etc.
                                };
                                s.push(ch);
                            }
                        }
                        Some('\'') => {
                            self.advance(); // closing quote
                            break;
                        }
                        Some(_) => {
                            s.push(self.advance().unwrap());
                        }
                    }
                }
                Tok::StringLit(s)
            }
            '[' => {
                self.advance(); // '['
                let negated = self.peek() == Some('^');
                if negated {
                    self.advance();
                }
                let mut body = String::new();
                while self.peek().is_some_and(|ch| ch != ']') {
                    body.push(self.advance().unwrap());
                }
                self.advance(); // ']'
                Tok::CharClass { body, negated }
            }
            '{' => {
                // Skip action blocks { ... }
                self.advance();
                let mut depth = 1;
                while depth > 0 {
                    match self.advance() {
                        Some('{') => depth += 1,
                        Some('}') => depth -= 1,
                        None => break,
                        _ => {}
                    }
                }
                self.next_token()
            }
            _ if c.is_alphanumeric() || c == '_' => {
                let mut s = String::new();
                while self
                    .peek()
                    .is_some_and(|ch| ch.is_alphanumeric() || ch == '_')
                {
                    s.push(self.advance().unwrap());
                }
                Tok::Ident(s)
            }
            '$' => {
                self.advance();
                // '$' can appear as a literal character in some g4 grammars
                Tok::StringLit("$".to_string())
            }
            _ => {
                // Skip unknown characters
                self.advance();
                self.next_token()
            }
        }
    }

    fn tokenize_all(mut self) -> Vec<Tok> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Tok::Eof {
                tokens.push(Tok::Eof);
                break;
            }
            tokens.push(tok);
        }
        tokens
    }
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct G4Parser {
    tokens: Vec<Tok>,
    pos: usize,
}

impl G4Parser {
    fn new(tokens: Vec<Tok>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Tok {
        self.tokens.get(self.pos).unwrap_or(&Tok::Eof)
    }

    fn advance(&mut self) -> Tok {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Tok::Eof);
        self.pos += 1;
        tok
    }

    fn expect_ident(&mut self) -> String {
        match self.advance() {
            Tok::Ident(s) => s,
            other => panic!("Expected identifier, got {other}"),
        }
    }

    fn expect(&mut self, expected: &Tok) {
        let tok = self.advance();
        assert!(tok == *expected, "Expected {expected}, got {tok}");
    }

    fn parse_grammar(&mut self) -> Grammar {
        let mut name = String::new();
        let mut rules = HashMap::new();
        let mut rule_order = Vec::new();

        // Skip to 'grammar' keyword
        while let Tok::Ident(ref s) = *self.peek() {
            if s == "grammar" {
                self.advance();
                name = self.expect_ident();
                self.expect(&Tok::Semicolon);
                break;
            }
            // Skip other top-level things (options, tokens, channels blocks)
            self.advance();
        }

        // Parse rules
        loop {
            match self.peek().clone() {
                Tok::Eof => break,
                Tok::Ident(s) if s == "fragment" => {
                    self.advance();
                    let rule = self.parse_rule(true);
                    rule_order.push(rule.name.clone());
                    rules.insert(rule.name.clone(), rule);
                }
                Tok::Ident(s)
                    if s == "options" || s == "tokens" || s == "channels" || s == "mode" =>
                {
                    // Skip blocks: options { ... }, tokens { ... }, etc.
                    self.advance();
                    // There might be a { ...  } block to skip
                    // These are handled by the tokenizer already (it skips { })
                    // but we need to skip to the next semicolon or rule
                    if *self.peek() == Tok::Semicolon {
                        self.advance();
                    }
                }
                Tok::Ident(_) => {
                    let rule = self.parse_rule(false);
                    rule_order.push(rule.name.clone());
                    rules.insert(rule.name.clone(), rule);
                }
                _ => {
                    // Skip unexpected tokens
                    self.advance();
                }
            }
        }

        Grammar {
            name,
            rules,
            rule_order,
        }
    }

    fn parse_rule(&mut self, is_fragment: bool) -> Rule {
        let name = self.expect_ident();
        self.expect(&Tok::Colon);

        let mut alternatives = Vec::new();
        let mut is_hidden = false;

        alternatives.push(self.parse_alternative());

        while *self.peek() == Tok::Pipe {
            self.advance();
            alternatives.push(self.parse_alternative());
        }

        // Check for -> channel(HIDDEN) or -> skip
        if *self.peek() == Tok::Arrow {
            self.advance();
            if let Tok::Ident(ref directive) = *self.peek() {
                if directive == "channel" {
                    self.advance();
                    if *self.peek() == Tok::LParen {
                        self.advance();
                        if let Tok::Ident(ref ch) = *self.peek() {
                            if ch == "HIDDEN" {
                                is_hidden = true;
                            }
                            self.advance();
                        }
                        if *self.peek() == Tok::RParen {
                            self.advance();
                        }
                    }
                } else if directive == "skip" {
                    is_hidden = true;
                    self.advance();
                } else {
                    self.advance();
                }
            }
        }

        self.expect(&Tok::Semicolon);

        Rule {
            name,
            is_fragment,
            is_hidden,
            alternatives,
        }
    }

    fn parse_alternative(&mut self) -> Alternative {
        let mut elements = Vec::new();
        let mut label = None;

        loop {
            match self.peek() {
                Tok::Pipe | Tok::Semicolon | Tok::RParen | Tok::Arrow | Tok::Eof => break,
                Tok::Hash => {
                    // Alternative label: # labelName
                    self.advance();
                    if let Tok::Ident(_) = self.peek() {
                        label = Some(self.expect_ident());
                    }
                    // Labels can sometimes have extra identifiers
                    // (caseCondExprIs, etc.) - skip them
                    break;
                }
                _ => {
                    if let Some(elem) = self.parse_element() {
                        elements.push(elem);
                    }
                }
            }
        }

        Alternative { elements, label }
    }

    fn parse_element(&mut self) -> Option<Element> {
        let atom = self.parse_atom()?;
        // Apply quantifier if present
        match self.peek() {
            Tok::Question => {
                self.advance();
                Some(Element::Optional(Box::new(atom)))
            }
            Tok::Star => {
                self.advance();
                Some(Element::ZeroOrMore(Box::new(atom)))
            }
            Tok::Plus => {
                self.advance();
                Some(Element::OneOrMore(Box::new(atom)))
            }
            _ => Some(atom),
        }
    }

    fn parse_atom(&mut self) -> Option<Element> {
        match self.peek().clone() {
            Tok::Ident(ref s) if s == "EOF" => {
                self.advance();
                Some(Element::Eof)
            }
            Tok::Ident(s) => {
                self.advance();
                Some(Element::RuleRef(s))
            }
            Tok::StringLit(s) => {
                self.advance();
                // Check for character range: 'a'..'z'
                if *self.peek() == Tok::DotDot {
                    self.advance();
                    if let Tok::StringLit(end) = self.advance() {
                        let start_char = s.chars().next().unwrap_or('a');
                        let end_char = end.chars().next().unwrap_or('z');
                        return Some(Element::Range(start_char, end_char));
                    }
                }
                Some(Element::StringLiteral(s))
            }
            Tok::CharClass { body, negated } => {
                self.advance();
                let ranges = parse_char_class_body(&body);
                Some(Element::CharClass { ranges, negated })
            }
            Tok::LParen => {
                self.advance();
                let mut alts = vec![self.parse_alternative()];
                while *self.peek() == Tok::Pipe {
                    self.advance();
                    alts.push(self.parse_alternative());
                }
                self.expect(&Tok::RParen);
                Some(Element::Group(alts))
            }
            Tok::Tilde => {
                self.advance();
                let inner = self.parse_atom()?;
                Some(Element::Not(Box::new(inner)))
            }
            Tok::Dot => {
                self.advance();
                Some(Element::Wildcard)
            }
            _ => None,
        }
    }
}

/// Parse the body of a character class like `a-zA-Z0-9_` into `CharRange` items.
/// Handles escape sequences like `\t`, `\r`, `\n`.
fn parse_char_class_body(body: &str) -> Vec<CharRange> {
    let chars: Vec<char> = body.chars().collect();
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = resolve_escape(&chars, &mut i);
        if i < chars.len() && chars[i] == '-' && i + 1 < chars.len() {
            i += 1; // skip '-'
            let end = resolve_escape(&chars, &mut i);
            ranges.push(CharRange::Range(c, end));
        } else {
            ranges.push(CharRange::Single(c));
        }
    }
    ranges
}

/// Resolve a possibly-escaped character at position `i`, advancing `i` past it.
fn resolve_escape(chars: &[char], i: &mut usize) -> char {
    if chars[*i] == '\\' && *i + 1 < chars.len() {
        *i += 1;
        let c = match chars[*i] {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            other => other,
        };
        *i += 1;
        c
    } else {
        let c = chars[*i];
        *i += 1;
        c
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse an ANTLR4 .g4 grammar file from its text content.
pub fn parse_g4(input: &str) -> Grammar {
    let tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize_all();
    let mut parser = G4Parser::new(tokens);
    parser.parse_grammar()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_rule() {
        let input = r#"
            grammar Test;
            startRule : module EOF ;
            module : WS? NEWLINE* ;
            WS : [ \t]+ ;
            NEWLINE : '\r'? '\n' ;
        "#;
        let grammar = parse_g4(input);
        assert_eq!(grammar.name, "Test");
        assert!(grammar.rules.contains_key("startRule"));
        assert!(grammar.rules.contains_key("module"));
        assert!(grammar.rules.contains_key("WS"));
        assert!(grammar.rules.contains_key("NEWLINE"));
    }

    #[test]
    fn test_parse_alternatives() {
        let input = r#"
            grammar Test;
            rule1 : A | B | C ;
            A : 'a' ;
            B : 'b' ;
            C : 'c' ;
        "#;
        let grammar = parse_g4(input);
        assert_eq!(grammar.rules["rule1"].alternatives.len(), 3);
    }

    #[test]
    fn test_parse_char_class() {
        let input = r#"
            grammar Test;
            LETTER : [a-zA-Z_] ;
        "#;
        let grammar = parse_g4(input);
        let rule = &grammar.rules["LETTER"];
        assert_eq!(rule.alternatives.len(), 1);
    }

    #[test]
    fn test_parse_fragment() {
        let input = r#"
            grammar Test;
            fragment DIGIT : [0-9] ;
        "#;
        let grammar = parse_g4(input);
        assert!(grammar.rules["DIGIT"].is_fragment);
    }

    #[test]
    fn test_parse_hidden_channel() {
        let input = r#"
            grammar Test;
            COMMENT : '//' ~[\r\n]* -> channel(HIDDEN) ;
        "#;
        let grammar = parse_g4(input);
        assert!(grammar.rules["COMMENT"].is_hidden);
    }
}
