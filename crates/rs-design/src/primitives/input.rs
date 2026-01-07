//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum InputSize {
    Sm,
    Md,
    Lg,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
}

#[component]
pub fn InputPrimitive(
    #[prop(optional)] input_type: Option<InputType>,
    #[prop(optional)] size: Option<InputSize>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] error: Option<bool>,
) -> impl IntoView {
    let input_type = input_type.unwrap_or(InputType::Text);
    let size = size.unwrap_or(InputSize::Md);
    let disabled = disabled.unwrap_or(false);
    let required = required.unwrap_or(false);
    let error = error.unwrap_or(false);

    view! {
        <input
            type=match input_type {
                InputType::Text => "text",
                InputType::Email => "email",
                InputType::Password => "password",
                InputType::Number => "number",
                InputType::Tel => "tel",
                InputType::Url => "url",
            }
            placeholder=placeholder
            disabled=disabled
            required=required
            data-size=match size {
                InputSize::Sm => "sm",
                InputSize::Md => "md",
                InputSize::Lg => "lg",
            }
            data-error=error
        />
    }
}
