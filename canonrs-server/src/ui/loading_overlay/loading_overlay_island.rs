use leptos::prelude::*;
use super::loading_overlay_ui::{LoadingOverlay, LoadingOverlayMode};
use canonrs_core::meta::LoadingState;

#[island]
pub fn LoadingOverlayIsland(
    children: Children,
    #[prop(optional, into)] state: Option<String>,
    #[prop(optional, into)] mode: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let state = match state.as_deref() {
        Some("loading") => LoadingState::Loading,
        _               => LoadingState::Idle,
    };
    let mode = match mode.as_deref() {
        Some("subtle")   => LoadingOverlayMode::Subtle,
        Some("skeleton") => LoadingOverlayMode::Skeleton,
        _                   => LoadingOverlayMode::Blocking,
    };
    let cls = class.unwrap_or_default();

    view! {
        <LoadingOverlay state=state mode=mode class=cls>
            {children()}
        </LoadingOverlay>
    }
}
