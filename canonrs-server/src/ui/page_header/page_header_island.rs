use leptos::prelude::*;
use super::page_header_ui::{
    PageHeader, PageHeaderBreadcrumbs, PageHeaderContent,
    PageHeaderTitle, PageHeaderDescription, PageHeaderActions, PageHeaderTabs,
};

#[component]
pub fn PageHeaderIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeader class=class>{children()}</PageHeader> }
}

#[component]
pub fn PageHeaderBreadcrumbsIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderBreadcrumbs class=class>{children()}</PageHeaderBreadcrumbs> }
}

#[component]
pub fn PageHeaderContentIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderContent class=class>{children()}</PageHeaderContent> }
}

#[component]
pub fn PageHeaderTitleIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderTitle class=class>{children()}</PageHeaderTitle> }
}

#[component]
pub fn PageHeaderDescriptionIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderDescription class=class>{children()}</PageHeaderDescription> }
}

#[component]
pub fn PageHeaderActionsIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderActions class=class>{children()}</PageHeaderActions> }
}

#[component]
pub fn PageHeaderTabsIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderTabs class=class>{children()}</PageHeaderTabs> }
}
