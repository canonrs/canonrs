//! @canon-level: strict
//! @canon-owner: primitives-team
//! AlertDialog Primitive - Reusa Dialog com role=alertdialog

pub use super::dialog::{
    DialogPrimitive as AlertDialogPrimitive,
    DialogTriggerPrimitive as AlertDialogTriggerPrimitive,
    DialogPortalPrimitive as AlertDialogPortalPrimitive,
    DialogOverlayPrimitive as AlertDialogOverlayPrimitive,
    DialogTitlePrimitive as AlertDialogTitlePrimitive,
    DialogDescriptionPrimitive as AlertDialogDescriptionPrimitive,
    DialogClosePrimitive as AlertDialogClosePrimitive,
};

use leptos::prelude::*;

#[component]
pub fn AlertDialogContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-dialog-content=""
            role="alertdialog"
            aria-modal="true"
            aria-live="assertive"
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}
