use leptos::prelude::*;
use super::alert_island::AlertIsland;
use canonrs_core::primitives::AlertVariant;

#[component]
pub fn AlertShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <AlertIsland title="Info" description="This is a default informational alert." />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Correct ARIA role and live region enforced by variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <AlertIsland variant=AlertVariant::Success     title="Success"     description="Your changes have been saved." />
                    <AlertIsland variant=AlertVariant::Warning     title="Warning"     description="Session expires in 5 minutes." />
                    <AlertIsland variant=AlertVariant::Destructive title="Error"       description="Failed to save changes." />
                    <AlertIsland variant=AlertVariant::Default        title="Info"        description="A new version is available." />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Dismissible"</span>
                <div data-rs-showcase-preview-row="">
                    <AlertIsland title="Update available" description="New version ready." dismissible=true />
                </div>
            </div>
        </div>
    }
}
