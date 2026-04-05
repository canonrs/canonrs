use leptos::prelude::*;
use super::status_dot_island::{StatusDotIsland, StatusDotIslandVariant};

#[component]
pub fn StatusDotShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <StatusDotIsland variant=StatusDotIslandVariant::Online       label="Online" />
                    <StatusDotIsland variant=StatusDotIslandVariant::Away         label="Away" />
                    <StatusDotIsland variant=StatusDotIslandVariant::Busy         label="Busy" />
                    <StatusDotIsland variant=StatusDotIslandVariant::DoNotDisturb label="Do not disturb" />
                    <StatusDotIsland variant=StatusDotIslandVariant::Offline      label="Offline" />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Presence states strictly separated from semantic feedback."
            </p>
        </div>
    }
}
