#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{
    DialogFooter as DialogFooterPrimitive,
    DialogPrimitive, DialogPortalPrimitive,
    DialogOverlayPrimitive, DialogContentPrimitive, DialogTitlePrimitive,
    DialogDescriptionPrimitive, DialogTriggerPrimitive, DialogClosePrimitive,
};

#[component]
pub fn Dialog(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogPrimitive class=class>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn DialogTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive class=class>
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
) -> impl IntoView {
    view! {
        <DialogContentPrimitive class=class>
            {children()}
        </DialogContentPrimitive>
    }
}

#[component]
pub fn DialogTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogTitlePrimitive class=class>
            {children()}
        </DialogTitlePrimitive>
    }
}

#[component]
pub fn DialogDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogDescriptionPrimitive class=class>
            {children()}
        </DialogDescriptionPrimitive>
    }
}

#[component]
pub fn DialogClose(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogClosePrimitive class=class>
            {children()}
        </DialogClosePrimitive>
    }
}

#[component]
pub fn DialogFooter(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DialogFooterPrimitive class=class>{children()}</DialogFooterPrimitive> }
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
