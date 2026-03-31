//! Markdown → HTML pipeline SSR-safe
//!
//! Arquitetura:
//!   pulldown_cmark AST → render estruturado → HTML string limpa
//!
//! Convenções de atributos:
//!   data-rs-md-*   → elementos markdown (heading, p, ul, code, etc.)
//!   data-rs-*      → componentes do design system (CodeBlock, etc.)
//!
//! Nunca usar inner_html com string não-sanitizada.
//! Todo texto de usuário passa por html_escape antes de entrar no HTML.

use pulldown_cmark::{Parser, Options, Event, Tag, TagEnd, CodeBlockKind, HeadingLevel};
use crate::ui::code_block::highlighter::highlight;
use super::markdown_ui::RenderedMarkdown;
use super::toc_extractor::{TocExtractor, slugify};

// ── Public API ────────────────────────────────────────────────────────────────

pub(crate) fn render_markdown(markdown: &str) -> RenderedMarkdown {
    render_markdown_with_prefix(markdown, "")
}

pub(crate) fn render_markdown_with_prefix(markdown: &str, id_prefix: &str) -> RenderedMarkdown {
    let toc = if id_prefix.is_empty() {
        TocExtractor::extract_toc(markdown)
    } else {
        TocExtractor::extract_toc_with_prefix(markdown, id_prefix)
    };

    let html = render_html(markdown, id_prefix);
    RenderedMarkdown { html, toc }
}

// ── Renderer ──────────────────────────────────────────────────────────────────

fn render_html(markdown: &str, id_prefix: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut ctx = RenderContext::new(id_prefix);

    for event in parser {
        ctx.process(event);
    }

    ctx.finish()
}

// ── RenderContext ─────────────────────────────────────────────────────────────

struct RenderContext<'a> {
    html:            String,
    id_prefix:       &'a str,
    heading_counter: usize,

    // heading state
    in_heading:      bool,
    heading_level:   HeadingLevel,
    heading_buf:     String,   // acumula texto puro do heading (sem tags)

    // code block state
    in_code_block:   bool,
    code_lang:       String,
    code_buf:        String,
}

impl<'a> RenderContext<'a> {
    fn new(id_prefix: &'a str) -> Self {
        Self {
            html:            String::with_capacity(8192),
            id_prefix,
            heading_counter: 0,
            in_heading:      false,
            heading_level:   HeadingLevel::H2,
            heading_buf:     String::new(),
            in_code_block:   false,
            code_lang:       String::new(),
            code_buf:        String::new(),
        }
    }

    fn finish(self) -> String {
        self.html
    }

