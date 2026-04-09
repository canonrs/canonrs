//! ConfirmDialog Island — Canon Rule #340 passthrough
use leptos::prelude::*;
use super::confirm_dialog_ui::{
    ConfirmDialog, ConfirmDialogTrigger, ConfirmDialogPortal, ConfirmDialogOverlay,
    ConfirmDialogContent, ConfirmDialogTitle, ConfirmDialogDescription,
    ConfirmDialogFooter, ConfirmDialogCancel, ConfirmDialogConfirm,
};
use canonrs_core::primitives::ConfirmDialogVariant;

#[component]
pub fn ConfirmDialogIsland(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialog variant=variant class=class>{children()}</ConfirmDialog> }
}

#[component]
pub fn ConfirmDialogTriggerIsland(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogTrigger variant=variant class=class.unwrap_or_default()>{children()}</ConfirmDialogTrigger> }
}

#[component]
pub fn ConfirmDialogPortalIsland(children: Children) -> impl IntoView {
    view! { <ConfirmDialogPortal>{children()}</ConfirmDialogPortal> }
}

#[component]
pub fn ConfirmDialogOverlayIsland(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogOverlay class=class.unwrap_or_default() /> }
}

#[component]
pub fn ConfirmDialogContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogContent class=class.unwrap_or_default()>{children()}</ConfirmDialogContent> }
}

#[component]
pub fn ConfirmDialogTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogTitle class=class.unwrap_or_default()>{children()}</ConfirmDialogTitle> }
}

#[component]
pub fn ConfirmDialogDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogDescription class=class.unwrap_or_default()>{children()}</ConfirmDialogDescription> }
}

#[component]
pub fn ConfirmDialogFooterIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogFooter class=class.unwrap_or_default()>{children()}</ConfirmDialogFooter> }
}

#[component]
pub fn ConfirmDialogCancelIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogCancel class=class.unwrap_or_default()>{children()}</ConfirmDialogCancel> }
}

#[component]
pub fn ConfirmDialogConfirmIsland(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogConfirm variant=variant class=class.unwrap_or_default()>{children()}</ConfirmDialogConfirm> }
}
