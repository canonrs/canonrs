use leptos::prelude::*;
use super::icon_island::IconIsland;

#[component]
pub fn IconShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland size="lg" variant="primary">"★"</IconIsland>
                    <IconIsland size="lg">"★"</IconIsland>
                    <IconIsland size="lg" variant="muted">"★"</IconIsland>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Icon size and variant enforced via typed enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland size="sm">"★"</IconIsland>
                    <IconIsland size="md">"★"</IconIsland>
                    <IconIsland size="lg">"★"</IconIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland>"★"</IconIsland>
                    <IconIsland variant="muted">"★"</IconIsland>
                    <IconIsland variant="primary">"★"</IconIsland>
                    <IconIsland variant="destructive">"★"</IconIsland>
                    <IconIsland variant="success">"★"</IconIsland>
                    <IconIsland variant="warning">"★"</IconIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Spin"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland spin=true>"⟳"</IconIsland>
                </div>
            </div>
        </div>
    }
}
