//! @canon-level: strict
//! @canon-owner: primitives-team
//! PageHeader Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn PageHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <header
            data-rs-page-header=""
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </header>
    }
}

#[component]
pub fn PageHeaderBreadcrumbsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-page-header-breadcrumbs=""
            aria-label="Breadcrumb"
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn PageHeaderContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-page-header-content=""
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PageHeaderTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h1
            data-rs-page-header-title=""
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </h1>
    }
}

#[component]
pub fn PageHeaderDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p
            data-rs-page-header-description=""
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </p>
    }
}

#[component]
pub fn PageHeaderActionsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-page-header-actions=""
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PageHeaderTabsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-page-header-tabs=""
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </div>
    }
}
