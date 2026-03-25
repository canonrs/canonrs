//! @canon-level: strict
//! @canon-owner: primitives-team
//! Toast Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::state_engine::visibility_attrs;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Info,
}

impl ToastVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Info => "info",
        }
    }
}

#[component]
pub fn ToastPrimitive(
    children: Children,
    #[prop(default = ToastVariant::Default)] variant: ToastVariant,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-toast=""
            data-rs-variant=variant.as_str()
            data-rs-state=s.data_rs_state
            role="status"
            aria-live="polite"
            aria-atomic="true"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToastViewportPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-toast-viewport="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ToastTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-toast-title="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ToastDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-toast-description="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ToastActionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button type="button" data-rs-toast-action="" class=class>
            {children()}
        </button>
    }
}

#[component]
pub fn ToastClosePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button type="button" data-rs-toast-close="" aria-label="Close" class=class>
            {children()}
        </button>
    }
}
