use leptos::prelude::*;
use canonrs_core::TocItem;
use canonrs_core::primitives::markdown::MarkdownPrimitive;

#[derive(Clone, Debug, Default)]
pub struct RenderedMarkdown {
    pub html: String,
    pub toc: Vec<TocItem>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TocPosition {
    #[default]
    Top,
    Sidebar,
}

#[cfg(feature = "ssr")]
fn build_inner_html(
    rendered: &RenderedMarkdown,
    show_toc: bool,
    show_toolbar: bool,
    toc_position: TocPosition,
    id: &str,
) -> String {
    let has_toc = !rendered.toc.is_empty() && show_toc;
    let is_sidebar = toc_position == TocPosition::Sidebar;
    let toc_id = format!("{}-toc", id);
    let mut out = String::new();

    // Toolbar
    if show_toolbar && !is_sidebar {
        out.push_str("<div data-rs-markdown-toolbar=\"\" role=\"toolbar\" aria-label=\"Markdown toolbar\">");
        if has_toc {
            out.push_str("<button type=\"button\" data-rs-markdown-toolbar-item=\"\" data-rs-action=\"toggle-toc\">Contents</button>");
        }
        out.push_str("</div>");
    }

    out.push_str("<div data-rs-md-layout=\"\">");

    // TOC top
    if has_toc && !is_sidebar {
        out.push_str(&format!(
            "<nav data-rs-markdown-toc=\"\" data-rs-state=\"closed\" aria-label=\"Table of contents\"><nav data-rs-toc=\"\" data-rs-mode=\"expand\" id=\"{}\"><p data-rs-toc-title=\"\">On this page</p><ul data-rs-toc-list=\"\">",
            toc_id
        ));
        for item in &rendered.toc {
            let is_child = item.level > 2;
            out.push_str(&format!(
                "<li data-rs-toc-item=\"\" data-rs-level=\"{}\" data-rs-target=\"{}\" data-rs-state=\"idle\" data-rs-child=\"{}\" data-rs-has-children=\"false\"><a data-rs-toc-link=\"\" href=\"#{}\">{}</a></li>",
                item.level,
                item.id,
                if is_child { "true" } else { "false" },
                item.id,
                item.text,
            ));
        }
        out.push_str("</ul></nav></nav>");
    }

    // TOC sidebar
    if has_toc && is_sidebar {
        out.push_str(&format!(
            "<aside data-rs-md-toc-sidebar=\"\"><nav data-rs-toc=\"\" data-rs-mode=\"expand\" id=\"{}\"><p data-rs-toc-title=\"\">On this page</p><ul data-rs-toc-list=\"\">",
            toc_id
        ));
        for item in &rendered.toc {
            let is_child = item.level > 2;
            out.push_str(&format!(
                "<li data-rs-toc-item=\"\" data-rs-level=\"{}\" data-rs-target=\"{}\" data-rs-state=\"idle\" data-rs-child=\"{}\" data-rs-has-children=\"false\"><a data-rs-toc-link=\"\" href=\"#{}\">{}</a></li>",
                item.level,
                item.id,
                if is_child { "true" } else { "false" },
                item.id,
                item.text,
            ));
        }
        out.push_str("</ul></nav></aside>");
    }

    // Content
    out.push_str(&format!(
        "<div data-rs-markdown-content=\"\" inner_html=\"{}\"></div>",
        rendered.html
    ));

    out.push_str("</div>");
    out
}

#[cfg(not(feature = "ssr"))]
fn build_inner_html(
    _rendered: &RenderedMarkdown,
    _show_toc: bool,
    _show_toolbar: bool,
    _toc_position: TocPosition,
    _id: &str,
) -> String {
    String::new()
}

#[component]
pub fn MarkdownSurface(
    rendered: RenderedMarkdown,
    #[prop(default = true)] show_toc: bool,
    #[prop(default = true)] show_toolbar: bool,
    #[prop(default = TocPosition::Top)] toc_position: TocPosition,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    let is_sidebar = toc_position == TocPosition::Sidebar;
    let inner = build_inner_html(&rendered, show_toc, show_toolbar, toc_position, &id);

    view! {
        <MarkdownPrimitive
            inner=inner
            attr:data-toc-position=if is_sidebar { "sidebar" } else { "top" }
        />
    }
}

#[component]
pub fn MarkdownPreview() -> impl IntoView {
    let rendered = RenderedMarkdown { html: "<p>Markdown preview</p>".to_string(), toc: vec![] };
    view! { <MarkdownSurface rendered=rendered show_toc=false show_toolbar=false /> }
}
