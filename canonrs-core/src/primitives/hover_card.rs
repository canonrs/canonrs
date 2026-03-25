//! @canon-level: strict
//! @canon-owner: primitives-team
//! HoverCard Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn HoverCardPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-hover-card=""
            data-rs-state={if open { "open" } else { "closed" }}
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn HoverCardTriggerPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-hover-card-trigger=""
            tabindex="0"
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </span>
    }
}

#[component]
pub fn HoverCardContentPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            role="tooltip"
            data-rs-hover-card-content=""
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}
