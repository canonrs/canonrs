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
        <div data-modal="" data-state="closed" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ModalTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] target_modal_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-modal-trigger={target_modal_id} class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ModalOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-modal-overlay="" aria-hidden="true" class=class id=id />
    }
}

#[component]
pub fn ModalContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-modal-content="" role="dialog" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}
