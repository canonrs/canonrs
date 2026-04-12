//! @canon-level: strict
//! Dialog Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::dialog_ui::{
    Dialog as DialogUi,
    DialogTrigger as DialogTriggerUi,
    DialogPortal as DialogPortalUi,
    DialogOverlay as DialogOverlayUi,
    DialogContent as DialogContentUi,
    DialogTitle as DialogTitleUi,
    DialogDescription as DialogDescriptionUi,
    DialogClose as DialogCloseUi,
    DialogFooter as DialogFooterUi
};



#[component]
pub fn Dialog(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DialogUi class=class.unwrap_or_default()>{children()}</DialogUi>
    }
}

#[component]
pub fn DialogTrigger(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogTriggerUi class=class.unwrap_or_default()>{children()}</DialogTriggerUi> }
}

#[component]
pub fn DialogPortal(children: Children) -> impl IntoView {
    view! { <DialogPortalUi>{children()}</DialogPortalUi> }
}

#[component]
pub fn DialogOverlay(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogOverlayUi class=class.unwrap_or_default() /> }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogContentUi class=class.unwrap_or_default()>{children()}</DialogContentUi> }
}

#[component]
pub fn DialogTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogTitleUi class=class.unwrap_or_default()>{children()}</DialogTitleUi> }
}

#[component]
pub fn DialogDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogDescriptionUi class=class.unwrap_or_default()>{children()}</DialogDescriptionUi> }
}

#[component]
pub fn DialogClose(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogCloseUi class=class.unwrap_or_default()>{children()}</DialogCloseUi> }
}

#[component]
pub fn DialogFooter(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogFooterUi class=class.unwrap_or_default()>{children()}</DialogFooterUi> }
}
