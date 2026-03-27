//! @canon-id: empty-state
//! @canon-label: Empty State
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Display when no content is available
//! @canon-description: Empty state placeholder
//! @canon-composable: true
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts: EmptyStateIcon, EmptyStateTitle, EmptyStateDescription, EmptyStateAction
//! @canon-tags: empty-state, empty, no-data, placeholder, zero-state

use leptos::prelude::*;
use canonrs_core::primitives::{
    EmptyStatePrimitive, EmptyStateIconPrimitive,
    EmptyStateTitlePrimitive, EmptyStateDescriptionPrimitive,
    EmptyStateActionPrimitive,
};
pub use canonrs_core::primitives::EmptyStateVariant;

#[component]
pub fn EmptyState(
    children: Children,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <EmptyStatePrimitive variant=variant class={class.unwrap_or_default()}>
            {children()}
        </EmptyStatePrimitive>
    }
}

#[component]
pub fn EmptyStateIcon(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <EmptyStateIconPrimitive class={class.unwrap_or_default()}>
            {children()}
        </EmptyStateIconPrimitive>
    }
}

#[component]
pub fn EmptyStateTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <EmptyStateTitlePrimitive class={class.unwrap_or_default()}>
            {children()}
        </EmptyStateTitlePrimitive>
    }
}

#[component]
pub fn EmptyStateDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <EmptyStateDescriptionPrimitive class={class.unwrap_or_default()}>
            {children()}
        </EmptyStateDescriptionPrimitive>
    }
}

#[component]
pub fn EmptyStateAction(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <EmptyStateActionPrimitive class={class.unwrap_or_default()}>
            {children()}
        </EmptyStateActionPrimitive>
    }
}

#[component]
pub fn EmptyStatePreview() -> impl IntoView {
    view! {
        <EmptyState>
            <EmptyStateTitle>"No items found"</EmptyStateTitle>
            <EmptyStateDescription>"Try adjusting your search or filters."</EmptyStateDescription>
        </EmptyState>
    }
}
