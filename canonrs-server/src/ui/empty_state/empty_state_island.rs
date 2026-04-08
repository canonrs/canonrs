//! EmptyState Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::empty_state_ui::{EmptyState, EmptyStateIcon, EmptyStateTitle, EmptyStateDescription, EmptyStateAction};
use canonrs_core::primitives::EmptyStateVariant;

#[component]
pub fn EmptyStateIsland(
    children: Children,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyState variant=variant class=class>{children()}</EmptyState> }
}

#[component]
pub fn EmptyStateIconIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateIcon class=class>{children()}</EmptyStateIcon> }
}

#[component]
pub fn EmptyStateTitleIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateTitle class=class>{children()}</EmptyStateTitle> }
}

#[component]
pub fn EmptyStateDescriptionIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateDescription class=class>{children()}</EmptyStateDescription> }
}

#[component]
pub fn EmptyStateActionIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateAction class=class>{children()}</EmptyStateAction> }
}
