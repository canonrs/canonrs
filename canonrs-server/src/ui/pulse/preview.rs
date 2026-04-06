use leptos::prelude::*;
use super::pulse_island::PulseIsland;

#[component]
pub fn PulseShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <PulseIsland />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Animation variant, size and speed strictly typed."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <PulseIsland variant="subtle" />
                    <PulseIsland />
                    <PulseIsland variant="emphasized" />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <PulseIsland size="small" />
                    <PulseIsland size="medium" />
                    <PulseIsland size="large" />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Speed"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <PulseIsland speed="slow" />
                    <PulseIsland speed="normal" />
                    <PulseIsland speed="fast" />
                </div>
            </div>
        </div>
    }
}
