//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pagination Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn PaginationPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav data-rs-pagination="" aria-label="Page navigation" class=class>
            {children()}
        </nav>
    }
}

#[component]
pub fn PaginationContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ul data-rs-pagination-content="" class=class>
            {children()}
        </ul>
    }
}

#[component]
pub fn PaginationItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <li data-rs-pagination-item="" class=class>
            {children()}
        </li>
    }
}

#[component]
pub fn PaginationLinkPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] is_active: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if is_active { "active" } else { "idle" };
    let aria = if is_active { Some("page") } else { None };
    view! {
        <a data-rs-pagination-link="" data-rs-state=state aria-current=aria href=href class=class>
            {children()}
        </a>
    }
}

#[component]
pub fn PaginationPreviousPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if disabled { "disabled" } else { "idle" };
    let aria_disabled = if disabled { Some("true") } else { None };
    view! {
        <a data-rs-pagination-previous="" data-rs-state=state aria-disabled=aria_disabled aria-label="Go to previous page" href=href class=class>
            {children()}
        </a>
    }
}

#[component]
pub fn PaginationNextPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if disabled { "disabled" } else { "idle" };
    let aria_disabled = if disabled { Some("true") } else { None };
    view! {
        <a data-rs-pagination-next="" data-rs-state=state aria-disabled=aria_disabled aria-label="Go to next page" href=href class=class>
            {children()}
        </a>
    }
}

#[component]
pub fn PaginationEllipsisPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-pagination-ellipsis="" aria-hidden="true" class=class>
            "\u{2026}"
            <span class="sr-only">"More pages"</span>
        </span>
    }
}
