//! Markdown → HTML pipeline SSR-safe

use pulldown_cmark::{Parser, Options, Event, Tag, TagEnd, CodeBlockKind, HeadingLevel};
use crate::ui::code_block::highlighter::highlight;
use super::markdown_ui::RenderedMarkdown;
use super::toc_extractor::{TocExtractor, slugify};

pub fn render_markdown(markdown: &str) -> RenderedMarkdown {
    render_markdown_with_prefix(markdown, "")
}

pub fn render_markdown_with_prefix(markdown: &str, id_prefix: &str) -> RenderedMarkdown {
    let toc = if id_prefix.is_empty() {
        TocExtractor::extract_toc(markdown)
    } else {
        TocExtractor::extract_toc_with_prefix(markdown, id_prefix)
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut in_heading = false;
    let mut heading_level = HeadingLevel::H2;
    let mut heading_text = String::new();
    let mut heading_counter = 0usize;

    for event in parser {
        match event {

            // ── Headings ─────────────────────────────────────────────────────
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_level = level;
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                let slug = if heading_text.is_empty() {
                    format!("heading-{}", heading_counter)
                } else {
                    slugify(&heading_text)
                };
                let id = if id_prefix.is_empty() {
                    slug
                } else {
                    format!("{}-{}", id_prefix, slug)
                };
                heading_counter += 1;
                let tag = heading_tag(heading_level);
                let level_num = heading_level_num(heading_level);
                html.push_str(&format!(
                    "<{} id=\"{}\" data-md-heading=\"\" data-md-level=\"{}\">{}</{}>",
                    tag,
                    html_escape::encode_text(&id),
                    level_num,
                    html_escape::encode_text(&heading_text),
                    tag
                ));
                heading_text.clear();
            }

            // ── Code blocks ──────────────────────────────────────────────────
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

            // ── Text ─────────────────────────────────────────────────────────
            Event::Text(text) => {
                if in_heading {
                    heading_text.push_str(&text);
                } else if in_code_block {
                    code_content.push_str(&text);
                } else {
                    html.push_str(&html_escape::encode_text(&text));
                }
            }

            // ── Inline ───────────────────────────────────────────────────────
            Event::Code(code) => {
                html.push_str(&format!(
                    "<code data-md-inline-code=\"\">{}</code>",
                    html_escape::encode_text(&code)
                ));
            }

            // ── Block elements ───────────────────────────────────────────────
            Event::Start(Tag::Paragraph)      => html.push_str("<p data-md-p=\"\">"),
            Event::End(TagEnd::Paragraph)     => html.push_str("</p>"),
            Event::Start(Tag::BlockQuote(_))  => html.push_str("<blockquote data-md-blockquote=\"\">"),
            Event::End(TagEnd::BlockQuote(_)) => html.push_str("</blockquote>"),
            Event::Start(Tag::List(Some(n)))  => html.push_str(&format!("<ol data-md-ol=\"\" start=\"{}\">", n)),
            Event::End(TagEnd::List(true))    => html.push_str("</ol>"),
            Event::Start(Tag::List(None))     => html.push_str("<ul data-md-ul=\"\">"),
            Event::End(TagEnd::List(false))   => html.push_str("</ul>"),
            Event::Start(Tag::Item)           => html.push_str("<li data-md-li=\"\">"),
            Event::End(TagEnd::Item)          => html.push_str("</li>"),

            // ── Inline formatting ─────────────────────────────────────────────
            Event::Start(Tag::Emphasis)       => html.push_str("<em>"),
            Event::End(TagEnd::Emphasis)      => html.push_str("</em>"),
            Event::Start(Tag::Strong)         => html.push_str("<strong>"),
            Event::End(TagEnd::Strong)        => html.push_str("</strong>"),
            Event::Start(Tag::Strikethrough)  => html.push_str("<del>"),
            Event::End(TagEnd::Strikethrough) => html.push_str("</del>"),

            // ── Links & Images ────────────────────────────────────────────────
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

            // ── Tables ───────────────────────────────────────────────────────
            Event::Start(Tag::Table(_))    => html.push_str("<table data-md-table=\"\">"),
            Event::End(TagEnd::Table)      => html.push_str("</table>"),
            Event::Start(Tag::TableHead)   => html.push_str("<thead data-md-thead=\"\"><tr>"),
            Event::End(TagEnd::TableHead)  => html.push_str("</tr></thead>"),
            Event::Start(Tag::TableRow)    => html.push_str("<tr data-md-tr=\"\">"),
            Event::End(TagEnd::TableRow)   => html.push_str("</tr>"),
            Event::Start(Tag::TableCell)   => html.push_str("<td data-md-td=\"\">"),
            Event::End(TagEnd::TableCell)  => html.push_str("</td>"),

            // ── Misc ─────────────────────────────────────────────────────────
            Event::Rule      => html.push_str("<hr data-md-hr=\"\">"),
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

fn heading_level_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}
