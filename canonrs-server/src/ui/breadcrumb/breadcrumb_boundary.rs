//! @canon-level: strict
//! Breadcrumb Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::breadcrumb_ui::{
    Breadcrumb as BreadcrumbUi,
    BreadcrumbItem as BreadcrumbItemUi,
    BreadcrumbLink as BreadcrumbLinkUi,
    BreadcrumbPage as BreadcrumbPageUi,
    BreadcrumbSeparator as BreadcrumbSeparatorUi,
    BreadcrumbEllipsis as BreadcrumbEllipsisUi
};
use canonrs_core::meta::ActivityState;



#[component]
pub fn Breadcrumb(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BreadcrumbUi class=class.unwrap_or_default()>{children()}</BreadcrumbUi>
    }
}

#[component]
pub fn BreadcrumbItem(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbItemUi class=class.unwrap_or_default()>{children()}</BreadcrumbItemUi> }
}

#[component]
pub fn BreadcrumbLink(
    children: Children,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BreadcrumbLinkUi href=href.unwrap_or_default() state=state class=class.unwrap_or_default()>
            {children()}
        </BreadcrumbLinkUi>
    }
}

#[component]
pub fn BreadcrumbPage(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbPageUi class=class.unwrap_or_default()>{children()}</BreadcrumbPageUi> }
}

#[component]
pub fn BreadcrumbSeparator(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbSeparatorUi class=class.unwrap_or_default()>{children()}</BreadcrumbSeparatorUi> }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <BreadcrumbEllipsisUi class=class.unwrap_or_default() /> }
}
