//! @canon-level: strict
//! @canon-owner: primitives-team
//! Modal Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ModalPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-modal=""
            data-rs-state={if open { "open" } else { "closed" }}
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ModalTriggerPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-modal-trigger=""
            aria-haspopup="dialog"
            aria-expanded="false"
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ModalOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-rs-modal-overlay="" aria-hidden="true" class=class id=if id.is_empty() { None } else { Some(id.clone()) } />
    }
}

#[component]
pub fn ModalContentPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-modal-content=""
            role="dialog"
            aria-modal="true"
            tabindex="-1"
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}
