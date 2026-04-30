//! @canon-level: strict
//! @canon-owner: primitives-team
//! ConfirmDialog Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ConfirmDialogVariant {
    #[default]
    Default,
    Destructive,
    Warning,
}
impl ConfirmDialogVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "primary",
            Self::Destructive => "destructive",
            Self::Warning     => "warning",
        }
    }
}

#[component]
pub fn ConfirmDialogPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] uid: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let uid_str = if uid.is_empty() { crate::infra::uid::generate("cd") } else { uid };
    view! {
        <div
            data-rs-confirm-dialog=""
            data-rs-interaction="overlay"
            data-rs-uid=uid_str
            data-rs-variant=variant.as_str()
            data-rs-state=s.data_rs_state
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ConfirmDialogOverlayPrimitive(
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-confirm-dialog-overlay=""
            data-rs-state=s.data_rs_state
            class=class
        />
    }
}

#[component]
pub fn ConfirmDialogTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h2 data-rs-confirm-dialog-title="" class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn ConfirmDialogDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-confirm-dialog-description="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ConfirmDialogCancelPrimitive(
    children: Children,
    #[prop(into, default = "Cancel".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-confirm-dialog-cancel=""
            data-rs-button=""
            data-rs-variant="ghost"
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ConfirmDialogConfirmPrimitive(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = "Confirm".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-confirm-dialog-confirm=""
            data-rs-button=""
            data-rs-variant=variant.as_str()
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ConfirmDialogTriggerPrimitive(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] target: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-confirm-dialog-trigger=""
            data-rs-button=""
            data-rs-variant=variant.as_str()
            data-rs-target=target
            data-rs-value=value
            data-rs-label=label
            aria-haspopup="alertdialog"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ConfirmDialogPortalPrimitive(children: ChildrenFn) -> impl IntoView {
    view! {
        <leptos::portal::Portal>
            <div data-rs-confirm-dialog-portal="">
                {children()}
            </div>
        </leptos::portal::Portal>
    }
}

#[component]
pub fn ConfirmDialogContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-confirm-dialog-content=""
            data-rs-state="closed"
            role="alertdialog"
            aria-modal="true"
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ConfirmDialogFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-confirm-dialog-footer="" class=class>
            {children()}
        </div>
    }
}
