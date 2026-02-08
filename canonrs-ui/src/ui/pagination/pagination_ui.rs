//! @canon-level: strict
//! @canon-owner: ui-team
//! Pagination UI Component

use leptos::prelude::*;
use crate::primitives::{
    PaginationPrimitive, PaginationContentPrimitive, PaginationItemPrimitive,
    PaginationLinkPrimitive, PaginationPreviousPrimitive, PaginationNextPrimitive,
    PaginationEllipsisPrimitive,
};

#[component]
pub fn Pagination(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PaginationPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PaginationPrimitive>
    }
}

#[component]
pub fn PaginationContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PaginationContentPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PaginationContentPrimitive>
    }
}

#[component]
pub fn PaginationItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PaginationItemPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PaginationItemPrimitive>
    }
}

#[component]
pub fn PaginationLink(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = false)] is_active: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PaginationLinkPrimitive href={href} is_active={is_active} class={class} id={id}>
            {children.map(|c| c())}
        </PaginationLinkPrimitive>
    }
}

#[component]
pub fn PaginationPrevious(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PaginationPreviousPrimitive href={href} class={class} id={id}>
            {children.map(|c| c())}
        </PaginationPreviousPrimitive>
    }
}

#[component]
pub fn PaginationNext(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PaginationNextPrimitive href={href} class={class} id={id}>
            {children.map(|c| c())}
        </PaginationNextPrimitive>
    }
}

#[component]
pub fn PaginationEllipsis(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PaginationEllipsisPrimitive class={class} id={id} />
    }
}
