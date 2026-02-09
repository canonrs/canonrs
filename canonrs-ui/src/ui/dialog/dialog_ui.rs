use leptos::prelude::*;
use crate::primitives::{DialogPrimitive, DialogTriggerPrimitive, DialogOverlayPrimitive, DialogContentPrimitive};

#[component]
pub fn Dialog(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default = String::new())] class: String,
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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive target_dialog_id=target_dialog_id class=class id=id>
            {children()}
        </DialogTriggerPrimitive>
    }
}

#[component]
pub fn DialogOverlay(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DialogOverlayPrimitive class=class id=id />
    }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DialogContentPrimitive class=class id=id>
            {children()}
        </DialogContentPrimitive>
    }
}
