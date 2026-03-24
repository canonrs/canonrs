//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pagination Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn PaginationPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <nav data-rs-pagination="" aria-label="Page navigation" class=class id=id>
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn PaginationContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ul data-rs-pagination-content="" class=class id=id>
            {children.map(|c| c())}
        </ul>
    }
}

#[component]
pub fn PaginationItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <li data-rs-pagination-item="" class=class id=id>
            {children.map(|c| c())}
        </li>
    }
}

#[component]
pub fn PaginationLinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] is_active: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let aria_current = if is_active { Some("page") } else { None };
    view! {
        <a
            data-rs-pagination-link=""
            data-rs-state={if is_active { "active" } else { "inactive" }}
            aria-current=aria_current
            href=href
            class=class
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationPreviousPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let aria_label = "Go to previous page";
    view! {
        <a
            data-rs-pagination-previous=""
            data-rs-state={move || if disabled.get() { "disabled" } else { "default" }}
            aria-disabled={move || if disabled.get() { "true" } else { "false" }}
            aria-label=aria_label
            href=href
            class=class
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationNextPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let aria_label = "Go to next page";
    view! {
        <a
            data-rs-pagination-next=""
            data-rs-state={move || if disabled.get() { "disabled" } else { "default" }}
            aria-disabled={move || if disabled.get() { "true" } else { "false" }}
            aria-label=aria_label
            href=href
            class=class
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationEllipsisPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span data-rs-pagination-ellipsis="" aria-hidden="true" class=class id=id>
            "…"
            <span class="sr-only">"More pages"</span>
        </span>
    }
}
