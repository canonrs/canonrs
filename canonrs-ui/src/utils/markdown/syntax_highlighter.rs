use html_escape;
pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self
    }

    pub fn highlight(&self, code: &str, _language: &str) -> Result<String, String> {
        Ok(html_escape::encode_text(code).to_string())
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
