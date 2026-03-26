//! @canon-level: ui
//! ColorPicker - native input[type=color], sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::{ColorPickerPrimitive, ColorPickerSwatchPrimitive};
use canonrs_core::meta::SelectionState;

fn make_input(value: String, name: String, disabled: bool) -> impl IntoView {
    view! {
        <input
            type="color"
            data-rs-color-picker-input=""
            value=value
            name=name
            disabled=disabled
            aria-label="Color picker"
            class=""
        />
    }
}

#[component]
pub fn ColorPicker(
    #[prop(into, default = "#000000".to_string())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let input = make_input(value, name, disabled);
    view! {
        <ColorPickerPrimitive class=class>
            {input}
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
