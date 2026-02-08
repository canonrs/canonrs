use leptos::prelude::*;
use crate::primitives::{
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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("page-header {}", class);

    view! {
        <PageHeaderPrimitive class={base_class} id={id}>
            {children.map(|c| c())}
        </PageHeaderPrimitive>
    }
}

#[component]
pub fn PageHeaderBreadcrumbs(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PageHeaderBreadcrumbsPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PageHeaderBreadcrumbsPrimitive>
    }
}

#[component]
pub fn PageHeaderContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PageHeaderContentPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PageHeaderContentPrimitive>
    }
}

#[component]
pub fn PageHeaderTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PageHeaderTitlePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PageHeaderTitlePrimitive>
    }
}

#[component]
pub fn PageHeaderDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PageHeaderDescriptionPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PageHeaderDescriptionPrimitive>
    }
}

#[component]
pub fn PageHeaderActions(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PageHeaderActionsPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PageHeaderActionsPrimitive>
    }
}

#[component]
pub fn PageHeaderTabs(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PageHeaderTabsPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PageHeaderTabsPrimitive>
    }
}
