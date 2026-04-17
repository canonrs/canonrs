//! @canon-level: strict
//! Pagination Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::pagination_ui::{
    Pagination as PaginationUi,
    PaginationContent as PaginationContentUi,
    PaginationItem as PaginationItemUi,
    PaginationLink as PaginationLinkUi,
    PaginationPrevious as PaginationPreviousUi,
    PaginationNext as PaginationNextUi,
    PaginationEllipsis as PaginationEllipsisUi
};
use canonrs_core::meta::{ActivityState, DisabledState};



#[component]
pub fn Pagination(
    children: Children,
    #[prop(default = 1usize)] current_page: usize,
    #[prop(default = 1usize)] total_pages: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationUi current_page=current_page total_pages=total_pages class=class>{children()}</PaginationUi>
    }
}

#[component]
pub fn PaginationContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationContentUi class=class>{children()}</PaginationContentUi> }
}

#[component]
pub fn PaginationItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationItemUi class=class>{children()}</PaginationItemUi> }
}

#[component]
pub fn PaginationLink(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(default = 0usize)] page: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationLinkUi href=href state=state page=page class=class>{children()}</PaginationLinkUi> }
}

#[component]
pub fn PaginationPrevious(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationPreviousUi href=href disabled=disabled class=class>{children()}</PaginationPreviousUi> }
}

#[component]
pub fn PaginationNext(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationNextUi href=href disabled=disabled class=class>{children()}</PaginationNextUi> }
}

#[component]
pub fn PaginationEllipsis(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationEllipsisUi class=class /> }
}
