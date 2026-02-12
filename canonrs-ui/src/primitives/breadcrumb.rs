use leptos::prelude::*;

#[component]
pub fn BreadcrumbPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav
            data-breadcrumb=""
            aria-label="Breadcrumb"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn BreadcrumbItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-breadcrumb-item=""
            class={class}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn BreadcrumbLinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = false)] current: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a data-breadcrumb-link="" href={href} aria-current={if current { "page" } else { "" }} class={class}>
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn BreadcrumbSeparatorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-breadcrumb-separator="" aria-hidden="true" class={class}>
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn BreadcrumbEllipsisPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span data-breadcrumb-ellipsis="" aria-hidden="true" class={class} id={id}>
            "..."
        </span>
    }
}
