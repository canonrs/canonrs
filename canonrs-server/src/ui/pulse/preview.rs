use leptos::prelude::*;
use super::pulse_ui::{Pulse, PulseVariant, PulseSize, PulseSpeed};

#[component]
pub fn PulseShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <Pulse variant=PulseVariant::Default>"●"</Pulse>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Animation variant, size and speed strictly typed."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Pulse variant=PulseVariant::Subtle>"●"</Pulse>
                    <Pulse variant=PulseVariant::Default>"●"</Pulse>
                    <Pulse variant=PulseVariant::Emphasized>"●"</Pulse>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Pulse size=PulseSize::Small>"●"</Pulse>
                    <Pulse size=PulseSize::Medium>"●"</Pulse>
                    <Pulse size=PulseSize::Large>"●"</Pulse>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Speed"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Pulse speed=PulseSpeed::Slow>"●"</Pulse>
                    <Pulse speed=PulseSpeed::Normal>"●"</Pulse>
                    <Pulse speed=PulseSpeed::Fast>"●"</Pulse>
                </div>
            </div>
        </div>
    }
}
