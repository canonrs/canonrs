//! @canon-level: strict
//! HoverCard Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::hover_card_ui::{HoverCard, HoverCardTrigger, HoverCardContent};
use canonrs_core::meta::VisibilityState;

#[island]
pub fn HoverCardInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn HoverCardIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <HoverCardInit />
        <HoverCard state=VisibilityState::Closed class=class.unwrap_or_default()>
            {children()}
        </HoverCard>
    }
}

#[component]
pub fn HoverCardTriggerIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HoverCardTrigger class=class.unwrap_or_default()>{children()}</HoverCardTrigger> }
}

#[component]
pub fn HoverCardContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HoverCardContent class=class.unwrap_or_default()>{children()}</HoverCardContent> }
}
