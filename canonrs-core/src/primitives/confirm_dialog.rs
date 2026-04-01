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
            Self::Default     => "default",
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
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-confirm-dialog=""
            data-rs-component="ConfirmDialog"
            data-rs-behavior="overlay"
            data-rs-variant=variant.as_str()
            data-rs-state=s.data_rs_state
            role="alertdialog"
            aria-modal="true"
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
            tabindex="-1"
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
pub fn ConfirmDialogActionsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-confirm-dialog-actions=""
            role="group"
            class=class
        >
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
            data-rs-variant=variant.as_str()
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}
