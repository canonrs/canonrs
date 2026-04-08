use leptos::prelude::*;
use super::color_picker_island::{ColorPickerIsland, ColorPickerSwatchIsland};
use canonrs_core::meta::DisabledState;

#[component]
pub fn ColorPickerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ColorPickerIsland value="#3b82f6" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Color input with swatch preview — state governed by data-rs-state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Swatches"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPickerSwatchIsland color="#3b82f6" />
                    <ColorPickerSwatchIsland color="#ef4444" />
                    <ColorPickerSwatchIsland color="#22c55e" />
                    <ColorPickerSwatchIsland color="#f59e0b" />
                    <ColorPickerSwatchIsland color="#8b5cf6" />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPickerIsland value="#6b7280" disabled=DisabledState::Disabled />
                </div>
            </div>
        </div>
    }
}
