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
    #[prop(into, default = String::new())] aria_labelledby: String,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_ad = crate::infra::uid::generate("ad");
    view! {
        <div
            data-rs-dialog-content=""
            data-rs-uid=uid_ad
            data-rs-interaction="overlay"
            role="alertdialog"
            aria-modal="true"
            aria-live="assertive"
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}
