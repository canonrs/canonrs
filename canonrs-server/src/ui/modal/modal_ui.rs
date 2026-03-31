
use leptos::prelude::*;
use canonrs_core::primitives::{ModalPrimitive, ModalOverlayPrimitive, ModalContentPrimitive};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Modal(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ModalPrimitive state=state class=class>
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
