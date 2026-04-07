use leptos::prelude::*;
use super::error_state_ui::{
    ErrorState, ErrorStateIcon, ErrorStateTitle,
    ErrorStateDescription, ErrorStateActions,
};

#[component]
pub fn ErrorStateIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorState class=class.unwrap_or_default()>{children()}</ErrorState> }
}

#[component]
pub fn ErrorStateIconIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateIcon class=class.unwrap_or_default()>{children()}</ErrorStateIcon> }
}

#[component]
pub fn ErrorStateTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateTitle class=class.unwrap_or_default()>{children()}</ErrorStateTitle> }
}

#[component]
pub fn ErrorStateDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateDescription class=class.unwrap_or_default()>{children()}</ErrorStateDescription> }
}

#[component]
pub fn ErrorStateActionsIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ErrorStateActions class=class.unwrap_or_default()>{children()}</ErrorStateActions> }
}
