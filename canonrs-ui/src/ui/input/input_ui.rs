use leptos::prelude::*;
use crate::primitives::InputPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant { Default, Error, Success, Warning }
impl InputVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Error => "error",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputSize { Sm, Md, Lg }
impl InputSize {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Sm => "sm", Self::Md => "md", Self::Lg => "lg" }
    }
}

#[component]
pub fn Input(
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(into, optional)] name: Option<String>,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = InputVariant::Default)] variant: InputVariant,
    #[prop(default = InputSize::Md)] size: InputSize,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <InputPrimitive
            attr:data-input=""
            attr:data-variant={variant.as_str()}
            attr:data-size={size.as_str()}
            class={class}
            id={id.unwrap_or_default()}
            name={name.unwrap_or_default()}
            value={value}
            disabled={disabled}
            placeholder={placeholder}
            aria_label={aria_label}
        />
    }
}
