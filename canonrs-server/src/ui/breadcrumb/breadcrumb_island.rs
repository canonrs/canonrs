//! @canon-level: strict
//! Breadcrumb Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::breadcrumb_ui::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbPage, BreadcrumbSeparator, BreadcrumbEllipsis,
};
use canonrs_core::meta::ActivityState;

#[island]
pub fn BreadcrumbInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn BreadcrumbIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BreadcrumbInit />
        <Breadcrumb class=class.unwrap_or_default()>{children()}</Breadcrumb>
    }
}

#[component]
pub fn BreadcrumbItemIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbItem class=class.unwrap_or_default()>{children()}</BreadcrumbItem> }
}

#[component]
pub fn BreadcrumbLinkIsland(
    children: Children,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BreadcrumbLink href=href.unwrap_or_default() state=state class=class.unwrap_or_default()>
            {children()}
        </BreadcrumbLink>
    }
}

#[component]
pub fn BreadcrumbPageIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbPage class=class.unwrap_or_default()>{children()}</BreadcrumbPage> }
}

#[component]
pub fn BreadcrumbSeparatorIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbSeparator class=class.unwrap_or_default()>{children()}</BreadcrumbSeparator> }
}

#[component]
pub fn BreadcrumbEllipsisIsland(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbEllipsis class=class.unwrap_or_default() /> }
}
