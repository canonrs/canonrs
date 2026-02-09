use leptos::prelude::*;
use crate::primitives::{
    HoverCardPrimitive,
    HoverCardTriggerPrimitive,
    HoverCardContentPrimitive,
};

#[component]
pub fn HoverCard(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <HoverCardPrimitive
            class=class
            id=id
        >
            {children()}
        </HoverCardPrimitive>
    }
}

#[component]
pub fn HoverCardTrigger(
    children: Children,
    #[prop(into)] target_hover_card_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <HoverCardTriggerPrimitive
            target_hover_card_id=target_hover_card_id
            class=class
            id=id
        >
            {children()}
        </HoverCardTriggerPrimitive>
    }
}

#[component]
pub fn HoverCardContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <HoverCardContentPrimitive
            class=class
            id=id
        >
            {children()}
        </HoverCardContentPrimitive>
    }
}
