//! @canon-id: loading-overlay
//! @canon-label: Loading Overlay
//! @canon-family: utility
//! @canon-category: Display
//! @canon-intent: Block UI during async operations
//! @canon-description: Full loading overlay
//! @canon-composable: false
//! @canon-capabilities: OpenClose
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: loading-overlay, loading, overlay, wait, spinner, block

use leptos::prelude::*;
use canonrs_core::primitives::LoadingOverlayPrimitive;
use canonrs_core::meta::LoadingState;

#[component]
pub fn LoadingOverlay(
    children: Children,
    #[prop(default = LoadingState::Idle)] state: LoadingState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <LoadingOverlayPrimitive state=state class=class>
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
