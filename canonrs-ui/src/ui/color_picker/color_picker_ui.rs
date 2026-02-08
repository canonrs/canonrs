use leptos::prelude::*;
use crate::primitives::{
    ColorPickerPrimitive,
    ColorPickerSwatchPrimitive
};

#[component]
pub fn ColorPicker(
    #[prop(default = "#000000".to_string())] value: String,
    #[prop(optional)] name: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let name = name.unwrap_or_default();
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <ColorPickerPrimitive
            value={value}
            name={name}
            disabled={disabled}
            class={class}
            id={id}
        />
    }
}

#[component]
pub fn ColorPickerSwatch(
    color: String,
    #[prop(default = false)] selected: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <ColorPickerSwatchPrimitive
            color={color}
            selected={selected}
            class={class}
            id={id}
        />
    }
}
