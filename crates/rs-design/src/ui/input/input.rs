use leptos::prelude::*;

use super::variants::{InputSize, InputValidation};
use super::types::InputType;

const BASE_CLASSES: &str = "\
    w-full \
    rounded-md \
    border \
    shadow-sm \
    bg-background \
    text-foreground \
    transition-all \
    focus-visible:outline-none \
    focus-visible:ring-1 \
    disabled:cursor-not-allowed \
    placeholder:text-muted-foreground";

#[component]
pub fn Input(
    #[prop(optional, into)] id: String,
    #[prop(default = InputType::Text)] input_type: InputType,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional)] value: RwSignal<String>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional)] size: Option<InputSize>,
    #[prop(default = InputValidation::None)] validation: InputValidation,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let size = size.unwrap_or_default();

    let input_handler = move |ev| {
        let val = event_target_value(&ev);
        value.set(val.clone());
        if let Some(cb) = on_input {
            cb.run(val);
        }
    };

    let classes = format!(
        "{} {} {} {}",
        BASE_CLASSES,
        size.classes(),
        validation.border_classes(),
        class
    );
    
    let inline_style = format!(
        "font-family: var(--font-family-sans); \
         font-weight: var(--font-weight-regular); \
         line-height: var(--line-height-normal); \
         {}",
        size.style()
    );

    view! {
        <input
            id=id
            type=input_type.as_str()
            placeholder=placeholder
            value=move || value.get()
            on:input=input_handler
            disabled=disabled
            class=classes
            style=inline_style
        />
    }
}
