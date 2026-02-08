use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::either::Either;
use leptos::ev;
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
    #[prop(optional)] on_input: Option<Callback<ev::Event>>,
    #[prop(optional)] on_change: Option<Callback<ev::Event>>,
    #[prop(optional)] on_blur: Option<Callback<ev::FocusEvent>>,
) -> impl IntoView {
    let base_class = format!(
        "input variant-{} size-{} {}",
        variant.as_str(),
        size.as_str(),
        class
    );

    if on_input.is_none() && on_change.is_none() && on_blur.is_none() {
        Either::Left(view! {
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
        })
    } else {
        Either::Right(view! {
            <InputPrimitive
                attr:data-input=""
                attr:data-variant={variant.as_str()}
                attr:data-size={size.as_str()}
                class={base_class.clone()}
                id={id.clone()}
                input_type={input_type.clone()}
                name={name.clone()}
                value={value.clone()}
                disabled={disabled}
                placeholder={placeholder.clone()}
                aria_label={aria_label.clone()}
                on:input=move |ev| if let Some(h) = &on_input { h.run(ev); }
                on:change=move |ev| if let Some(h) = &on_change { h.run(ev); }
                on:blur=move |ev| if let Some(h) = &on_blur { h.run(ev); }
            />
        })
    }
}
