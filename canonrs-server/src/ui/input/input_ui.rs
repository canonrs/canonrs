use leptos::prelude::*;
use canonrs_core::primitives::{InputPrimitive, InputVariant, InputSize};
use canonrs_core::meta::DisabledState;

#[component]
pub fn Input(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = "text".to_string())] input_type: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = InputVariant::Default)] variant: InputVariant,
    #[prop(default = InputSize::Md)] size: InputSize,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <InputPrimitive
            class=class
            name=name
            value=value
            disabled=disabled
            placeholder=placeholder
            aria_label=aria_label
            input_type=input_type
            variant=variant
            size=size
        />
    }
}

#[component]
pub fn InputPreview() -> impl IntoView {
    view! { <Input placeholder="Type something..." /> }
}
