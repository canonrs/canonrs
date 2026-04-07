use leptos::prelude::*;
use super::empty_state_ui::{
    EmptyState, EmptyStateIcon, EmptyStateTitle,
    EmptyStateDescription, EmptyStateAction, EmptyStateVariant,
};

#[component]
pub fn EmptyStateIsland(
    children: Children,
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant_val = match variant.as_deref() {
        Some("no-data")    => EmptyStateVariant::NoData,
        Some("no-results") => EmptyStateVariant::NoResults,
        Some("error")      => EmptyStateVariant::Error,
        _                  => EmptyStateVariant::Default,
    };
    view! {
        <EmptyState variant=variant_val class=class.unwrap_or_default()>
            {children()}
        </EmptyState>
    }
}

#[component]
pub fn EmptyStateIconIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <EmptyStateIcon class=class.unwrap_or_default()>{children()}</EmptyStateIcon> }
}

#[component]
pub fn EmptyStateTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <EmptyStateTitle class=class.unwrap_or_default()>{children()}</EmptyStateTitle> }
}

#[component]
pub fn EmptyStateDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <EmptyStateDescription class=class.unwrap_or_default()>{children()}</EmptyStateDescription> }
}

#[component]
pub fn EmptyStateActionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <EmptyStateAction class=class.unwrap_or_default()>{children()}</EmptyStateAction> }
}
