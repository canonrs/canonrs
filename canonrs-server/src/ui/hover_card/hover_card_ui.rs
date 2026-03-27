//! @canon-id: hover-card
//! @canon-label: Hover Card
//! @canon-family: overlay
//! @canon-category: Overlay
//! @canon-intent: Show rich preview on hover
//! @canon-description: Hover card popup
//! @canon-composable: true
//! @canon-capabilities: OpenClose
//! @canon-required-parts: HoverCardTrigger, HoverCardContent
//! @canon-optional-parts:
//! @canon-tags: hover-card, preview, card, hover, popup

use leptos::prelude::*;
use canonrs_core::primitives::{
    HoverCardPrimitive, HoverCardTriggerPrimitive, HoverCardContentPrimitive
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn HoverCard(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if open { VisibilityState::Open } else { VisibilityState::Closed };
    view! {
        <HoverCardPrimitive state=state class=class>
            {children()}
        </HoverCardPrimitive>
    }
}

#[component]
pub fn HoverCardTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <HoverCardTriggerPrimitive class=class>
            {children()}
        </HoverCardTriggerPrimitive>
    }
}

#[component]
pub fn HoverCardContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <HoverCardContentPrimitive class=class>
            {children()}
        </HoverCardContentPrimitive>
    }
}

#[component]
pub fn HoverCardPreview() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger>"Hover me"</HoverCardTrigger>
            <HoverCardContent>"Card content"</HoverCardContent>
        </HoverCard>
    }
}
