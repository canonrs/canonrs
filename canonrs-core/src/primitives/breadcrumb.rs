//! @canon-level: strict
//! @canon-owner: primitives-team
//! Breadcrumb Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::ActivityState;

#[component]
pub fn BreadcrumbPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_bc = crate::infra::uid::generate("bc");
    view! {
        <nav
            data-rs-breadcrumb=""
            data-rs-uid=uid_bc
            data-rs-interaction="nav"
            aria-label="Breadcrumb"
            class=class
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-breadcrumb-item="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbLinkPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let aria_current = if state == ActivityState::Active { Some("page") } else { None };
    view! {
        <a
            data-rs-breadcrumb-link=""
            data-rs-activity=state.as_str()
            href=href
            aria-current=aria_current
            class=class
        >
            {children()}
        </a>
    }
}

#[component]
pub fn BreadcrumbSeparatorPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-breadcrumb-separator="" aria-hidden="true" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbEllipsisPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-breadcrumb-ellipsis="" aria-hidden="true" class=class>
            "..."
        </span>
    }
}
