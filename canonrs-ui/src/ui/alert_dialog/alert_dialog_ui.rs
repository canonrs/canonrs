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
    #[prop(default = String::new())] class: String,
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
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogTriggerPrimitive target_dialog_id=target_dialog_id class=class>
            {children()}
        </AlertDialogTriggerPrimitive>
    }
}

#[component]
pub fn AlertDialogOverlay(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogOverlayPrimitive class=class />
    }
}

#[component]
pub fn AlertDialogContent(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogContentPrimitive class=class>
            {children()}
        </AlertDialogContentPrimitive>
    }
}

#[component]
pub fn AlertDialogTitle(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogTitlePrimitive class=class>
            {children()}
        </AlertDialogTitlePrimitive>
    }
}

#[component]
pub fn AlertDialogDescription(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogDescriptionPrimitive class=class>
            {children()}
        </AlertDialogDescriptionPrimitive>
    }
}
