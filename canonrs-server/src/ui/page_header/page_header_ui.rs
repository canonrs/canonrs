//! @canon-id: page-header
//! @canon-label: Page Header
//! @canon-family: layout
//! @canon-category: Display
//! @canon-intent: Page title and actions bar
//! @canon-description: Page header with title and actions
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: page-header, title, heading, actions, breadcrumb

use leptos::prelude::*;
use canonrs_core::primitives::{
    PageHeaderPrimitive,
    PageHeaderBreadcrumbsPrimitive,
    PageHeaderContentPrimitive,
    PageHeaderTitlePrimitive,
    PageHeaderDescriptionPrimitive,
    PageHeaderActionsPrimitive,
    PageHeaderTabsPrimitive,
};

#[component]
pub fn PageHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PageHeaderPrimitive class=class>
            {children()}
        </PageHeaderPrimitive>
    }
}

#[component]
pub fn PageHeaderBreadcrumbs(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PageHeaderBreadcrumbsPrimitive class=class>
            {children()}
        </PageHeaderBreadcrumbsPrimitive>
    }
}

#[component]
pub fn PageHeaderContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PageHeaderContentPrimitive class=class>
            {children()}
        </PageHeaderContentPrimitive>
    }
}

#[component]
pub fn PageHeaderTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PageHeaderTitlePrimitive class=class>
            {children()}
        </PageHeaderTitlePrimitive>
    }
}

#[component]
pub fn PageHeaderDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PageHeaderDescriptionPrimitive class=class>
            {children()}
        </PageHeaderDescriptionPrimitive>
    }
}

#[component]
pub fn PageHeaderActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PageHeaderActionsPrimitive class=class>
            {children()}
        </PageHeaderActionsPrimitive>
    }
}

#[component]
pub fn PageHeaderTabs(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PageHeaderTabsPrimitive class=class>
            {children()}
        </PageHeaderTabsPrimitive>
    }
}

#[component]
pub fn PageHeaderPreview() -> impl IntoView {
    view! {
        <PageHeader>
            <PageHeaderContent>
                <PageHeaderTitle>"Page Title"</PageHeaderTitle>
                <PageHeaderDescription>"Page description goes here."</PageHeaderDescription>
            </PageHeaderContent>
        </PageHeader>
    }
}
