use leptos::prelude::*;
use super::color_picker_ui::{ColorPicker, ColorPickerSwatch, ColorPickerDisplay, ColorFormat};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn ColorPickerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ColorPicker value="#3b82f6" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Color input with swatch preview — state governed by data-rs-state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Swatches"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPickerSwatch color="#3b82f6" selected=SelectionState::Selected />
                    <ColorPickerSwatch color="#ef4444" />
                    <ColorPickerSwatch color="#22c55e" />
                    <ColorPickerSwatch color="#f59e0b" />
                    <ColorPickerSwatch color="#8b5cf6" />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Color Format Display"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPickerDisplay value="#3b82f6" format=ColorFormat::Hex />
                    <ColorPickerDisplay value="rgb(59, 130, 246)" format=ColorFormat::Rgb />
                    <ColorPickerDisplay value="hsl(217, 91%, 60%)" format=ColorFormat::Hsl />
                    <ColorPickerDisplay value="cmyk(76%, 47%, 0%, 4%)" format=ColorFormat::Cmyk />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPicker value="#6b7280" disabled=DisabledState::Disabled />
                </div>
            </div>
        </div>
    }
}
