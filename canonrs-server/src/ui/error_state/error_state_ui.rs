//! @canon-id: error-state
//! @canon-label: Error State
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Display error condition to user
//! @canon-description: Error state display
//! @canon-composable: true
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts: ErrorStateIcon, ErrorStateTitle, ErrorStateDescription, ErrorStateAction
//! @canon-tags: error-state, error, failure, problem, try-again

use leptos::prelude::*;
use canonrs_core::primitives::{
    ErrorStatePrimitive, ErrorStateIconPrimitive,
    ErrorStateTitlePrimitive, ErrorStateDescriptionPrimitive,
    ErrorStateActionsPrimitive,
};

#[component]
pub fn ErrorState(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ErrorStatePrimitive class={class.unwrap_or_default()}>
            {children()}
        </ErrorStatePrimitive>
    }
}

#[component]
pub fn ErrorStateIcon(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ErrorStateIconPrimitive class={class.unwrap_or_default()}>
            {children()}
        </ErrorStateIconPrimitive>
    }
}

#[component]
pub fn ErrorStateTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ErrorStateTitlePrimitive class={class.unwrap_or_default()}>
            {children()}
        </ErrorStateTitlePrimitive>
    }
}

#[component]
pub fn ErrorStateDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ErrorStateDescriptionPrimitive class={class.unwrap_or_default()}>
            {children()}
        </ErrorStateDescriptionPrimitive>
    }
}

#[component]
pub fn ErrorStateActions(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ErrorStateActionsPrimitive class={class.unwrap_or_default()}>
            {children()}
        </ErrorStateActionsPrimitive>
    }
}

#[component]
pub fn ErrorStatePreview() -> impl IntoView {
    view! {
        <ErrorState>
            <ErrorStateTitle>"Something went wrong"</ErrorStateTitle>
            <ErrorStateDescription>"We encountered an error. Please try again."</ErrorStateDescription>
        </ErrorState>
    }
}
