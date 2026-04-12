

use leptos::prelude::*;
use super::markdown_ui::{
    MarkdownSurface as MarkdownSurfaceUi,
    MarkdownTOC as MarkdownTOCUi,
    MarkdownContent as MarkdownContentUi,
    MarkdownLayout as MarkdownLayoutUi,
    RenderedMarkdown,
    TocPosition
};

#[component]
pub fn MarkdownSurface(
    rendered: RenderedMarkdown,
    #[prop(optional)] show_toc: Option<bool>,
    #[prop(optional)] show_toolbar: Option<bool>,
    #[prop(optional)] toc_position: Option<TocPosition>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let toc_pos = toc_position.unwrap_or(TocPosition::Top);
    view! {
        <MarkdownSurfaceUi
            rendered=rendered
            show_toc=show_toc.unwrap_or(true)
            show_toolbar=show_toolbar.unwrap_or(true)
            toc_position=toc_pos
            id=id.unwrap_or_default()
        />
    }
}

#[component]
pub fn MarkdownLayout(
    children: Children,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional)] toc_position: Option<TocPosition>,
) -> impl IntoView {
    let toc_pos = toc_position.unwrap_or(TocPosition::Top);
    view! {
        <MarkdownLayoutUi value=value.unwrap_or_default() toc_position=toc_pos>
            {children()}
        </MarkdownLayoutUi>
    }
}

#[component]
pub fn MarkdownContent(
    rendered: RenderedMarkdown,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! { <MarkdownContentUi rendered=rendered id=id.unwrap_or_default() /> }
}

#[component]
pub fn MarkdownTOC(
    toc: Vec<canonrs_core::TocItem>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] scroll_target: Option<String>,
) -> impl IntoView {
    view! { <MarkdownTOCUi toc=toc id=id.unwrap_or_default() scroll_target=scroll_target.unwrap_or_default() /> }
}
