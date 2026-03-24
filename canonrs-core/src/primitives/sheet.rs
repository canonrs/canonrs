//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sheet Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn SheetPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::from("right"))] side: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-sheet=""
            data-rs-state="closed"
            data-rs-side=side
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SheetTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-sheet-trigger=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn SheetContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-sheet-content=""
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

#[component]
pub fn SheetOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-rs-sheet-overlay="" aria-hidden="true" class=class id=id />
    }
}
