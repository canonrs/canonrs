//! @canon-level: strict
//! LoadingOverlay Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::loading_overlay_ui::{LoadingOverlay, LoadingOverlayMode};
use canonrs_core::meta::LoadingState;

#[component]
pub fn LoadingOverlayIsland(
    children: Children,
    #[prop(into, default = String::new())] state: String,
    #[prop(into, default = String::new())] mode: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let loading_state = match state.as_str() {
        "loading" => LoadingState::Loading,
        _         => LoadingState::Idle,
    };
    let overlay_mode = match mode.as_str() {
        "subtle"   => LoadingOverlayMode::Subtle,
        "skeleton" => LoadingOverlayMode::Skeleton,
        _          => LoadingOverlayMode::Blocking,
    };
    view! {
        <LoadingOverlay state=loading_state mode=overlay_mode class=class>
            {children()}
        </LoadingOverlay>
    }
}
