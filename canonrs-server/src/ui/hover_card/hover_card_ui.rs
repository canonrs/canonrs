#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    HoverCardPrimitive, HoverCardTriggerPrimitive, HoverCardContentPrimitive,
    HoverCardSide,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn HoverCard(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
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
    view! { <HoverCardTriggerPrimitive class=class>{children()}</HoverCardTriggerPrimitive> }
}

#[component]
pub fn HoverCardContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = HoverCardSide::Top)] side: HoverCardSide,
) -> impl IntoView {
    view! {
        <HoverCardContentPrimitive side=side class=class>
            {children()}
        </HoverCardContentPrimitive>
    }
}
