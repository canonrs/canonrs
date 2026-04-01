use leptos::prelude::*;
use super::color_picker_ui::{ColorPicker, ColorPickerSwatch};
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
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPicker value="#6b7280" disabled=DisabledState::Disabled />
                </div>
            </div>
        </div>
    }
}
