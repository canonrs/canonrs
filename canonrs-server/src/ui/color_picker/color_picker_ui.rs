//! @canon-id: color-picker
//! @canon-label: Color Picker
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Select a color value
//! @canon-description: Color picker input
//! @canon-composable: true
//! @canon-capabilities: Value, Disabled
//! @canon-required-parts:
//! @canon-optional-parts: ColorPickerSwatch, ColorPickerInput
//! @canon-tags: color-picker, color, palette, rgb, hex

use leptos::prelude::*;
use canonrs_core::primitives::{ColorPickerPrimitive, ColorPickerInputPrimitive, ColorPickerSwatchPrimitive};
use canonrs_core::meta::DisabledState;
use canonrs_core::meta::SelectionState;

#[component]
pub fn ColorPicker(
    #[prop(into, default = "#000000".to_string())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ColorPickerPrimitive class=class>
            <ColorPickerInputPrimitive value=value name=name disabled=disabled />
        </ColorPickerPrimitive>
    }
}

#[component]
pub fn ColorPickerSwatch(
    #[prop(into)] color: String,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let selected_state = if selected { SelectionState::Selected } else { SelectionState::Unselected };
    view! {
        <ColorPickerSwatchPrimitive color=color selected=selected_state class=class />
    }
}

#[component]
pub fn ColorPickerPreview() -> impl IntoView {
    view! {
        <ColorPicker value="#3b82f6" />
    }
}
