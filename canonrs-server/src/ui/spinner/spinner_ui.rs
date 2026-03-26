//! @canon-level: ui
//! Spinner - Declarative UI wrapper

use leptos::prelude::*;
use canonrs_core::primitives::SpinnerPrimitive;
use canonrs_core::meta::LoadingState;
pub use canonrs_core::primitives::SpinnerSize;

#[component]
pub fn Spinner(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(default = false)] paused: bool,
    #[prop(into, default = "Loading".to_string())] aria_label: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let state = if paused { LoadingState::Idle } else { LoadingState::Loading };
    view! {
        <SpinnerPrimitive
            size=size
            state=state
            aria_label=aria_label
            class={class.unwrap_or_default()}
        />
    }
}

#[component]
pub fn SpinnerPreview() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            <Spinner size=SpinnerSize::Small />
            <Spinner size=SpinnerSize::Medium />
            <Spinner size=SpinnerSize::Large />
        </div>
    }
}
