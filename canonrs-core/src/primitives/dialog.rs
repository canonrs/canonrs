//! @canon-level: strict
//! @canon-owner: primitives-team
//! Dialog Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[component]
pub fn DialogPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] uid: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let uid_str = if uid.is_empty() { crate::infra::uid::generate("dlg") } else { uid };
    view! {
        <div
            data-rs-dialog=""
            data-rs-interaction="overlay"
            data-rs-uid=uid_str
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
    #[prop(into, default = String::new())] target: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-dialog-trigger=""
            data-rs-button=""
            data-rs-variant="primary"
            data-rs-target=target
            aria-haspopup="dialog"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DialogPortalPrimitive(children: ChildrenFn) -> impl IntoView {
    view! {
        <leptos::portal::Portal>
            <div data-rs-dialog-portal="">
                {children()}
            </div>
        </leptos::portal::Portal>
    }
}

#[component]
pub fn DialogOverlayPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-dialog-overlay=""
            data-rs-state="closed"
            class=class
        />
    }
}

#[component]
pub fn DialogContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-dialog-content=""
            data-rs-state="closed"
            role="dialog"
            aria-modal="true"
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
pub fn DialogFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-dialog-footer="" class=class>
            {children()}
        </div>
    }
}
