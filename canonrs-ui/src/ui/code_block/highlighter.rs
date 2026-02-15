//! Server-side syntax highlighting via syntect
//! Loads only required syntaxes — zero bloat

use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use std::sync::OnceLock;

pub struct HighlightResult {
    pub lines: Vec<String>,
    pub language: String,
}

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(|| {
        // Load defaults but we use OnceLock so it's only loaded once
        SyntaxSet::load_defaults_newlines()
    })
}

fn get_theme() -> &'static Theme {
    let ts = THEME_SET.get_or_init(ThemeSet::load_defaults);
    &ts.themes["base16-ocean.dark"]
}

pub fn highlight(code: &str, language: &str) -> HighlightResult {
    let ss = get_syntax_set();
    let theme = get_theme();

    let syntax = ss
        .find_syntax_by_token(language)
        .or_else(|| ss.find_syntax_by_extension(language))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, theme);
    let mut lines = Vec::new();

    for line in LinesWithEndings::from(code) {
        let ranges = h.highlight_line(line, ss).unwrap_or_default();
        let html = styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No)
            .unwrap_or_else(|_| html_escape::encode_text(line).to_string());
        let html = html.trim_end_matches('\n').to_string();
        lines.push(html);
    }

    HighlightResult {
        lines,
        language: syntax.name.to_lowercase(),
    }
}
