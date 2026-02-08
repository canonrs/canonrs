//! @canon-level: strict
//! @canon-owner: primitives-team
//! Markdown Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn MarkdownPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-markdown=""
            class={class}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn MarkdownTocPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            attr:data-md-toc=""
            class={class}
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn MarkdownContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-md-content=""
            class={class}
        >
            {children.map(|c| c())}
        </div>
    }
}
