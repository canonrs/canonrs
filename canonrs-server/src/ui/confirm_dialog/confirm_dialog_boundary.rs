//! ConfirmDialog Island — Canon Rule #340 passthrough
use leptos::prelude::*;
use super::confirm_dialog_ui::{
    ConfirmDialog as ConfirmDialogUi,
    ConfirmDialogTrigger as ConfirmDialogTriggerUi,
    ConfirmDialogPortal as ConfirmDialogPortalUi,
    ConfirmDialogOverlay as ConfirmDialogOverlayUi,
    ConfirmDialogContent as ConfirmDialogContentUi,
    ConfirmDialogTitle as ConfirmDialogTitleUi,
    ConfirmDialogDescription as ConfirmDialogDescriptionUi,
    ConfirmDialogFooter as ConfirmDialogFooterUi,
    ConfirmDialogCancel as ConfirmDialogCancelUi,
    ConfirmDialogConfirm as ConfirmDialogConfirmUi,
};
pub use canonrs_core::primitives::ConfirmDialogVariant;

#[component]
pub fn ConfirmDialog(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] uid: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogUi variant=variant uid=uid class=class.unwrap_or_default()>{children()}</ConfirmDialogUi> }
}

#[component]
pub fn ConfirmDialogTrigger(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] target: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] label: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogTriggerUi variant=variant target=target value=value label=label class=class.unwrap_or_default()>{children()}</ConfirmDialogTriggerUi> }
}

#[component]
pub fn ConfirmDialogPortal(children: ChildrenFn) -> impl IntoView {
    view! { <ConfirmDialogPortalUi>{children()}</ConfirmDialogPortalUi> }
}

#[component]
pub fn ConfirmDialogOverlay(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogOverlayUi class=class.unwrap_or_default() /> }
}

#[component]
pub fn ConfirmDialogContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogContentUi class=class.unwrap_or_default()>{children()}</ConfirmDialogContentUi> }
}

#[component]
pub fn ConfirmDialogTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogTitleUi class=class.unwrap_or_default()>{children()}</ConfirmDialogTitleUi> }
}

#[component]
pub fn ConfirmDialogDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogDescriptionUi class=class.unwrap_or_default()>{children()}</ConfirmDialogDescriptionUi> }
}

#[component]
pub fn ConfirmDialogFooter(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogFooterUi class=class.unwrap_or_default()>{children()}</ConfirmDialogFooterUi> }
}

#[component]
pub fn ConfirmDialogCancel(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogCancelUi class=class.unwrap_or_default()>{children()}</ConfirmDialogCancelUi> }
}

#[component]
pub fn ConfirmDialogConfirm(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ConfirmDialogConfirmUi variant=variant class=class.unwrap_or_default()>{children()}</ConfirmDialogConfirmUi> }
}
