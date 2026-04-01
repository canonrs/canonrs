
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
            <ColorPickerTriggerPrimitive state=VisibilityState::Closed disabled=disabled>
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

#[component]
pub fn ColorPickerPreview() -> impl IntoView {
    view! {
        <ColorPicker value="#3b82f6" />
    }
}
