use leptos::prelude::*;
use super::page_header_ui::{
    PageHeader, PageHeaderBreadcrumbs, PageHeaderContent,
    PageHeaderTitle, PageHeaderDescription, PageHeaderActions, PageHeaderTabs,
};

#[component]
pub fn PageHeaderIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PageHeader class=class.unwrap_or_default()>{children()}</PageHeader> }
}

#[component]
pub fn PageHeaderBreadcrumbsIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PageHeaderBreadcrumbs class=class.unwrap_or_default()>{children()}</PageHeaderBreadcrumbs> }
}

#[component]
pub fn PageHeaderContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PageHeaderContent class=class.unwrap_or_default()>{children()}</PageHeaderContent> }
}

#[component]
pub fn PageHeaderTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PageHeaderTitle class=class.unwrap_or_default()>{children()}</PageHeaderTitle> }
}

#[component]
pub fn PageHeaderDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PageHeaderDescription class=class.unwrap_or_default()>{children()}</PageHeaderDescription> }
}

#[component]
pub fn PageHeaderActionsIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PageHeaderActions class=class.unwrap_or_default()>{children()}</PageHeaderActions> }
}

#[component]
pub fn PageHeaderTabsIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PageHeaderTabs class=class.unwrap_or_default()>{children()}</PageHeaderTabs> }
}
