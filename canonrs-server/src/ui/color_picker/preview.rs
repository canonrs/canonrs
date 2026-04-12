use leptos::prelude::*;
use super::color_picker_boundary::{ColorPicker, ColorPickerSwatch, ColorPickerSwatches};
use canonrs_core::meta::DisabledState;

#[component]
pub fn ColorPickerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Color Picker"</span>
                <div data-rs-showcase-preview-stage="">
                    <ColorPicker value="#3b82f6" />
                </div>
            </div>

            <p data-rs-showcase-preview-anchor="">
                "Color input with swatch preview — state governed by data-rs-state."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Swatches"</span>
                <div data-rs-showcase-preview-stage="">
                    <ColorPickerSwatches value="#3b82f6">
                        <ColorPickerSwatch color="#3b82f6" />
                        <ColorPickerSwatch color="#ef4444" />
                        <ColorPickerSwatch color="#22c55e" />
                        <ColorPickerSwatch color="#f59e0b" />
                        <ColorPickerSwatch color="#8b5cf6" />
                        <ColorPickerSwatch color="#ec4899" />
                    </ColorPickerSwatches>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-stage="">
                    <ColorPicker value="#6b7280" disabled=DisabledState::Disabled />
                </div>
            </div>

        </div>
    }
}
