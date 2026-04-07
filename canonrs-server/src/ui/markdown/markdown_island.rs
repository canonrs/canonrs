#[island]
pub fn MarkdownInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        let f = Closure::wrap(Box::new(move || {
            crate::interactions::markdown::init_all();
        }) as Box<dyn Fn()>);
        leptos::web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .ok();
        f.forget();
    }
    view! { <></> }
}

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
        <MarkdownInit />
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
