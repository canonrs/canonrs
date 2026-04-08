//! @canon-level: strict
//! @canon-owner: primitives-team
//! Markdown Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::{NavigationState, VisibilityState};
use crate::infra::state_engine::visibility_attrs;

#[component]
pub fn MarkdownPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] inner: String,
) -> impl IntoView {
    // inner contém o HTML do layout completo (TOC + content)
    // É injetado via inner_html apenas em SSR — hydration via MarkdownContentPrimitive
    #[cfg(feature = "ssr")]
    {
        view! {
            <div
                data-rs-markdown=""
            data-rs-interaction="content"
                data-rs-component="Markdown"
                data-rs-behavior="content"
                class=class
                inner_html=inner
            ></div>
        }.into_any()
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = inner;
        view! {
            <div
                data-rs-markdown=""
                data-rs-component="Markdown"
                data-rs-behavior="content"
                class=class
            ></div>
        }.into_any()
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
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let va = visibility_attrs(state);
    view! {
        <nav
            data-rs-markdown-toc=""
            data-rs-state=va.data_rs_state
            aria-hidden=va.aria_hidden
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
    #[prop(default = NavigationState::Inactive)] state: NavigationState,
) -> impl IntoView {
    let aria_current = if state == NavigationState::Current { Some("page") } else { None };
    view! {
        <li
            data-rs-markdown-toc-item=""
            data-rs-level=level.to_string()
            data-rs-state=state.as_str()
        >
            <a
                data-rs-markdown-toc-link=""
                aria-current=aria_current
                href=href
            >{text}</a>
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
            inner_html=inner.clone()
        ></div>
    }
}
