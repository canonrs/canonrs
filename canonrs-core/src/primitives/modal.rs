//! @canon-level: strict
//! @canon-owner: primitives-team
//! Modal Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

fn modal_uid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    let ctr = CTR.fetch_add(1, Ordering::SeqCst);
    format!("mo-{:016x}-{:08x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(ctr),
        ctr)
}

#[component]
pub fn ModalPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-modal=""
            data-rs-interaction="overlay"
            data-rs-uid=modal_uid()
            data-rs-component="Modal"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ModalTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <button
            type="button"
            data-rs-modal-trigger=""
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
pub fn ModalOverlayPrimitive(
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-modal-overlay=""
            data-rs-state=s.data_rs_state
            class=class
        />
    }
}

#[component]
pub fn ModalContentPrimitive(
    children: Children,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-modal-content=""
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
pub fn ModalPortalPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <div data-rs-modal-portal="" class=class>{children()}</div> }
}

#[component]
pub fn ModalTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <h2 data-rs-modal-title="" class=class>{children()}</h2> }
}

#[component]
pub fn ModalDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <p data-rs-modal-description="" class=class>{children()}</p> }
}

#[component]
pub fn ModalClosePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button type="button" data-rs-modal-close="" data-rs-button="" data-rs-variant="ghost" class=class>
            {children()}
        </button>
    }
}

#[component]
pub fn ModalFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <div data-rs-modal-footer="" class=class>{children()}</div> }
}
