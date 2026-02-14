use leptos::prelude::*;
use crate::primitives::{
    DialogPrimitive, DialogTriggerPrimitive, DialogOverlayPrimitive,
    DialogContentPrimitive, DialogTitlePrimitive, DialogDescriptionPrimitive,
};

#[component]
pub fn Dialog(
    children: Children,
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogPrimitive id=id class=class>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn DialogTrigger(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive target_dialog_id=target_dialog_id class=class>
            {children()}
        </DialogTriggerPrimitive>
    }
}

#[component]
pub fn DialogOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogOverlayPrimitive class=class />
    }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] labelledby: Option<String>,
    #[prop(optional, into)] describedby: Option<String>,
) -> impl IntoView {
    view! {
        <DialogContentPrimitive class=class labelledby=labelledby.unwrap_or_default() describedby=describedby.unwrap_or_default()>
            {children()}
        </DialogContentPrimitive>
    }
}

#[component]
pub fn DialogTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! {
        <DialogTitlePrimitive class=class id=id.unwrap_or_default()>
            {children()}
        </DialogTitlePrimitive>
    }
}

#[component]
pub fn DialogDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! {
        <DialogDescriptionPrimitive class=class id=id.unwrap_or_default()>
            {children()}
        </DialogDescriptionPrimitive>
    }
}

#[component]
pub fn DialogClose(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            data-dialog-close=target_dialog_id
            type="button"
            class=class
        >
            {children()}
        </button>
    }
}
