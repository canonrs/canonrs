use leptos::prelude::*;
use super::markdown_ui::{MarkdownSurface, MarkdownTOC, MarkdownContent, MarkdownLayout, RenderedMarkdown, TocPosition};

#[component]
pub fn MarkdownSurfaceIsland(
    rendered: RenderedMarkdown,
    #[prop(optional)] show_toc: Option<bool>,
    #[prop(optional)] show_toolbar: Option<bool>,
    #[prop(optional, into)] toc_position: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let toc_pos = match toc_position.as_deref() {
        Some("sidebar") => TocPosition::Sidebar,
        _               => TocPosition::Top,
    };
    view! {
        <MarkdownSurface
            rendered=rendered
            show_toc=show_toc.unwrap_or(true)
            show_toolbar=show_toolbar.unwrap_or(true)
            toc_position=toc_pos
            id=id.unwrap_or_default()
        />
    }
}

#[component]
pub fn MarkdownLayoutIsland(
    children: Children,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] toc_position: Option<String>,
) -> impl IntoView {
    let toc_pos = match toc_position.as_deref() {
        Some("sidebar") => TocPosition::Sidebar,
        _               => TocPosition::Top,
    };
    view! {
        <MarkdownLayout value=value.unwrap_or_default() toc_position=toc_pos>
            {children()}
        </MarkdownLayout>
    }
}

#[component]
pub fn MarkdownContentIsland(
    rendered: RenderedMarkdown,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! { <MarkdownContent rendered=rendered id=id.unwrap_or_default() /> }
}

#[component]
pub fn MarkdownTOCIsland(
    toc: Vec<canonrs_core::TocItem>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] scroll_target: Option<String>,
) -> impl IntoView {
    view! { <MarkdownTOC toc=toc id=id.unwrap_or_default() scroll_target=scroll_target.unwrap_or_default() /> }
}
