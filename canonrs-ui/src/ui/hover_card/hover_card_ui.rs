use leptos::prelude::*;
use crate::primitives::{
    HoverCardPrimitive,
    HoverCardTriggerPrimitive,
    HoverCardContentPrimitive,
};

#[component]
pub fn HoverCard(
    children: Children,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <HoverCardPrimitive
            open=open
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
    #[prop(default = String::new())] describedby_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <HoverCardTriggerPrimitive
            describedby_id=describedby_id
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
    open: Signal<bool>,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = 0)] side_offset: i32,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <HoverCardContentPrimitive
            open=open
            content_id=content_id
            side_offset=side_offset
            class=class
        >
            {children()}
        </HoverCardContentPrimitive>
    }
}
