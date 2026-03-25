//! @canon-level: ui
//! LoadingOverlay - Declarative UI wrapper

use leptos::prelude::*;
use canonrs_core::primitives::LoadingOverlayPrimitive;

#[component]
pub fn LoadingOverlay(
    children: Children,
    #[prop(default = false)] loading: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <LoadingOverlayPrimitive loading=loading class={class.unwrap_or_default()}>
            {children()}
        </LoadingOverlayPrimitive>
    }
}

#[component]
pub fn LoadingOverlayPreview() -> impl IntoView {
    view! {
        <LoadingOverlay loading=true>
            <div style="min-height:100px;padding:2rem;">
                "Content behind loading overlay"
            </div>
        </LoadingOverlay>
    }
}
