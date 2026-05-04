//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pagination Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};

#[component]
pub fn PaginationPrimitive(
    children: Children,
    #[prop(default = 1usize)] current_page: usize,
    #[prop(default = 1usize)] total_pages: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_pg = crate::infra::uid::generate("pg");
    view! {
        <nav
            data-rs-pagination=""
            data-rs-current-page=current_page.to_string()
            data-rs-total-pages=total_pages.to_string()
            data-rs-uid=uid_pg
            data-rs-interaction="nav"
            aria-label="Page navigation"
            class=class
        >
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
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(default = 0usize)] page: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let is_active = state == ActivityState::Active;
    view! {
        <a
            data-rs-pagination-link=""
            data-rs-page=page.to_string()
            data-rs-activity=state.as_str()
            aria-current=if is_active { Some("page") } else { None }
            href=href
            class=class
        >
            {children()}
        </a>
    }
}

#[component]
pub fn PaginationPreviousPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a
            data-rs-pagination-previous=""
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-disabled=disabled.aria_disabled()
            aria-label="Go to previous page"
            href=if disabled.disabled() { "#".to_string() } else { href }
            tabindex=if disabled.disabled() { "-1" } else { "0" }
            class=class
        >
            {children()}
        </a>
    }
}

#[component]
pub fn PaginationNextPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a
            data-rs-pagination-next=""
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-disabled=disabled.aria_disabled()
            aria-label="Go to next page"
            href=if disabled.disabled() { "#".to_string() } else { href }
            tabindex=if disabled.disabled() { "-1" } else { "0" }
            class=class
        >
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
