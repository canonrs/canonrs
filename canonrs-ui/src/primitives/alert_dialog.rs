//! @canon-level: strict
//! @canon-owner: primitives-team
//! AlertDialog Primitive - Dialog não-dismissível para ações críticas

use leptos::prelude::*;

pub use super::dialog::{
    DialogPrimitive as AlertDialogPrimitive,
    DialogTriggerPrimitive as AlertDialogTriggerPrimitive,
    DialogBackdropPrimitive as AlertDialogOverlayPrimitive,
    DialogPopupPrimitive as AlertDialogContentPrimitive,
    DialogTitlePrimitive as AlertDialogTitlePrimitive,
    DialogDescriptionPrimitive as AlertDialogDescriptionPrimitive,
};

#[component]
pub fn AlertDialogPortalPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-alert-dialog-portal="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertDialogHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-alert-dialog-header="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertDialogFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-alert-dialog-footer="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertDialogActionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button data-alert-dialog-action="" type="button" class={class} id={id}>
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn AlertDialogCancelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button data-alert-dialog-cancel="" type="button" class={class} id={id}>
            {children.map(|c| c())}
        </button>
    }
}
