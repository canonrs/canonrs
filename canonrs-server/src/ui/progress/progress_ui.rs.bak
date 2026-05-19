#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{ProgressPrimitive, ProgressIndicatorPrimitive};
use canonrs_core::primitives::progress::ProgressState;

#[component]
pub fn Progress(
    #[prop(default = 0.0)] value: f64,
    #[prop(default = ProgressState::Default)] state: ProgressState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let clamped = value.clamp(0.0, 100.0);
    let transform = format!("transform: translateX(-{:.0}%)", 100.0 - clamped);
    view! {
        <ProgressPrimitive value=clamped state=state class=class>
            <ProgressIndicatorPrimitive style=transform />
        </ProgressPrimitive>
    }
}

#[component]
pub fn ProgressPreview() -> impl IntoView {
    view! {
        <Progress value=50.0 />
    }
}
