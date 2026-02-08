//! Button UI Component

use leptos::prelude::*;
use crate::primitives::ButtonPrimitive;

#[derive(Clone, Copy, Debug)]
pub enum ButtonVariant {
    Solid,
    Outline,
    Ghost,
    Danger,
    Success,
    Warning,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Danger => "danger",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Solid)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let variant_str = variant.as_str();
    let size_str = size.as_str();

    view! {
        <ButtonPrimitive
            attr:data-button=""
            attr:data-ui-variant=variant_str
            attr:data-ui-size=size_str
            class=class
            id=id
            disabled=disabled
        >
            {children()}
        </ButtonPrimitive>
    }
}
