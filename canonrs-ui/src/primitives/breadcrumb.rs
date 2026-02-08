use leptos::prelude::*;

#[component]
pub fn BreadcrumbPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav
            attr:data-breadcrumb=""
            attr:aria-label="Breadcrumb"
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
            attr:data-breadcrumb-item=""
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
        <a attr:data-breadcrumb-link="" href={href} attr:aria-current={if current { "page" } else { "" }} class={class}>
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
        <span attr:data-breadcrumb-separator="" attr:aria-hidden="true" class={class}>
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
        <span attr:data-breadcrumb-ellipsis="" attr:aria-hidden="true" class={class} id={id}>
            "..."
        </span>
    }
}
