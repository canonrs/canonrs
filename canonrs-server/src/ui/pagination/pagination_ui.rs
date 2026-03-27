//! @canon-id: pagination
//! @canon-label: Pagination
//! @canon-family: navigation
//! @canon-category: Navigation
//! @canon-intent: Navigate between pages of content
//! @canon-description: Page navigation control
//! @canon-composable: true
//! @canon-capabilities: Active, Disabled
//! @canon-required-parts:
//! @canon-optional-parts: PaginationContent, PaginationItem, PaginationLink
//! @canon-tags: pagination, pages, next, prev, navigate

use leptos::prelude::*;
use canonrs_core::primitives::{
    PaginationPrimitive, PaginationContentPrimitive, PaginationItemPrimitive,
    PaginationLinkPrimitive, PaginationPreviousPrimitive, PaginationNextPrimitive,
    PaginationEllipsisPrimitive,
};

#[component]
pub fn Pagination(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationPrimitive class=class>
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
    #[prop(default = false)] is_active: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationLinkPrimitive href=href state=is_active.into() class=class>
            {children()}
        </PaginationLinkPrimitive>
    }
}

#[component]
pub fn PaginationPrevious(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationPreviousPrimitive href=href disabled=disabled.into() class=class>
            {children()}
        </PaginationPreviousPrimitive>
    }
}

#[component]
pub fn PaginationNext(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationNextPrimitive href=href disabled=disabled.into() class=class>
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
                    <PaginationLink href="#" is_active=true>"1"</PaginationLink>
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
