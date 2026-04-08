//! @canon-level: strict
//! Pagination Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::pagination_ui::{
    Pagination, PaginationContent, PaginationItem,
    PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis,
};
use canonrs_core::meta::{ActivityState, DisabledState};

#[island]
pub fn PaginationInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn PaginationIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PaginationInit />
        <Pagination class=class>{children()}</Pagination>
    }
}

#[component]
pub fn PaginationContentIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationContent class=class>{children()}</PaginationContent> }
}

#[component]
pub fn PaginationItemIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationItem class=class>{children()}</PaginationItem> }
}

#[component]
pub fn PaginationLinkIsland(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationLink href=href state=state class=class>{children()}</PaginationLink> }
}

#[component]
pub fn PaginationPreviousIsland(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationPrevious href=href disabled=disabled class=class>{children()}</PaginationPrevious> }
}

#[component]
pub fn PaginationNextIsland(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationNext href=href disabled=disabled class=class>{children()}</PaginationNext> }
}

#[component]
pub fn PaginationEllipsisIsland(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PaginationEllipsis class=class /> }
}
