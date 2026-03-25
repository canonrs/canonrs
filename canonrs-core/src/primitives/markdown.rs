//! @canon-level: strict
//! @canon-owner: primitives-team
//! Markdown Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn MarkdownPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, default = String::new())] inner: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-markdown=""
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
            inner_html=inner
        ></div>
    }
}

#[component]
pub fn MarkdownToolbarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-markdown-toolbar=""
            role="toolbar"
            aria-label="Markdown toolbar"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MarkdownToolbarItemPrimitive(
    children: Children,
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
            {children()}
        </button>
    }
}

#[component]
pub fn MarkdownTocPrimitive(
    children: Children,
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
            {children()}
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
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] html: String,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let inner = html;
    #[cfg(not(feature = "ssr"))]
    let inner = { let _ = html; String::new() };

    view! {
        <div
            data-rs-markdown-content=""
            class=class
            inner_html=inner
        ></div>
    }
}
