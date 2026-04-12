//! Modal Island — Canon Rule #340 passthrough
use leptos::prelude::*;
use super::modal_ui::{
    Modal as ModalUi,
    ModalTrigger as ModalTriggerUi,
    ModalPortal as ModalPortalUi,
    ModalOverlay as ModalOverlayUi,
    ModalContent as ModalContentUi,
    ModalTitle as ModalTitleUi,
    ModalDescription as ModalDescriptionUi,
    ModalClose as ModalCloseUi,
    ModalFooter as ModalFooterUi
};

#[component]
pub fn Modal(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ModalUi class=class>{children()}</ModalUi> }
}

#[component]
pub fn ModalTrigger(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalTriggerUi class=class.unwrap_or_default()>{children()}</ModalTriggerUi> }
}

#[component]
pub fn ModalPortal(
    children: Children,
) -> impl IntoView {
    view! { <ModalPortalUi>{children()}</ModalPortalUi> }
}

#[component]
pub fn ModalOverlay(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalOverlayUi class=class.unwrap_or_default() /> }
}

#[component]
pub fn ModalContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalContentUi class=class.unwrap_or_default()>{children()}</ModalContentUi> }
}

#[component]
pub fn ModalTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalTitleUi class=class.unwrap_or_default()>{children()}</ModalTitleUi> }
}

#[component]
pub fn ModalDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalDescriptionUi class=class.unwrap_or_default()>{children()}</ModalDescriptionUi> }
}

#[component]
pub fn ModalClose(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalCloseUi class=class.unwrap_or_default()>{children()}</ModalCloseUi> }
}

#[component]
pub fn ModalFooter(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ModalFooterUi class=class.unwrap_or_default()>{children()}</ModalFooterUi> }
}
