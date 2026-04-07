
use leptos::prelude::*;
use canonrs_core::TocItem;

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
fn build_html(
    rendered: &RenderedMarkdown,
    show_toc: bool,
    show_toolbar: bool,
    toc_position: TocPosition,
    id: &str,
    extra_attrs: &str,
) -> String {
    let has_toc = !rendered.toc.is_empty() && show_toc;
    let is_sidebar = toc_position == TocPosition::Sidebar;
    let toc_id = format!("{}-toc", id);
    let toc_pos = if is_sidebar { "sidebar" } else { "top" };
    let mut out = String::new();

    // Shell outer
    out.push_str(&format!(
        "<div data-rs-markdown=\"\" data-rs-component=\"Markdown\" data-rs-behavior=\"content\" data-toc-position=\"{}\" data-rs-value=\"{}\" {}>",
        toc_pos, id, extra_attrs
    ));

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
                html_escape::encode_double_quoted_attribute(&item.id),
                if is_child { "true" } else { "false" },
                html_escape::encode_double_quoted_attribute(&item.id),
                html_escape::encode_text(&item.text),
            ));
        }
        out.push_str("</ul></nav></nav>");
    }

    // TOC sidebar — fora do scroll wrapper
    if has_toc && is_sidebar {
        let content_id = format!("{}-content", id);
        out.push_str(&format!(
            "<aside data-rs-md-toc-sidebar=\"\" data-rs-scroll-target=\"{scroll_target}\"><nav data-rs-toc=\"\" data-rs-mode=\"nested\" id=\"{toc_id}\"><p data-rs-toc-title=\"\">On this page</p><ul data-rs-toc-list=\"\">",
            scroll_target = content_id,
            toc_id = toc_id
        ));
        for item in &rendered.toc {
            let is_child = item.level > 2;
            out.push_str(&format!(
                "<li data-rs-toc-item=\"\" data-rs-level=\"{}\" data-rs-target=\"{}\" data-rs-state=\"idle\" data-rs-child=\"{}\" data-rs-has-children=\"false\"><a data-rs-toc-link=\"\" href=\"#{}\">{}</a></li>",
                item.level,
                html_escape::encode_double_quoted_attribute(&item.id),
                if is_child { "true" } else { "false" },
                html_escape::encode_double_quoted_attribute(&item.id),
                html_escape::encode_text(&item.text),
            ));
        }
        out.push_str("</ul></nav></aside>");
    }

    // Content — HTML direto, sem inner_html como atributo
    out.push_str(&format!("<div data-rs-markdown-content=\"\" id=\"{}-content\">", id));
    out.push_str(&rendered.html);
    out.push_str("</div>");

    out.push_str("</div>"); // data-rs-md-layout
    out.push_str("</div>"); // data-rs-markdown

    out
}

#[cfg(not(feature = "ssr"))]
fn build_html(
    _rendered: &RenderedMarkdown,
    _show_toc: bool,
    _show_toolbar: bool,
    _toc_position: TocPosition,
    _id: &str,
    _extra_attrs: &str,
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
    let html = build_html(&rendered, show_toc, show_toolbar, toc_position, &id, "");

    // SSR: injeta o HTML completo via inner_html no wrapper raiz
    // O wrapper raiz é neutro — apenas âncora para o inner_html
    view! {
        <div inner_html=html></div>
    }
}

#[component]
pub fn MarkdownPreview() -> impl IntoView {
    let rendered = RenderedMarkdown { html: "<p>Markdown preview</p>".to_string(), toc: vec![] };
    view! { <MarkdownSurface rendered=rendered show_toc=false show_toolbar=false /> }
}

/// Renders only the TOC from a RenderedMarkdown — composable, layout-agnostic.
#[component]
pub fn MarkdownTOC(
    #[allow(unused_variables)] toc: Vec<canonrs_core::TocItem>,
    #[prop(into, default = String::new())]
    #[allow(unused_variables)] id: String,
    #[prop(into, optional)]
    #[allow(unused_variables)] scroll_target: Option<String>,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let html = {
        let toc_id = format!("{}-toc", id);
        let scroll_attr = scroll_target
            .map(|t| format!(" data-rs-scroll-target=\"{}\"", t))
            .unwrap_or_default();
        let mut out = format!(
            "<aside data-rs-md-toc-sidebar=\"\"{}><nav data-rs-toc=\"\" data-rs-mode=\"expand\" id=\"{}\"><p data-rs-toc-title=\"\">On this page</p><ul data-rs-toc-list=\"\">",
            scroll_attr, toc_id
        );
        for item in &toc {
            let is_child = item.level > 2;
            out.push_str(&format!(
                "<li data-rs-toc-item=\"\" data-rs-level=\"{}\" data-rs-target=\"{}\" data-rs-state=\"idle\" data-rs-child=\"{}\"><a data-rs-toc-link=\"\" href=\"#{}\">{}</a></li>",
                item.level,
                html_escape::encode_double_quoted_attribute(&item.id),
                if is_child { "true" } else { "false" },
                html_escape::encode_double_quoted_attribute(&item.id),
                html_escape::encode_text(&item.text),
            ));
        }
        out.push_str("</ul></nav></aside>");
        out
    };
    #[cfg(not(feature = "ssr"))]
    let html = String::new();

    view! { <div inner_html=html /> }
}

/// Renders only the markdown content — no TOC, no layout.
#[component]
pub fn MarkdownContent(
    #[allow(unused_variables)] rendered: RenderedMarkdown,
    #[prop(into, default = String::new())]
    #[allow(unused_variables)] id: String,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let html = format!(
        "<div data-rs-markdown-content=\"\" id=\"{}-content\">{}</div>",
        id, rendered.html
    );
    #[cfg(not(feature = "ssr"))]
    let html = String::new();

    view! { <div inner_html=html /> }
}

/// Layout shell — composes TOC + Content with grid layout.
#[component]
pub fn MarkdownLayout(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = TocPosition::Sidebar)] toc_position: TocPosition,
) -> impl IntoView {
    let toc_pos = if toc_position == TocPosition::Sidebar { "sidebar" } else { "top" };
    view! {
        <div
            data-rs-markdown=""
            data-rs-component="Markdown"
            data-rs-behavior="content"
            data-toc-position=toc_pos
            data-rs-value=value
        >
            <div data-rs-md-layout="">
                {children()}
            </div>
        </div>
    }
}