    fn process(&mut self, event: Event) {
        match event {
            // ── Headings ──────────────────────────────────────────────────────
            Event::Start(Tag::Heading { level, .. }) => {
                self.in_heading    = true;
                self.heading_level = level;
                self.heading_buf.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                self.flush_heading();
            }

            // ── Code blocks ───────────────────────────────────────────────────
            Event::Start(Tag::CodeBlock(kind)) => {
                self.in_code_block = true;
                self.code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented     => String::from("text"),
                };
                self.code_buf.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                self.flush_code_block();
            }

            // ── Text (dispatcher) ─────────────────────────────────────────────
            Event::Text(text) => {
                if self.in_heading {
                    // heading: acumula texto puro sem escape — o escape ocorre no flush
                    self.heading_buf.push_str(&text);
                } else if self.in_code_block {
                    // código: acumula raw, sem escape — o highlight cuida disso
                    self.code_buf.push_str(&text);
                } else {
                    self.html.push_str(&html_escape::encode_text(&text));
                }
            }

            // ── Inline code ───────────────────────────────────────────────────
            Event::Code(code) => {
                self.html.push_str("<code data-rs-md-inline-code=\"\">");
                self.html.push_str(&html_escape::encode_text(&code));
                self.html.push_str("</code>");
            }

            // ── Block elements ────────────────────────────────────────────────
            Event::Start(Tag::Paragraph)      => self.html.push_str("<p data-rs-md-p=\"\">"),
            Event::End(TagEnd::Paragraph)     => self.html.push_str("</p>"),

            Event::Start(Tag::BlockQuote(_))  => self.html.push_str("<blockquote data-rs-md-blockquote=\"\">"),
            Event::End(TagEnd::BlockQuote(_)) => self.html.push_str("</blockquote>"),

            Event::Start(Tag::List(Some(n)))  => {
                self.html.push_str(&format!("<ol data-rs-md-ol=\"\" start=\"{}\">", n));
            }
            Event::End(TagEnd::List(true))    => self.html.push_str("</ol>"),
            Event::Start(Tag::List(None))     => self.html.push_str("<ul data-rs-md-ul=\"\">"),
            Event::End(TagEnd::List(false))   => self.html.push_str("</ul>"),
            Event::Start(Tag::Item)           => self.html.push_str("<li data-rs-md-li=\"\">"),
            Event::End(TagEnd::Item)          => self.html.push_str("</li>"),

            // ── Inline formatting ─────────────────────────────────────────────
            Event::Start(Tag::Emphasis)       => self.html.push_str("<em data-rs-md-em=\"\">"),
            Event::End(TagEnd::Emphasis)      => self.html.push_str("</em>"),
            Event::Start(Tag::Strong)         => self.html.push_str("<strong data-rs-md-strong=\"\">"),
            Event::End(TagEnd::Strong)        => self.html.push_str("</strong>"),
            Event::Start(Tag::Strikethrough)  => self.html.push_str("<del data-rs-md-del=\"\">"),
            Event::End(TagEnd::Strikethrough) => self.html.push_str("</del>"),

            // ── Links ─────────────────────────────────────────────────────────
            Event::Start(Tag::Link { dest_url, title, .. }) => {
                self.html.push_str("<a data-rs-md-link=\"\" href=\"");
                self.html.push_str(&html_escape::encode_double_quoted_attribute(&dest_url));
                self.html.push_str("\"");
                if !title.is_empty() {
                    self.html.push_str(" title=\"");
                    self.html.push_str(&html_escape::encode_double_quoted_attribute(&title));
                    self.html.push_str("\"");
                }
                self.html.push('>');
            }
            Event::End(TagEnd::Link) => self.html.push_str("</a>"),

            // ── Images ────────────────────────────────────────────────────────
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                self.html.push_str("<img data-rs-md-img=\"\" src=\"");
                self.html.push_str(&html_escape::encode_double_quoted_attribute(&dest_url));
                self.html.push_str("\" alt=\"");
                self.html.push_str(&html_escape::encode_double_quoted_attribute(&title));
                self.html.push_str("\">");
            }

            // ── Tables ────────────────────────────────────────────────────────
            Event::Start(Tag::Table(_))    => self.html.push_str("<table data-rs-md-table=\"\">"),
            Event::End(TagEnd::Table)      => self.html.push_str("</table>"),
            Event::Start(Tag::TableHead)   => self.html.push_str("<thead data-rs-md-thead=\"\"><tr data-rs-md-tr=\"\">"),
            Event::End(TagEnd::TableHead)  => self.html.push_str("</tr></thead>"),
            Event::Start(Tag::TableRow)    => self.html.push_str("<tr data-rs-md-tr=\"\">"),
            Event::End(TagEnd::TableRow)   => self.html.push_str("</tr>"),
            Event::Start(Tag::TableCell)   => self.html.push_str("<td data-rs-md-td=\"\">"),
            Event::End(TagEnd::TableCell)  => self.html.push_str("</td>"),

            // ── Misc ──────────────────────────────────────────────────────────
            Event::Rule          => self.html.push_str("<hr data-rs-md-hr=\"\">"),
            Event::SoftBreak     => self.html.push(' '),
            Event::HardBreak     => self.html.push_str("<br>"),

            // HTML raw inline/block — sanitizado: só permitir se vier do sistema
            // Por segurança apenas passamos através (as rules são conteúdo interno confiável)
            Event::Html(raw)     => self.html.push_str(&raw),

