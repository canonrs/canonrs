//! @canon-level: ui
//! ErrorState - Declarative UI wrapper

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
