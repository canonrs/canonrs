#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    ConfirmDialogPrimitive, ConfirmDialogTriggerPrimitive, ConfirmDialogPortalPrimitive,
    ConfirmDialogOverlayPrimitive, ConfirmDialogContentPrimitive,
    ConfirmDialogTitlePrimitive, ConfirmDialogDescriptionPrimitive,
    ConfirmDialogFooterPrimitive, ConfirmDialogCancelPrimitive,
    ConfirmDialogConfirmPrimitive, ConfirmDialogVariant,
};

#[component]
pub fn ConfirmDialog(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ConfirmDialogPrimitive variant=variant class=class>
            {children()}
        </ConfirmDialogPrimitive>
    }
}

#[component]
pub fn ConfirmDialogTrigger(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogTriggerPrimitive variant=variant class=class>{children()}</ConfirmDialogTriggerPrimitive> }
}

#[component]
pub fn ConfirmDialogPortal(children: Children) -> impl IntoView {
    view! { <ConfirmDialogPortalPrimitive>{children()}</ConfirmDialogPortalPrimitive> }
}

#[component]
pub fn ConfirmDialogOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogOverlayPrimitive class=class /> }
}

#[component]
pub fn ConfirmDialogContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogContentPrimitive class=class>{children()}</ConfirmDialogContentPrimitive> }
}

#[component]
pub fn ConfirmDialogTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogTitlePrimitive class=class>{children()}</ConfirmDialogTitlePrimitive> }
}

#[component]
pub fn ConfirmDialogDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogDescriptionPrimitive class=class>{children()}</ConfirmDialogDescriptionPrimitive> }
}

#[component]
pub fn ConfirmDialogFooter(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogFooterPrimitive class=class>{children()}</ConfirmDialogFooterPrimitive> }
}

#[component]
pub fn ConfirmDialogCancel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogCancelPrimitive class=class>{children()}</ConfirmDialogCancelPrimitive> }
}

#[component]
pub fn ConfirmDialogConfirm(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ConfirmDialogConfirmPrimitive variant=variant class=class>{children()}</ConfirmDialogConfirmPrimitive> }
}

#[component]
pub fn ConfirmDialogPreview() -> impl IntoView {
    view! { <ConfirmDialog><ConfirmDialogTrigger>"Open"</ConfirmDialogTrigger></ConfirmDialog> }
}
