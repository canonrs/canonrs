//! @canon-id: dialog
//! @canon-label: Dialog
//! @canon-family: overlay
//! @canon-category: Overlay
//! @canon-intent: Display critical content requiring user interaction
//! @canon-description: Modal dialog component
//! @canon-composable: true
//! @canon-capabilities: OpenClose, FocusTrap, KeyboardEsc, AriaModal
//! @canon-required-parts: DialogContent, DialogTitle
//! @canon-optional-parts: DialogOverlay, DialogDescription, DialogClose
//! @canon-tags: dialog, modal, popup, window, overlay, confirmation

use leptos::prelude::*;
use canonrs_core::primitives::{
    DialogPrimitive, DialogPortalPrimitive,
    DialogOverlayPrimitive, DialogContentPrimitive, DialogTitlePrimitive,
    DialogDescriptionPrimitive, DialogTriggerPrimitive, DialogClosePrimitive,
};

#[component]
pub fn Dialog(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogPrimitive class={class.unwrap_or_default()}>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn DialogTrigger(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive class={class.unwrap_or_default()}>
            {children()}
        </DialogTriggerPrimitive>
    }
}

#[component]
pub fn DialogPortal(
    children: Children,
) -> impl IntoView {
    view! {
        <DialogPortalPrimitive>
            {children()}
        </DialogPortalPrimitive>
    }
}

#[component]
pub fn DialogOverlay(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogOverlayPrimitive class={class.unwrap_or_default()} />
    }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogContentPrimitive class={class.unwrap_or_default()}>
            {children()}
        </DialogContentPrimitive>
    }
}

#[component]
pub fn DialogTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogTitlePrimitive class={class.unwrap_or_default()}>
            {children()}
        </DialogTitlePrimitive>
    }
}

#[component]
pub fn DialogDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogDescriptionPrimitive class={class.unwrap_or_default()}>
            {children()}
        </DialogDescriptionPrimitive>
    }
}

#[component]
pub fn DialogClose(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogClosePrimitive class={class.unwrap_or_default()}>
            {children()}
        </DialogClosePrimitive>
    }
}

#[component]
pub fn DialogPreview() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Open Dialog"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay />
                <DialogContent>
                    <DialogTitle>"Dialog Title"</DialogTitle>
                    <DialogDescription>"Dialog description."</DialogDescription>
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
