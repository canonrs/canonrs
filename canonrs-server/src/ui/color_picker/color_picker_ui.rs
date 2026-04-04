
use leptos::prelude::*;
use canonrs_core::primitives::{ColorPickerPrimitive, ColorPickerTriggerPrimitive, ColorPickerInputPrimitive, ColorPickerSwatchPrimitive};
use canonrs_core::meta::{DisabledState, SelectionState, VisibilityState};

#[component]
pub fn ColorPicker(
    #[prop(into, default = "#000000".to_string())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ColorPickerPrimitive state=VisibilityState::Closed disabled=disabled class=class>
            <ColorPickerTriggerPrimitive state=VisibilityState::Closed disabled=disabled color=value.clone()>
                <ColorPickerInputPrimitive value=value name=name disabled=disabled />
            </ColorPickerTriggerPrimitive>
        </ColorPickerPrimitive>
    }
}

#[component]
pub fn ColorPickerSwatch(
    #[prop(into)] color: String,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ColorPickerSwatchPrimitive color=color selected=selected class=class />
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ColorFormat { #[default] Hex, Rgb, Hsl, Cmyk }
impl ColorFormat {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Hex => "hex", Self::Rgb => "rgb", Self::Hsl => "hsl", Self::Cmyk => "cmyk" }
    }
}

#[component]
pub fn ColorPickerDisplay(
    #[prop(into, default = "#3b82f6".to_string())] value: String,
    #[prop(default = ColorFormat::Hex)] format: ColorFormat,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-color-picker-display="" data-rs-format=format.as_str() class=class>
            <span data-rs-color-display-format="">{format.as_str().to_uppercase()}</span>
            <span data-rs-color-display-value="" data-rs-color-value=value.clone()>{value.clone()}</span>
        </div>
    }
}

#[component]
pub fn ColorPickerPreview() -> impl IntoView {
    view! {
        <ColorPicker value="#3b82f6" />
    }
}
