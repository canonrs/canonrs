//! @canon-level: strict
//! Dialog Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::dialog_ui::{
    Dialog, DialogTrigger, DialogPortal,
    DialogOverlay, DialogContent, DialogTitle,
    DialogDescription, DialogClose, DialogFooter,
};



#[component]
pub fn DialogIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <Dialog class=class.unwrap_or_default()>{children()}</Dialog>
    }
}

#[component]
pub fn DialogTriggerIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogTrigger class=class.unwrap_or_default()>{children()}</DialogTrigger> }
}

#[component]
pub fn DialogPortalIsland(children: Children) -> impl IntoView {
    view! { <DialogPortal>{children()}</DialogPortal> }
}

#[component]
pub fn DialogOverlayIsland(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogOverlay class=class.unwrap_or_default() /> }
}

#[component]
pub fn DialogContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogContent class=class.unwrap_or_default()>{children()}</DialogContent> }
}

#[component]
pub fn DialogTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogTitle class=class.unwrap_or_default()>{children()}</DialogTitle> }
}

#[component]
pub fn DialogDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogDescription class=class.unwrap_or_default()>{children()}</DialogDescription> }
}

#[component]
pub fn DialogCloseIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogClose class=class.unwrap_or_default()>{children()}</DialogClose> }
}

#[component]
pub fn DialogFooterIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <DialogFooter class=class.unwrap_or_default()>{children()}</DialogFooter> }
}
