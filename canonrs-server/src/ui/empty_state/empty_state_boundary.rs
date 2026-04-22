//! EmptyState Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::empty_state_ui::{
    EmptyState as EmptyStateUi,
    EmptyStateIcon as EmptyStateIconUi,
    EmptyStateTitle as EmptyStateTitleUi,
    EmptyStateDescription as EmptyStateDescriptionUi,
    EmptyStateAction as EmptyStateActionUi
};
pub use canonrs_core::primitives::EmptyStateVariant;

#[component]
pub fn EmptyState(
    children: Children,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateUi variant=variant class=class>{children()}</EmptyStateUi> }
}

#[component]
pub fn EmptyStateIcon(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateIconUi class=class>{children()}</EmptyStateIconUi> }
}

#[component]
pub fn EmptyStateTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateTitleUi class=class>{children()}</EmptyStateTitleUi> }
}

#[component]
pub fn EmptyStateDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateDescriptionUi class=class>{children()}</EmptyStateDescriptionUi> }
}

#[component]
pub fn EmptyStateAction(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyStateActionUi class=class>{children()}</EmptyStateActionUi> }
}
