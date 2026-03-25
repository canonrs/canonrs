//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tooltip Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TooltipPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-tooltip=""
            data-rs-component="Tooltip"
            data-rs-state={if open { "open" } else { "closed" }}
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TooltipTriggerPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-tooltip-trigger=""
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TooltipContentPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tooltip-content=""
            role="tooltip"
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TooltipProviderPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-tooltip-provider="" class=class>
            {children()}
        </div>
    }
}
