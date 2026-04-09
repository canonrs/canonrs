//! Modal Island — Canon Rule #340 passthrough
use leptos::prelude::*;
use super::modal_ui::{
    Modal, ModalTrigger, ModalPortal, ModalOverlay,
    ModalContent, ModalTitle, ModalDescription, ModalClose, ModalFooter,
};

#[component]
pub fn ModalIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <Modal class=class>{children()}</Modal> }
}

#[component]
pub fn ModalTriggerIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalTrigger class=class.unwrap_or_default()>{children()}</ModalTrigger> }
}

#[component]
pub fn ModalPortalIsland(
    children: Children,
) -> impl IntoView {
    view! { <ModalPortal>{children()}</ModalPortal> }
}

#[component]
pub fn ModalOverlayIsland(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalOverlay class=class.unwrap_or_default() /> }
}

#[component]
pub fn ModalContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalContent class=class.unwrap_or_default()>{children()}</ModalContent> }
}

#[component]
pub fn ModalTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalTitle class=class.unwrap_or_default()>{children()}</ModalTitle> }
}

#[component]
pub fn ModalDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalDescription class=class.unwrap_or_default()>{children()}</ModalDescription> }
}

#[component]
pub fn ModalCloseIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalClose class=class.unwrap_or_default()>{children()}</ModalClose> }
}

#[component]
pub fn ModalFooterIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalFooter class=class.unwrap_or_default()>{children()}</ModalFooter> }
}
