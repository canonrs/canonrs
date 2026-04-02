use leptos::prelude::*;
use canonrs_core::primitives::{LoadingOverlayPrimitive, OverlayMode};
use canonrs_core::meta::LoadingState;

pub use canonrs_core::primitives::OverlayMode as LoadingOverlayMode;

#[component]
pub fn LoadingOverlay(
    children: Children,
    #[prop(default = LoadingState::Idle)] state: LoadingState,
    #[prop(default = OverlayMode::Blocking)] mode: OverlayMode,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <LoadingOverlayPrimitive state=state mode=mode class=class>
            {children()}
        </LoadingOverlayPrimitive>
    }
}

#[component]
pub fn LoadingOverlayPreview() -> impl IntoView {
    view! {
        <LoadingOverlay state=LoadingState::Loading>
            <div style="min-height:100px;padding:2rem;">
                "Content behind loading overlay"
            </div>
        </LoadingOverlay>
    }
}
