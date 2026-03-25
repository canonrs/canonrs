//! @canon-level: ui
//! Modal - attribute-driven
//! Trigger: attr:data-rs-modal-trigger=""
//! Close:   attr:data-rs-modal-close=""

use leptos::prelude::*;
use canonrs_core::primitives::{ModalPrimitive, ModalOverlayPrimitive, ModalContentPrimitive};

#[component]
pub fn Modal(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ModalPrimitive open=open class=class>
            {children()}
        </ModalPrimitive>
    }
}

#[component]
pub fn ModalOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalOverlayPrimitive class=class /> }
}

#[component]
pub fn ModalContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ModalContentPrimitive class=class>
            {children()}
        </ModalContentPrimitive>
    }
}

#[component]
pub fn ModalPreview() -> impl IntoView {
    view! {
        <Modal>
            <button type="button" data-rs-modal-trigger="">"Open Modal"</button>
            <ModalOverlay />
            <ModalContent>
                <p>"Modal content"</p>
                <button type="button" data-rs-modal-close="">"Close"</button>
            </ModalContent>
        </Modal>
    }
}
