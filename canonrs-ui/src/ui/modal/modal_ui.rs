use leptos::prelude::*;
use crate::primitives::{ModalPrimitive, ModalTriggerPrimitive, ModalOverlayPrimitive, ModalContentPrimitive};

#[component]
pub fn Modal(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ModalPrimitive id=id class=class>
            {children()}
        </ModalPrimitive>
    }
}

#[component]
pub fn ModalTrigger(
    children: Children,
    #[prop(into)] target_modal_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ModalTriggerPrimitive target_modal_id=target_modal_id class=class id=id>
            {children()}
        </ModalTriggerPrimitive>
    }
}

#[component]
pub fn ModalOverlay(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ModalOverlayPrimitive class=class id=id />
    }
}

#[component]
pub fn ModalContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ModalContentPrimitive class=class id=id>
            {children()}
        </ModalContentPrimitive>
    }
}
