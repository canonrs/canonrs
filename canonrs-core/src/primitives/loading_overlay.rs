//! @canon-level: strict
//! @canon-owner: primitives-team
//! LoadingOverlay Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::LoadingState;

#[derive(Clone, PartialEq, Default)]
pub enum OverlayMode {
    #[default]
    Blocking,
    Subtle,
    Skeleton,
}

impl OverlayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            OverlayMode::Blocking => "blocking",
            OverlayMode::Subtle   => "subtle",
            OverlayMode::Skeleton => "skeleton",
        }
    }
}

#[component]
pub fn LoadingOverlayPrimitive(
    children: Children,
    #[prop(default = LoadingState::Idle)] state: LoadingState,
    #[prop(default = OverlayMode::Blocking)] mode: OverlayMode,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let is_loading = state == LoadingState::Loading;
    let data_state = if is_loading { "loading" } else { "idle" };
    let overlay_aria_hidden = if is_loading { "false" } else { "true" };
    view! {
        <div
            data-rs-overlay-container=""
            data-rs-component="LoadingOverlay"
            data-rs-state=data_state
            data-rs-overlay-mode=mode.as_str()
            aria-busy=if is_loading { "true" } else { "false" }
            class=class
        >
            <div data-rs-overlay-content="">
                {children()}
            </div>
            <div
                data-rs-loading-overlay=""
                data-rs-uid=crate::infra::uid::generate("lo")
                data-rs-interaction="init"
                aria-hidden=overlay_aria_hidden
                aria-live="polite"
            >
                <span data-rs-loading-overlay-spinner="">"⟳"</span>
            </div>
        </div>
    }
}
