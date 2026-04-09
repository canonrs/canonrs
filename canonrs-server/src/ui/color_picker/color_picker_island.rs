//! @canon-level: strict
//! ColorPicker Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::color_picker_ui::{ColorPicker, ColorPickerSwatch, ColorPickerDisplay, ColorFormat};
use canonrs_core::meta::{DisabledState, SelectionState};



#[component]
pub fn ColorPickerIsland(
    #[prop(into, default = "#000000".to_string())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ColorPicker value=value name=name disabled=disabled class=class />
    }
}

#[component]
pub fn ColorPickerSwatchIsland(
    #[prop(into)] color: String,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ColorPickerSwatch color=color selected=selected class=class /> }
}

#[component]
pub fn ColorPickerDisplayIsland(
    #[prop(into, default = "#3b82f6".to_string())] value: String,
    #[prop(default = ColorFormat::Hex)] format: ColorFormat,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ColorPickerDisplay value=value format=format class=class /> }
}
