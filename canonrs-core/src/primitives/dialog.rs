//! @canon-level: strict
//! @canon-owner: primitives-team
//! Dialog Primitive - HTML puro + ARIA completo

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[component]
pub fn DialogPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-dialog=""
            data-rs-interaction="overlay"
            data-rs-component="Dialog"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DialogTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <button
            type="button"
            data-rs-dialog-trigger=""
            data-rs-button=""
            data-rs-variant="primary"
            data-rs-state=s.data_rs_state
            aria-haspopup="dialog"
            aria-expanded=s.aria_expanded
            aria-controls=aria_controls
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DialogPortalPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-dialog-portal=""
            data-rs-component="DialogPortal"
            data-rs-behavior="portal"
            data-rs-state=s.data_rs_state
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DialogOverlayPrimitive(
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-dialog-overlay=""
            data-rs-state=s.data_rs_state
            class=class
        />
    }
}

#[component]
pub fn DialogContentPrimitive(
    children: Children,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-dialog-content=""
            role="dialog"
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
pub fn DialogTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h2 data-rs-dialog-title="" class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn DialogDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-dialog-description="" class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn DialogClosePrimitive(
    children: Children,
    #[prop(into, default = "Close dialog".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-dialog-close=""
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}
