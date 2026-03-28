//! @canon-id: input
//! @canon-label: Input
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Capture text or data from user
//! @canon-description: Text input field
//! @canon-composable: false
//! @canon-capabilities: Value, Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: input, field, text, form

use leptos::prelude::*;
use leptos::prelude::event_target_value;
use canonrs_core::primitives::{InputPrimitive, InputVariant, InputSize};

#[component]
pub fn Input(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = "text".to_string())] input_type: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = InputVariant::Default)] variant: InputVariant,
    #[prop(default = InputSize::Md)] size: InputSize,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <InputPrimitive
            class=class
            name=name
            prop:value=value
            disabled=disabled.into()
            placeholder=placeholder
            aria_label=aria_label
            input_type=input_type
            variant=variant
            size=size
            on:input=move |ev| {
                if let Some(cb) = on_input {
                    cb.run(event_target_value(&ev));
                }
            }
        />
    }
}

#[component]
pub fn InputPreview() -> impl IntoView {
    view! { <Input placeholder="Type something..." /> }
}
