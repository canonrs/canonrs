#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::meta::{ActivityState, DisabledState};
use canonrs_core::primitives::{
    PaginationPrimitive, PaginationContentPrimitive, PaginationItemPrimitive,
    PaginationLinkPrimitive, PaginationPreviousPrimitive, PaginationNextPrimitive,
    PaginationEllipsisPrimitive,
};

#[component]
pub fn Pagination(
    children: Children,
    #[prop(default = 1usize)] current_page: usize,
    #[prop(default = 1usize)] total_pages: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationPrimitive current_page=current_page total_pages=total_pages class=class>
            {children()}
        </PaginationPrimitive>
    }
}

#[component]
pub fn PaginationContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationContentPrimitive class=class>
            {children()}
        </PaginationContentPrimitive>
    }
}

#[component]
pub fn PaginationItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationItemPrimitive class=class>
            {children()}
        </PaginationItemPrimitive>
    }
}

#[component]
pub fn PaginationLink(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(default = 0usize)] page: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationLinkPrimitive href=href state=state page=page class=class>
            {children()}
        </PaginationLinkPrimitive>
    }
}

#[component]
pub fn PaginationPrevious(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationPreviousPrimitive href=href disabled=disabled class=class>
            {children()}
        </PaginationPreviousPrimitive>
    }
}

#[component]
pub fn PaginationNext(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationNextPrimitive href=href disabled=disabled class=class>
            {children()}
        </PaginationNextPrimitive>
    }
}

#[component]
pub fn PaginationEllipsis(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationEllipsisPrimitive class=class />
    }
}

#[component]
pub fn PaginationPreview() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious href="#">"\u{2190}"</PaginationPrevious>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#" state=ActivityState::Active>"1"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#">"2"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href="#">"\u{2192}"</PaginationNext>
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
