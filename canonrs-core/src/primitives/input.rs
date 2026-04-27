//! @canon-level: strict
//! @canon-owner: primitives-team
//! Input Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::DisabledState;

#[derive(Clone, PartialEq, Default, Debug)]
pub enum InputVariant {
    #[default]
    Default,
    Error,
    Success,
    Warning,
}
impl InputVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Error   => "error",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub enum InputSize {
    #[default]
    Md,
    Sm,
    Lg,
}
impl InputSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Md => "md",
            Self::Sm => "sm",
            Self::Lg => "lg",
        }
    }
}

#[component]
pub fn InputPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = "text".to_string())] input_type: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] rs_value: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(default = InputVariant::Default)] variant: InputVariant,
    #[prop(default = InputSize::Md)] size: InputSize,
) -> impl IntoView {
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <input
            data-rs-input=""
            data-rs-uid=crate::infra::uid::generate("inp")
            data-rs-interaction="init"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            type=input_type
            class=class
            name=name
            prop:value=value
            data-rs-value=rs_value
            placeholder=placeholder
            disabled=disabled.as_bool()
            aria-disabled=aria_disabled
            aria-label=aria_label
        />
    }
}
