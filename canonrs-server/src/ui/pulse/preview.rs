use leptos::prelude::*;
use super::pulse_island::PulseIsland;
use super::pulse_ui::{PulseVariant, PulseSize, PulseSpeed};

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
                    <PulseIsland variant=PulseVariant::Subtle />
                    <PulseIsland />
                    <PulseIsland variant=PulseVariant::Emphasized />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <PulseIsland size=PulseSize::Small />
                    <PulseIsland size=PulseSize::Medium />
                    <PulseIsland size=PulseSize::Large />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Speed"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <PulseIsland speed=PulseSpeed::Slow />
                    <PulseIsland speed=PulseSpeed::Normal />
                    <PulseIsland speed=PulseSpeed::Fast />
                </div>
            </div>
        </div>
    }
}
