//! Input Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::input_ui::Input;
use canonrs_core::primitives::{InputVariant, InputSize};
use canonrs_core::meta::DisabledState;

#[component]
pub fn InputIsland(
    #[prop(into, default = String::new())] class:       String,
    #[prop(into, default = "text".to_string())] input_type: String,
    #[prop(into, default = String::new())] name:        String,
    #[prop(into, default = String::new())] value:       String,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] aria_label:  String,
    #[prop(default = InputVariant::Default)] variant:   InputVariant,
    #[prop(default = InputSize::Md)] size:              InputSize,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! {
        <Input
            class=class
            input_type=input_type
            name=name
            value=value
            placeholder=placeholder
            aria_label=aria_label
            variant=variant
            size=size
            disabled=disabled
        />
    }
}
