use leptos::prelude::*;
use canonrs_core::primitives::{
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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <HoverCardTriggerPrimitive
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

#[component]
pub fn HoverCardPreview() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger>"Hover"</HoverCardTrigger>
            <HoverCardContent>"Card content"</HoverCardContent>
        </HoverCard>
    }
}
