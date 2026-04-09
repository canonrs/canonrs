use leptos::prelude::*;
use super::status_dot_island::StatusDotIsland;
use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDotShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <StatusDotIsland variant=StatusDotVariant::Online       label="Online" />
                    <StatusDotIsland variant=StatusDotVariant::Away         label="Away" />
                    <StatusDotIsland variant=StatusDotVariant::Busy         label="Busy" />
                    <StatusDotIsland variant=StatusDotVariant::DoNotDisturb label="Do not disturb" />
                    <StatusDotIsland variant=StatusDotVariant::Offline      label="Offline" />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Presence states strictly separated from semantic feedback."
            </p>
        </div>
    }
}
