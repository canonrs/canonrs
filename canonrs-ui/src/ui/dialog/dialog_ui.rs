use leptos::prelude::*;
use crate::primitives::{
    DialogPrimitive,
    DialogTriggerPrimitive,
    DialogBackdropPrimitive,
    DialogPopupPrimitive,
    DialogHeaderPrimitive,
    DialogTitlePrimitive,
    DialogBodyPrimitive,
    DialogClosePrimitive,
};

#[component]
pub fn Dialog(
    children: Children,
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
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
pub fn DialogTrigger(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
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
pub fn DialogBackdrop(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DialogBackdropPrimitive
            class=class
            id=id
        />
    }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
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
pub fn DialogHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
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
pub fn DialogTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
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
pub fn DialogBody(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
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
pub fn DialogClose(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
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
