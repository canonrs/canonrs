use leptos::prelude::*;
use crate::primitives::InputPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant {
    Default,
    Error,
    Success,
}

impl InputVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Error => "error",
            Self::Success => "success",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputSize {
    Sm,
    Md,
    Lg,
}

impl InputSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

#[component]
pub fn Input(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = InputVariant::Default)] variant: InputVariant,
    #[prop(default = InputSize::Md)] size: InputSize,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] aria_label: String,
) -> impl IntoView {
    let base_class = format!(
        "input variant-{} size-{} {}",
        variant.as_str(),
        size.as_str(),
        class
    );

    view! {
        <InputPrimitive
            attr:data-input=""
            attr:data-variant={variant.as_str()}
            attr:data-size={size.as_str()}
            class={base_class}
            id={id}
            input_type={input_type}
            name={name}
            value={value}
            disabled={disabled}
            placeholder={placeholder}
            aria_label={aria_label}
        />
    }
}
