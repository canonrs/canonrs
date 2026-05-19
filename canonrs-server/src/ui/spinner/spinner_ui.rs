#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::SpinnerPrimitive;
use canonrs_core::meta::LoadingState;
pub use canonrs_core::primitives::SpinnerSize;

#[component]
pub fn Spinner(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(default = false)] paused: bool,
    #[prop(into, default = "Loading".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if paused { LoadingState::Idle } else { LoadingState::Loading };
    view! {
        <SpinnerPrimitive
            size=size
            state=state
            aria_label=aria_label
            class=class
        />
    }
}

