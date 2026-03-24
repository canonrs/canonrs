//! @canon-level: strict
//! @canon-owner: primitives-team
//! Markdown Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn MarkdownPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-markdown=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn MarkdownToolbarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-markdown-toolbar=""
            role="toolbar"
            aria-label="Markdown toolbar"
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn MarkdownToolbarItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] action: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-markdown-toolbar-item=""
            data-rs-action=action
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn MarkdownTocPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = "closed".to_string())] state: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-markdown-toc=""
            data-rs-state=state
            aria-label="Table of contents"
            class=class
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn MarkdownTocItemPrimitive(
    #[prop(into)] href: String,
    #[prop(into)] text: String,
    #[prop(default = 2u8)] level: u8,
) -> impl IntoView {
    view! {
        <li
            data-rs-markdown-toc-item=""
            data-rs-level=level.to_string()
            data-rs-state="inactive"
        >
            <a
                data-rs-markdown-toc-link=""
                href=href
            >
                {text}
            </a>
        </li>
    }
}

#[component]
pub fn MarkdownContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-markdown-content=""
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}
