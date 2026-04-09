use leptos::prelude::*;
use canonrs_core::primitives::{
    ModalPrimitive, ModalTriggerPrimitive, ModalPortalPrimitive,
    ModalOverlayPrimitive, ModalContentPrimitive, ModalTitlePrimitive,
    ModalDescriptionPrimitive, ModalClosePrimitive, ModalFooterPrimitive,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Modal(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalPrimitive state=state class=class>{children()}</ModalPrimitive> }
}

#[component]
pub fn ModalTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalTriggerPrimitive class=class>{children()}</ModalTriggerPrimitive> }
}

#[component]
pub fn ModalPortal(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalPortalPrimitive class=class>{children()}</ModalPortalPrimitive> }
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
    view! { <ModalContentPrimitive class=class>{children()}</ModalContentPrimitive> }
}

#[component]
pub fn ModalTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalTitlePrimitive class=class>{children()}</ModalTitlePrimitive> }
}

#[component]
pub fn ModalDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalDescriptionPrimitive class=class>{children()}</ModalDescriptionPrimitive> }
}

#[component]
pub fn ModalClose(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalClosePrimitive class=class>{children()}</ModalClosePrimitive> }
}

#[component]
pub fn ModalFooter(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalFooterPrimitive class=class>{children()}</ModalFooterPrimitive> }
}
