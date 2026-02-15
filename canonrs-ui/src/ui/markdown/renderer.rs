//! Markdown → HTML pipeline SSR-safe

use pulldown_cmark::{Parser, Options, Event, Tag, TagEnd, CodeBlockKind, HeadingLevel};
use crate::ui::code_block::highlighter::highlight;
use super::markdown_ui::RenderedMarkdown;
use super::toc_extractor::TocExtractor;

pub fn render_markdown(markdown: &str) -> RenderedMarkdown {
    let toc = TocExtractor::extract_toc(markdown);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                html.push_str(&format!("<{} data-md-heading=\"\">", heading_tag(level)));
            }
            Event::End(TagEnd::Heading(level)) => {
                html.push_str(&format!("</{}>", heading_tag(level)));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => "text".to_string(),
                };
                code_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let result = highlight(&code_content, &code_lang);
                html.push_str(&format!(
                    "<div data-code-block=\"\" data-language=\"{}\"><div data-code-header=\"\"><span data-code-language=\"\">{}</span></div><pre data-code-pre=\"\">{}</pre></div>",
                    html_escape::encode_text(&code_lang),
                    html_escape::encode_text(&code_lang),
                    result.lines.join("\n")
                ));
                code_content.clear();
                code_lang.clear();
            }
            Event::Text(text) => {
                if in_code_block {
                    code_content.push_str(&text);
                } else {
                    html.push_str(&html_escape::encode_text(&text));
                }
            }
            Event::Code(code) => {
                html.push_str(&format!(
                    "<code data-md-inline-code=\"\">{}</code>",
                    html_escape::encode_text(&code)
                ));
            }
            Event::Start(Tag::Paragraph) => html.push_str("<p data-md-p=\"\">"),
            Event::End(TagEnd::Paragraph) => html.push_str("</p>"),
            Event::Start(Tag::BlockQuote(_)) => html.push_str("<blockquote data-md-blockquote=\"\">"),
            Event::End(TagEnd::BlockQuote(_)) => html.push_str("</blockquote>"),
            Event::Start(Tag::List(Some(start))) => html.push_str(&format!("<ol data-md-ol=\"\" start=\"{}\">", start)),
            Event::End(TagEnd::List(true)) => html.push_str("</ol>"),
            Event::Start(Tag::List(None)) => html.push_str("<ul data-md-ul=\"\">"),
            Event::End(TagEnd::List(false)) => html.push_str("</ul>"),
            Event::Start(Tag::Item) => html.push_str("<li data-md-li=\"\">"),
            Event::End(TagEnd::Item) => html.push_str("</li>"),
            Event::Start(Tag::Emphasis) => html.push_str("<em>"),
            Event::End(TagEnd::Emphasis) => html.push_str("</em>"),
            Event::Start(Tag::Strong) => html.push_str("<strong>"),
            Event::End(TagEnd::Strong) => html.push_str("</strong>"),
            Event::Start(Tag::Strikethrough) => html.push_str("<del>"),
            Event::End(TagEnd::Strikethrough) => html.push_str("</del>"),
            Event::Start(Tag::Link { dest_url, title, .. }) => {
                html.push_str(&format!(
                    "<a data-md-link=\"\" href=\"{}\" title=\"{}\">",
                    html_escape::encode_text(&dest_url),
                    html_escape::encode_text(&title)
                ));
            }
            Event::End(TagEnd::Link) => html.push_str("</a>"),
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                html.push_str(&format!(
                    "<img data-md-img=\"\" src=\"{}\" alt=\"{}\">",
                    html_escape::encode_text(&dest_url),
                    html_escape::encode_text(&title)
                ));
            }
            Event::Start(Tag::Table(_)) => html.push_str("<table data-md-table=\"\">"),
            Event::End(TagEnd::Table) => html.push_str("</table>"),
            Event::Start(Tag::TableHead) => html.push_str("<thead><tr>"),
            Event::End(TagEnd::TableHead) => html.push_str("</tr></thead>"),
            Event::Start(Tag::TableRow) => html.push_str("<tr>"),
            Event::End(TagEnd::TableRow) => html.push_str("</tr>"),
            Event::Start(Tag::TableCell) => html.push_str("<td>"),
            Event::End(TagEnd::TableCell) => html.push_str("</td>"),
            Event::Rule => html.push_str("<hr data-md-hr=\"\">"),
            Event::SoftBreak => html.push(' '),
            Event::HardBreak => html.push_str("<br>"),
            Event::Html(raw) => html.push_str(&raw),
            _ => {}
        }
    }

    RenderedMarkdown { html, toc }
}

fn heading_tag(level: HeadingLevel) -> &'static str {
    match level {
        HeadingLevel::H1 => "h1",
        HeadingLevel::H2 => "h2",
        HeadingLevel::H3 => "h3",
        HeadingLevel::H4 => "h4",
        HeadingLevel::H5 => "h5",
        HeadingLevel::H6 => "h6",
    }
}
