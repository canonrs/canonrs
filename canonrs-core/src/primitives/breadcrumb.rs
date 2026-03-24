//! @canon-level: strict
//! @canon-owner: primitives-team
//! Breadcrumb Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn BreadcrumbPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <nav data-rs-breadcrumb="" aria-label="Breadcrumb" class=class id=id>
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn BreadcrumbItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-breadcrumb-item="" class=class>
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn BreadcrumbLinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] current: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a
            data-rs-breadcrumb-link=""
            data-rs-current={if current { "true" } else { "false" }}
            href=href
            aria-current={if current { Some("page") } else { None }}
            class=class
        >
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn BreadcrumbSeparatorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-breadcrumb-separator="" aria-hidden="true" class=class>
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn BreadcrumbEllipsisPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span data-rs-breadcrumb-ellipsis="" aria-hidden="true" class=class id=id>
            "..."
        </span>
    }
}
