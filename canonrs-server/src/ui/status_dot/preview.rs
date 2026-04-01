use leptos::prelude::*;
use super::status_dot_ui::StatusDot;
use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDotShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <StatusDot variant=StatusDotVariant::Online>"Online"</StatusDot>
                    <StatusDot variant=StatusDotVariant::Away>"Away"</StatusDot>
                    <StatusDot variant=StatusDotVariant::Busy>"Busy"</StatusDot>
                    <StatusDot variant=StatusDotVariant::DoNotDisturb>"Do not disturb"</StatusDot>
                    <StatusDot variant=StatusDotVariant::Offline>"Offline"</StatusDot>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Presence states strictly separated from semantic feedback."
            </p>
        </div>
    }
}
