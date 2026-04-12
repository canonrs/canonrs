use leptos::prelude::*;
use super::page_header_ui::{
    PageHeader as PageHeaderUi,
    PageHeaderBreadcrumbs as PageHeaderBreadcrumbsUi,
    PageHeaderContent as PageHeaderContentUi,
    PageHeaderTitle as PageHeaderTitleUi,
    PageHeaderDescription as PageHeaderDescriptionUi,
    PageHeaderActions as PageHeaderActionsUi,
    PageHeaderTabs as PageHeaderTabsUi
};

#[component]
pub fn PageHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderUi class=class>{children()}</PageHeaderUi> }
}

#[component]
pub fn PageHeaderBreadcrumbs(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderBreadcrumbsUi class=class>{children()}</PageHeaderBreadcrumbsUi> }
}

#[component]
pub fn PageHeaderContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderContentUi class=class>{children()}</PageHeaderContentUi> }
}

#[component]
pub fn PageHeaderTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderTitleUi class=class>{children()}</PageHeaderTitleUi> }
}

#[component]
pub fn PageHeaderDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderDescriptionUi class=class>{children()}</PageHeaderDescriptionUi> }
}

#[component]
pub fn PageHeaderActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderActionsUi class=class>{children()}</PageHeaderActionsUi> }
}

#[component]
pub fn PageHeaderTabs(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PageHeaderTabsUi class=class>{children()}</PageHeaderTabsUi> }
}
