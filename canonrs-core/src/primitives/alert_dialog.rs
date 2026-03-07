//! @canon-level: strict
//! @canon-owner: primitives-team
//! AlertDialog Primitive - Reusa Dialog com role=alertdialog

// Reutiliza primitives do Dialog exceto Content
pub use super::dialog::{
    DialogPrimitive as AlertDialogPrimitive,
    DialogTriggerPrimitive as AlertDialogTriggerPrimitive,
    DialogOverlayPrimitive as AlertDialogOverlayPrimitive,
    DialogTitlePrimitive as AlertDialogTitlePrimitive,
    DialogDescriptionPrimitive as AlertDialogDescriptionPrimitive,
};

use leptos::prelude::*;

/// AlertDialogContent usa role="alertdialog" + aria-live="assertive"
#[component]
pub fn AlertDialogContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(into, default = String::new())] labelledby: String,
    #[prop(into, default = String::new())] describedby: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog-content=""
            role="alertdialog"
            aria-modal="true"
            aria-live="assertive"
            aria-labelledby={if labelledby.is_empty() { None } else { Some(labelledby.clone()) }}
            aria-describedby={if describedby.is_empty() { None } else { Some(describedby.clone()) }}
            tabindex="-1"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
