use leptos::prelude::*;
use crate::primitives::{
    AlertDialogPrimitive,
    AlertDialogTriggerPrimitive,
    AlertDialogOverlayPrimitive,
    AlertDialogContentPrimitive,
    AlertDialogTitlePrimitive,
    AlertDialogDescriptionPrimitive,
};

#[component]
pub fn AlertDialog(
    children: Children,
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogPrimitive id=id class=class>
            {children()}
        </AlertDialogPrimitive>
    }
}

#[component]
pub fn AlertDialogTrigger(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogTriggerPrimitive target_dialog_id=target_dialog_id class=class>
            {children()}
        </AlertDialogTriggerPrimitive>
    }
}

#[component]
pub fn AlertDialogOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogOverlayPrimitive class=class />
    }
}

#[component]
pub fn AlertDialogContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] labelledby: Option<String>,
    #[prop(optional, into)] describedby: Option<String>,
) -> impl IntoView {
    view! {
        <AlertDialogContentPrimitive
            class=class
            labelledby=labelledby.unwrap_or_default()
            describedby=describedby.unwrap_or_default()
        >
            {children()}
        </AlertDialogContentPrimitive>
    }
}

#[component]
pub fn AlertDialogTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AlertDialogTitlePrimitive class=class id=id.unwrap_or_default()>
            {children()}
        </AlertDialogTitlePrimitive>
    }
}

#[component]
pub fn AlertDialogDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AlertDialogDescriptionPrimitive class=class id=id.unwrap_or_default()>
            {children()}
        </AlertDialogDescriptionPrimitive>
    }
}

#[component]
pub fn AlertDialogClose(
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
