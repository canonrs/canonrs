//! @canon-id: alert-dialog
//! @canon-label: Alert Dialog
//! @canon-family: overlay
//! @canon-category: Overlay
//! @canon-intent: Confirm destructive actions with user
//! @canon-description: Alert dialog for critical confirmations
//! @canon-composable: true
//! @canon-capabilities: OpenClose, FocusTrap, AriaModal
//! @canon-required-parts: AlertDialogContent, AlertDialogTitle
//! @canon-optional-parts: AlertDialogOverlay, AlertDialogDescription, AlertDialogAction
//! @canon-tags: alert-dialog, confirm, destructive, modal, overlay

use leptos::prelude::*;
use canonrs_core::primitives::{
    AlertDialogPrimitive,
    AlertDialogPortalPrimitive,
    AlertDialogOverlayPrimitive,
    AlertDialogContentPrimitive,
    AlertDialogTitlePrimitive,
    AlertDialogDescriptionPrimitive,
};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn AlertDialog(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogPrimitive class=class>
            {children()}
        </AlertDialogPrimitive>
    }
}

#[component]
pub fn AlertDialogTrigger(
    children: Children,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Button
            variant=variant
            class=class
            attr:data-rs-dialog-trigger=""
            attr:aria-haspopup="dialog"
            attr:aria-expanded="false"
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn AlertDialogPortal(
    children: Children,
) -> impl IntoView {
    view! {
        <AlertDialogPortalPrimitive>
            {children()}
        </AlertDialogPortalPrimitive>
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
) -> impl IntoView {
    view! {
        <AlertDialogContentPrimitive class=class aria_labelledby="alert-title">
            {children()}
        </AlertDialogContentPrimitive>
    }
}

#[component]
pub fn AlertDialogTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialogDescriptionPrimitive class=class>
            {children()}
        </AlertDialogDescriptionPrimitive>
    }
}

#[component]
pub fn AlertDialogClose(
    children: Children,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Button
            variant=variant
            class=class
            attr:data-rs-dialog-close=""
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn AlertDialogPreview() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Delete Account"</AlertDialogTrigger>
            <AlertDialogPortal>
                <AlertDialogOverlay />
                <AlertDialogContent>
                    <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "This action cannot be undone."
                    </AlertDialogDescription>
                    <div style="display:flex;gap:0.5rem;margin-top:1rem;justify-content:flex-end;">
                        <AlertDialogClose>"Cancel"</AlertDialogClose>
                        <button type="button">"Confirm"</button>
                    </div>
                </AlertDialogContent>
            </AlertDialogPortal>
        </AlertDialog>
    }
}
