//! @canon-level: strict
//! @canon-owner: primitives-team
//! Button Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::DisabledState;

#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    Primary,
}
impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Destructive => "destructive",
            Self::Outline     => "outline",
            Self::Secondary   => "secondary",
            Self::Ghost       => "ghost",
            Self::Link        => "link",
            Self::Primary     => "primary",
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonSize {
    #[default]
    Md,
    Sm,
    Lg,
    Icon,
}
impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Md   => "md",
            Self::Sm   => "sm",
            Self::Lg   => "lg",
            Self::Icon => "icon",
        }
    }
}

#[component]
pub fn ButtonPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-button=""
            data-rs-component="Button"
            data-ui-variant=variant.as_str()
            data-ui-size=size.as_str()
            disabled=disabled.as_bool()
            aria-disabled=disabled.aria()
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}
