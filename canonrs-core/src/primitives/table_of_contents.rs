//! TableOfContents Primitives - Semantic structural wrappers
//! Pure SSR components, zero state, zero effects

use leptos::prelude::*;

// ── Nav Container ────────────────────────────────────────────────────────────

#[component]
pub fn TocPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] data_toc_mode: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-toc=""
            data-rs-mode=data_toc_mode
            id={id}
            class={class}
            aria-label="Table of contents"
        >
            {children.map(|c| c())}
        </nav>
    }
}

// ── Title ────────────────────────────────────────────────────────────────────

#[component]
pub fn TocTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-toc-title="" class={class}>
            {children.map(|c| c())}
        </p>
    }
}

// ── List ─────────────────────────────────────────────────────────────────────

#[component]
pub fn TocListPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ul data-rs-toc-list="" class={class}>
            {children.map(|c| c())}
        </ul>
    }
}

// ── Subtree List ─────────────────────────────────────────────────────────────

#[component]
pub fn TocSubtreePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] data_state: String,
) -> impl IntoView {
    view! {
        <ul
            data-rs-toc-subtree=""
            data-rs-state=data_state
            class={class}
        >
            {children.map(|c| c())}
        </ul>
    }
}

// ── Item ─────────────────────────────────────────────────────────────────────

#[component]
pub fn TocItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into)] data_level: String,
    #[prop(into)] data_target: String,
    #[prop(into, default = "idle".to_string())] data_state: String,
    #[prop(into, default = String::new())] data_child: String,
    #[prop(into, default = String::new())] data_has_children: String,
) -> impl IntoView {
    view! {
        <li
            data-rs-toc-item=""
            data-rs-level=data_level
            data-rs-target=data_target
            data-rs-state=data_state
            data-rs-child=data_child
            data-rs-has-children=data_has_children
            class={class}
        >
            {children.map(|c| c())}
        </li>
    }
}

// ── Link ─────────────────────────────────────────────────────────────────────

#[component]
pub fn TocLinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a data-rs-toc-link="" href={href} class={class}>
            {children.map(|c| c())}
        </a>
    }
}

// ── Expand Button (for nested mode) ──────────────────────────────────────────

#[component]
pub fn TocExpandButtonPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = "false".to_string())] aria_expanded: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-toc-expand-btn=""
            aria-expanded={aria_expanded}
            class={class}
        >
            {children.map(|c| c()).unwrap_or_else(|| "›".into_any())}
        </button>
    }
}
