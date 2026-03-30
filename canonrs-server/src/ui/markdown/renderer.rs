//! Markdown → HTML pipeline SSR-safe

use pulldown_cmark::{Parser, Options, Event, Tag, TagEnd, CodeBlockKind, HeadingLevel};
use crate::ui::code_block::highlighter::highlight;
use super::markdown_ui::RenderedMarkdown;
use super::toc_extractor::{TocExtractor, slugify};

pub(crate) fn render_markdown(markdown: &str) -> RenderedMarkdown {
    render_markdown_with_prefix(markdown, "")
}

pub(crate) fn render_markdown_with_prefix(markdown: &str, id_prefix: &str) -> RenderedMarkdown {
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
                let lines_html: String = result.lines.iter().enumerate()
                    .map(|(i, line)| format!(
                        "<span data-rs-code-line=\"\" data-rs-line-number=\"{}\">{}</span>",
                        i + 1, line
                    ))
                    .collect::<Vec<_>>()
                    .join("");
                let escaped_code = html_escape::encode_double_quoted_attribute(&code_content);
                let escaped_lang = html_escape::encode_text(&code_lang);
                html.push_str(&format!(
                    "<div data-rs-code-block=\"\" data-rs-component=\"CodeBlock\" data-rs-behavior=\"content\" data-rs-language=\"{lang}\"><div data-rs-code-header=\"\"><div data-code-header-left=\"\"><span data-rs-code-language=\"\">{lang}</span></div><button data-rs-copy-button=\"\" data-rs-copy-text=\"{code}\" data-rs-reset-delay=\"1300\" data-rs-state=\"idle\" aria-label=\"Copy to clipboard\"><span data-rs-copy-content=\"\"><svg data-rs-copy-icon=\"\" xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"><rect width=\"14\" height=\"14\" x=\"8\" y=\"8\" rx=\"2\" ry=\"2\"/><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"/></svg><span data-rs-copy-label=\"\">Copy</span></span><span data-rs-copied-content=\"\"><svg data-rs-copied-icon=\"\" xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"><path d=\"M20 6 9 17l-5-5\"/></svg><span data-rs-copied-label=\"\" aria-live=\"polite\">Copied!</span></span><span data-rs-error-content=\"\"><svg data-rs-error-icon=\"\" xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"m15 9-6 6M9 9l6 6\"/></svg><span data-rs-error-label=\"\" aria-live=\"assertive\">Failed</span></span></button></div><pre data-rs-code-pre=\"\">{lines}</pre></div>",
                    lang = escaped_lang,
                    code = escaped_code,
                    lines = lines_html,
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
            Event::Start(Tag::Table(_))    => html.push_str("<table data-rs-md-table=\"\">"),
            Event::End(TagEnd::Table)      => html.push_str("</table>"),
            Event::Start(Tag::TableHead)   => html.push_str("<thead data-rs-md-thead=\"\"><tr>"),
            Event::End(TagEnd::TableHead)  => html.push_str("</tr></thead>"),
            Event::Start(Tag::TableRow)    => html.push_str("<tr data-rs-md-tr=\"\">"),
            Event::End(TagEnd::TableRow)   => html.push_str("</tr>"),
            Event::Start(Tag::TableCell)   => html.push_str("<td data-rs-md-td=\"\">"),
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
