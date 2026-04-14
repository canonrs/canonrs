//! @canon-level: strict
//! @canon-owner: primitives-team
//! Breadcrumb Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::ActivityState;
use crate::infra::state_engine::activity_attrs;

#[component]
pub fn BreadcrumbPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-breadcrumb=""
            data-rs-uid=crate::infra::uid::generate("bc")
            data-rs-interaction="nav"
            data-rs-component="Breadcrumb"
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
    let a = activity_attrs(state);
    let is_current = state == ActivityState::Active;
    view! {
        <a
            data-rs-breadcrumb-link=""
            data-rs-state=a.data_rs_state
            href=href
            aria-current=if is_current { Some("page") } else { None }
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
