//! @canon-level: strict
//! @canon-owner: primitives-team
//! Popover Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn PopoverPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-popover=""
            data-rs-component="Popover"
            data-rs-state={if open { "open" } else { "closed" }}
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverTriggerPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-popover-trigger=""
            aria-haspopup="dialog"
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn PopoverContentPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-popover-content=""
            role="dialog"
            aria-modal="false"
            tabindex="-1"
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}
