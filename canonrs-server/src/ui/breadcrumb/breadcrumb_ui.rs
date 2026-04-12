#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{
    BreadcrumbPrimitive, BreadcrumbItemPrimitive, BreadcrumbLinkPrimitive,
    BreadcrumbSeparatorPrimitive, BreadcrumbEllipsisPrimitive,
};
use canonrs_core::meta::ActivityState;

#[component]
pub fn Breadcrumb(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BreadcrumbPrimitive class=class>
            {children()}
        </BreadcrumbPrimitive>
    }
}

#[component]
pub fn BreadcrumbItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BreadcrumbItemPrimitive class=class>
            {children()}
        </BreadcrumbItemPrimitive>
    }
}

#[component]
pub fn BreadcrumbLink(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BreadcrumbLinkPrimitive href=href state=state class=class>
            {children()}
        </BreadcrumbLinkPrimitive>
    }
}

#[component]
pub fn BreadcrumbPage(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-breadcrumb-page="" aria-current="page" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BreadcrumbSeparatorPrimitive class=class>
            {children()}
        </BreadcrumbSeparatorPrimitive>
    }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BreadcrumbItemPrimitive class=String::new()>
            <BreadcrumbEllipsisPrimitive class=class />
        </BreadcrumbItemPrimitive>
    }
}

#[component]
pub fn BreadcrumbPreview() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbItem>
                <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbLink href="#">"Components"</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
            </BreadcrumbItem>
        </Breadcrumb>
    }
}
