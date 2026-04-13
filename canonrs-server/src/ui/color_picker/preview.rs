use leptos::prelude::*;
use super::color_picker_boundary::{ColorPicker, ColorPickerSwatch, ColorPickerSwatches};
use canonrs_core::meta::DisabledState;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ColorPickerShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <ColorPicker value="#3b82f6" />
            <p data-rs-showcase-preview-anchor="">
                "Color input with swatch preview — state governed by data-rs-state."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Swatches"</span>
                <ColorPickerSwatches value="#3b82f6">
                    <ColorPickerSwatch color="#3b82f6" />
                    <ColorPickerSwatch color="#ef4444" />
                    <ColorPickerSwatch color="#22c55e" />
                    <ColorPickerSwatch color="#f59e0b" />
                    <ColorPickerSwatch color="#8b5cf6" />
                    <ColorPickerSwatch color="#ec4899" />
                </ColorPickerSwatches>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <ColorPicker value="#6b7280" disabled=DisabledState::Disabled />
            </Stack>
        </Stack>
    }
}
