//! @canon-level: strict
//! @canon-owner: primitives-team
//! PageHeader Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn PageHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <header
            data-page-header=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </header>
    }
}

#[component]
pub fn PageHeaderBreadcrumbsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav
            data-page-header-breadcrumbs=""
            attr:aria-label="Breadcrumb"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn PageHeaderContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-page-header-content=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn PageHeaderTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <h1
            data-page-header-title=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </h1>
    }
}

#[component]
pub fn PageHeaderDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <p
            data-page-header-description=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn PageHeaderActionsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-page-header-actions=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn PageHeaderTabsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-page-header-tabs=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