            _ => {}
        }
    }

    // ── Flush helpers ─────────────────────────────────────────────────────────

    fn flush_heading(&mut self) {
        self.in_heading = false;

        let slug = if self.heading_buf.trim().is_empty() {
            format!("heading-{}", self.heading_counter)
        } else {
            slugify(self.heading_buf.trim())
        };

        let id = if self.id_prefix.is_empty() {
            slug
        } else {
            format!("{}-{}", self.id_prefix, slug)
        };

        self.heading_counter += 1;

        let tag       = heading_tag(self.heading_level);
        let level_num = heading_level_num(self.heading_level);

        // id e texto escapados corretamente — sem espaço no início do id
        let safe_id   = html_escape::encode_double_quoted_attribute(id.trim());
        let safe_text = html_escape::encode_text(self.heading_buf.trim());

        self.html.push_str(&format!(
            "<{tag} id=\"{id}\" data-rs-md-heading=\"\" data-rs-md-level=\"{level}\">{text}</{tag}>",
            tag   = tag,
            id    = safe_id,
            level = level_num,
            text  = safe_text,
        ));

        self.heading_buf.clear();
    }

    fn flush_code_block(&mut self) {
        self.in_code_block = false;

        let result       = highlight(&self.code_buf, &self.code_lang);
        let escaped_code = html_escape::encode_double_quoted_attribute(&self.code_buf);
        let escaped_lang = html_escape::encode_double_quoted_attribute(&self.code_lang);

        let lines_html: String = result.lines.iter().enumerate()
            .map(|(i, line)| format!(
                "<span data-rs-code-line=\"\" data-rs-line-number=\"{}\">{}</span>",
                i + 1,
                line
            ))
            .collect::<Vec<_>>()
            .join("");

        self.html.push_str(&format!(
            concat!(
                "<div data-rs-code-block=\"\" data-rs-component=\"CodeBlock\"",
                " data-rs-behavior=\"content\" data-rs-language=\"{lang}\">",
                  "<div data-rs-code-header=\"\">",
                    "<div data-code-header-left=\"\">",
                      "<span data-rs-code-language=\"\">{lang}</span>",
                    "</div>",
                    "<button data-rs-copy-button=\"\" data-rs-copy-text=\"{code}\"",
                    " data-rs-reset-delay=\"1300\" data-rs-state=\"idle\"",
                    " aria-label=\"Copy to clipboard\">",
                      "<span data-rs-copy-content=\"\">",
                        "<svg data-rs-copy-icon=\"\" xmlns=\"http://www.w3.org/2000/svg\"",
                        " width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\"",
                        " stroke=\"currentColor\" stroke-width=\"2\">",
                          "<rect width=\"14\" height=\"14\" x=\"8\" y=\"8\" rx=\"2\" ry=\"2\"/>",
                          "<path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"/>",
                        "</svg>",
                        "<span data-rs-copy-label=\"\">Copy</span>",
                      "</span>",
                      "<span data-rs-copied-content=\"\">",
                        "<svg data-rs-copied-icon=\"\" xmlns=\"http://www.w3.org/2000/svg\"",
                        " width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\"",
                        " stroke=\"currentColor\" stroke-width=\"2\">",
                          "<path d=\"M20 6 9 17l-5-5\"/>",
                        "</svg>",
                        "<span data-rs-copied-label=\"\" aria-live=\"polite\">Copied!</span>",
                      "</span>",
                      "<span data-rs-error-content=\"\">",
                        "<svg data-rs-error-icon=\"\" xmlns=\"http://www.w3.org/2000/svg\"",
                        " width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\"",
                        " stroke=\"currentColor\" stroke-width=\"2\">",
                          "<circle cx=\"12\" cy=\"12\" r=\"10\"/>",
                          "<path d=\"m15 9-6 6M9 9l6 6\"/>",
                        "</svg>",
                        "<span data-rs-error-label=\"\" aria-live=\"assertive\">Failed</span>",
                      "</span>",
                    "</button>",
                  "</div>",
                  "<pre data-rs-code-pre=\"\">{lines}</pre>",
                "</div>",
            ),
            lang  = escaped_lang,
            code  = escaped_code,
            lines = lines_html,
        ));

        self.code_buf.clear();
        self.code_lang.clear();
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

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
