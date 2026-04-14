//! @canon-level: strict
//! @canon-owner: primitives-team
//! Toast Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Default)]
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
    pub fn aria_live(&self) -> &'static str {
        match self { Self::Error | Self::Warning => "assertive", _ => "polite" }
    }
    pub fn role(&self) -> &'static str {
        match self { Self::Error | Self::Warning => "alert", _ => "status" }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub enum ToastLifecycle {
    #[default]
    Open,
    Closing,
    Closed,
}

impl ToastLifecycle {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Open => "open", Self::Closing => "closing", Self::Closed => "closed" }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub enum ToastPosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    TopCenter,
    BottomCenter,
}

impl ToastPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TopRight => "top-right",
            Self::TopLeft => "top-left",
            Self::BottomRight => "bottom-right",
            Self::BottomLeft => "bottom-left",
            Self::TopCenter => "top-center",
            Self::BottomCenter => "bottom-center",
        }
    }
}

#[component]
pub fn ToastPrimitive(
    children: Children,
    #[prop(default = ToastVariant::Default)] variant: ToastVariant,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(default = ToastLifecycle::Open)] lifecycle: ToastLifecycle,
    #[prop(into, optional)] title_id: Option<String>,
    #[prop(into, optional)] description_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-toast=""
            data-rs-uid=crate::infra::uid::generate("ts")
            data-rs-interaction="init"
            data-rs-variant=variant.as_str()
            data-rs-state=s.data_rs_state
            data-rs-lifecycle=lifecycle.as_str()
            role=variant.role()
            aria-live=variant.aria_live()
            aria-atomic="true"
            aria-labelledby=title_id
            aria-describedby=description_id
            hidden=s.hidden
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToastViewportPrimitive(
    children: Children,
    #[prop(default = ToastPosition::TopRight)] position: ToastPosition,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-toast-viewport=""
            data-rs-position=position.as_str()
            aria-label="Notifications"
            role="region"
            class=class
        >
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
    #[prop(into)] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-toast-action=""
            aria-label=aria_label
            class=class
        >
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
        <button
            type="button"
            data-rs-toast-close=""
            aria-label="Close notification"
            class=class
        >
            {children()}
        </button>
    }
}
