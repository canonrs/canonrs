//! @canon-level: strict
//! @canon-owner: primitives-team
//! Modal Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ModalPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-modal=""
            data-rs-state="closed"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ModalTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-modal-trigger=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn ModalOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-rs-modal-overlay="" aria-hidden="true" class=class id=id />
    }
}

#[component]
pub fn ModalContentPrimitive(
    #[prop(optional)] children: Option<Children>,
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
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
