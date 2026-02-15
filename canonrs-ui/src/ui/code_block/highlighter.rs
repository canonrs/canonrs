//! Server-side syntax highlighting via syntect
//! Runs at SSR time — zero client JS needed

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct HighlightResult {
    pub lines: Vec<String>,
    pub language: String,
}

pub fn highlight(code: &str, language: &str) -> HighlightResult {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ss
        .find_syntax_by_token(language)
        .or_else(|| ss.find_syntax_by_extension(language))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let theme = &ts.themes["base16-ocean.dark"];
    let mut h = HighlightLines::new(syntax, theme);

    let mut lines = Vec::new();
    for line in LinesWithEndings::from(code) {
        let ranges = h.highlight_line(line, &ss).unwrap_or_default();
        let html = styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No)
            .unwrap_or_else(|_| html_escape::encode_text(line).to_string());
        // Remove trailing newline from HTML
        let html = html.trim_end_matches('\n').to_string();
        lines.push(html);
    }

    HighlightResult {
        lines,
        language: syntax.name.to_lowercase(),
    }
}
