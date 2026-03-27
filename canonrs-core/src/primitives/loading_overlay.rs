//! @canon-level: strict
//! @canon-owner: primitives-team
//! LoadingOverlay Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{LoadingState, VisibilityState};
use crate::infra::state_engine::{loading_attrs, visibility_attrs};

#[component]
pub fn LoadingOverlayPrimitive(
    children: Children,
    #[prop(default = LoadingState::Idle)] state: LoadingState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let la  = loading_attrs(state);
    let vis = visibility_attrs(
        if state == LoadingState::Loading {
            VisibilityState::Open
        } else {
            VisibilityState::Closed
        },
    );
    view! {
        <div
            data-rs-loading-overlay=""
            data-rs-component="LoadingOverlay"
            data-rs-behavior="overlay"
            data-rs-state=la.data_rs_state
            aria-busy=la.aria_busy
            aria-hidden=vis.aria_hidden
            aria-live="polite"
            hidden=vis.hidden
            class=class
        >
            {children()}
        </div>
    }
}
