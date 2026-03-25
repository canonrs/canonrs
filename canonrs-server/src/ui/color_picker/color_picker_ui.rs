//! @canon-level: ui
//! ColorPicker - native input[type=color], sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::{ColorPickerPrimitive, ColorPickerSwatchPrimitive};

#[component]
pub fn ColorPicker(
    #[prop(into, default = "#000000".to_string())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let value2 = value.clone();
    view! {
        <div data-rs-color-picker-wrapper="">
            <div
                data-rs-color-picker=""
                style=format!("background-color: {}", value2)
            >
                <ColorPickerPrimitive
                    value=value
                    name=name
                    disabled=disabled
                    class=class
                />
            </div>
        </div>
    }
}

#[component]
pub fn ColorPickerSwatch(
    #[prop(into)] color: String,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ColorPickerSwatchPrimitive color=color selected=selected class=class />
    }
}

#[component]
pub fn ColorPickerPreview() -> impl IntoView {
    view! {
        <ColorPicker value="#3b82f6" />
    }
}
