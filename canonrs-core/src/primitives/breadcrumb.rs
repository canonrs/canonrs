//! @canon-level: strict
//! @canon-owner: primitives-team
//! Breadcrumb Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn BreadcrumbPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav data-rs-breadcrumb="" aria-label="Breadcrumb" class=class>
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
    #[prop(default = false)] current: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a
            data-rs-breadcrumb-link=""
            data-rs-state={if current { "current" } else { "inactive" }}
            href=href
            aria-current={if current { Some("page") } else { None }}
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
