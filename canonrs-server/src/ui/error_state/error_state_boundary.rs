use leptos::prelude::*;
use super::error_state_ui::{
    ErrorState as ErrorStateUi,
    ErrorStateIcon as ErrorStateIconUi,
    ErrorStateTitle as ErrorStateTitleUi,
    ErrorStateDescription as ErrorStateDescriptionUi,
    ErrorStateActions as ErrorStateActionsUi
};

#[component]
pub fn ErrorState(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateUi class=class.unwrap_or_default()>{children()}</ErrorStateUi> }
}

#[component]
pub fn ErrorStateIcon(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateIconUi class=class.unwrap_or_default()>{children()}</ErrorStateIconUi> }
}

#[component]
pub fn ErrorStateTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateTitleUi class=class.unwrap_or_default()>{children()}</ErrorStateTitleUi> }
}

#[component]
pub fn ErrorStateDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateDescriptionUi class=class.unwrap_or_default()>{children()}</ErrorStateDescriptionUi> }
}

#[component]
pub fn ErrorStateActions(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateActionsUi class=class.unwrap_or_default()>{children()}</ErrorStateActionsUi> }
}
