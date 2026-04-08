//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pagination Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};
use crate::infra::state_engine::{activity_attrs, disabled_attrs};

#[component]
pub fn PaginationPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-pagination=""
            data-rs-interaction="nav"
            data-rs-component="Pagination"
            data-rs-behavior="navigation"
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let a = activity_attrs(state);
    let is_active = state == ActivityState::Active;
    view! {
        <a
            data-rs-pagination-link=""
            data-rs-state=a.data_rs_state
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
    let d = disabled_attrs(disabled);
    view! {
        <a
            data-rs-pagination-previous=""
            data-rs-disabled=d.data_rs_disabled
            aria-disabled=d.aria_disabled
            aria-label="Go to previous page"
            href=if d.disabled { "#".to_string() } else { href }
            tabindex=if d.disabled { "-1" } else { "0" }
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
    let d = disabled_attrs(disabled);
    view! {
        <a
            data-rs-pagination-next=""
            data-rs-disabled=d.data_rs_disabled
            aria-disabled=d.aria_disabled
            aria-label="Go to next page"
            href=if d.disabled { "#".to_string() } else { href }
            tabindex=if d.disabled { "-1" } else { "0" }
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
