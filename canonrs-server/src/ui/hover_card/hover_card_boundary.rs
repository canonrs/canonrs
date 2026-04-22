//! @canon-level: strict
//! HoverCard Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::hover_card_ui::{
    HoverCard as HoverCardUi,
    HoverCardTrigger as HoverCardTriggerUi,
    HoverCardContent as HoverCardContentUi
};
pub use canonrs_core::primitives::HoverCardSide;
use canonrs_core::meta::VisibilityState;

#[component]
pub fn HoverCard(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <HoverCardUi state=VisibilityState::Closed class=class.unwrap_or_default()>
            {children()}
        </HoverCardUi>
    }
}

#[component]
pub fn HoverCardTrigger(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HoverCardTriggerUi class=class.unwrap_or_default()>{children()}</HoverCardTriggerUi> }
}

#[component]
pub fn HoverCardContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(default = HoverCardSide::Top)] side: HoverCardSide,
) -> impl IntoView {
    view! { <HoverCardContentUi side=side class=class.unwrap_or_default()>{children()}</HoverCardContentUi> }
}
