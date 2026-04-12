use leptos::prelude::*;
use super::boundary::StatusDot;
use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDotShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <StatusDot variant=StatusDotVariant::Online       label="Online" />
                    <StatusDot variant=StatusDotVariant::Away         label="Away" />
                    <StatusDot variant=StatusDotVariant::Busy         label="Busy" />
                    <StatusDot variant=StatusDotVariant::DoNotDisturb label="Do not disturb" />
                    <StatusDot variant=StatusDotVariant::Offline      label="Offline" />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Presence states strictly separated from semantic feedback."
            </p>
        </div>
    }
}
