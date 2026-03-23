//! @canon-level: strict
//! @canon-owner: primitives-team
//! PageHeader Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn PageHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <header
            data-rs-page-header=""
            class={class}
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </header>
    }
}

#[component]
pub fn PageHeaderBreadcrumbsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <nav
            data-rs-page-header-breadcrumbs=""
            attr:aria-label="Breadcrumb"
            class={class}
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn PageHeaderContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-page-header-content=""
            class={class}
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn PageHeaderTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <h1
            data-rs-page-header-title=""
            class={class}
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </h1>
    }
}

#[component]
pub fn PageHeaderDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <p
            data-rs-page-header-description=""
            class={class}
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn PageHeaderActionsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-page-header-actions=""
            class={class}
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn PageHeaderTabsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-page-header-tabs=""
            class={class}
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </div>
    }
}
