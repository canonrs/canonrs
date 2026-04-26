#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    DialogPrimitive, DialogTriggerPrimitive, DialogPortalPrimitive,
    DialogOverlayPrimitive, DialogContentPrimitive, DialogTitlePrimitive,
    DialogDescriptionPrimitive, DialogClosePrimitive, DialogFooterPrimitive,
};

#[component]
pub fn Dialog(
    children: Children,
    #[prop(into, default = String::new())] uid: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DialogPrimitive uid=uid class=class>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn DialogTrigger(
    children: Children,
    #[prop(into, default = String::new())] target: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DialogTriggerPrimitive target=target class=class>{children()}</DialogTriggerPrimitive> }
}

#[component]
pub fn DialogPortal(children: ChildrenFn) -> impl IntoView {
    view! { <DialogPortalPrimitive>{children()}</DialogPortalPrimitive> }
}

#[component]
pub fn DialogOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DialogOverlayPrimitive class=class /> }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DialogContentPrimitive class=class>{children()}</DialogContentPrimitive> }
}

#[component]
pub fn DialogTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DialogTitlePrimitive class=class>{children()}</DialogTitlePrimitive> }
}

#[component]
pub fn DialogDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DialogDescriptionPrimitive class=class>{children()}</DialogDescriptionPrimitive> }
}

#[component]
pub fn DialogClose(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DialogClosePrimitive class=class>{children()}</DialogClosePrimitive> }
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
        <Dialog uid="dialog-preview">
            <DialogTrigger>"Open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay />
                <DialogContent>
                    <DialogTitle>"Dialog"</DialogTitle>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
