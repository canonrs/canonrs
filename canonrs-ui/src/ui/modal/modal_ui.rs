use leptos::prelude::*;
use crate::primitives::{
    DialogPrimitive,
    DialogTriggerPrimitive,
    DialogBackdropPrimitive,
    DialogPopupPrimitive,
    DialogClosePrimitive,
    DialogHeaderPrimitive,
    DialogTitlePrimitive,
    DialogBodyPrimitive,
};

#[component]
pub fn Modal(
    children: ChildrenFn,
    #[prop(into)] id: String,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    view! {
        <DialogPrimitive
            id=id
            class=class
        >
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn ModalTrigger(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogTriggerPrimitive
            target_dialog_id=target_dialog_id
            class=class
            id=id
        >
            {children()}
        </DialogTriggerPrimitive>
    }
}

#[component]
pub fn ModalOverlay(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogBackdropPrimitive
            class=class
            id=id
        />
    }
}

#[component]
pub fn ModalContent(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogPopupPrimitive
            class=class
            id=id
        >
            {children()}
        </DialogPopupPrimitive>
    }
}

#[component]
pub fn ModalHeader(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogHeaderPrimitive
            class=class
            id=id
        >
            {children()}
        </DialogHeaderPrimitive>
    }
}

#[component]
pub fn ModalTitle(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogTitlePrimitive
            class=class
            id=id
        >
            {children()}
        </DialogTitlePrimitive>
    }
}

#[component]
pub fn ModalBody(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogBodyPrimitive
            class=class
            id=id
        >
            {children()}
        </DialogBodyPrimitive>
    }
}

#[component]
pub fn ModalFooter(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogBodyPrimitive
            class=class
            id=id
        >
            {children()}
        </DialogBodyPrimitive>
    }
}

#[component]
pub fn ModalClose(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <DialogClosePrimitive
            target_dialog_id=target_dialog_id
            class=class
            id=id
        >
            {children()}
        </DialogClosePrimitive>
    }
}
