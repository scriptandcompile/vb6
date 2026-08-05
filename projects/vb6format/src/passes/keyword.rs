use crate::context::Context;
use crate::passes::{FormatPass, TokenBuffer};
use crate::settings::FmtSettings;
use vb6parse::SyntaxKind;
use vb6parse::parsers::CstNode;

pub struct KeywordCasePass<'a> {
    settings: &'a FmtSettings,
}

impl<'a> KeywordCasePass<'a> {
    pub fn new(settings: &'a FmtSettings) -> Self {
        Self { settings }
    }

    fn keyword_camel_text(&self, kind: SyntaxKind) -> Option<String> {
        if !kind.is_keyword() {
            return None;
        }
        let kind_name = kind.to_string();
        kind_name.strip_suffix("Keyword").map(|s| s.to_string())
    }
}

impl FormatPass for KeywordCasePass<'_> {
    fn on_token(&self, token: &CstNode, _context: &mut Context, buffer: &mut TokenBuffer) {
        let Some(canonical_keyword) = self.keyword_camel_text(token.kind()) else {
            return;
        };

        buffer.text = match self.settings.keyword_case.as_str() {
            "upper" => canonical_keyword.to_ascii_uppercase(),
            "lower" => canonical_keyword.to_ascii_lowercase(),
            "camel" => canonical_keyword,
            "first" => {
                let mut chars = canonical_keyword.chars();
                let Some(first) = chars.next() else {
                    return;
                };
                let mut out = String::new();
                out.push_str(&first.to_uppercase().to_string());
                out.push_str(&chars.as_str().to_ascii_lowercase());
                out
            }
            _ => token.text().to_string(),
        };
    }
}
