//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Solid,
    Outline,
    Ghost,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn ButtonPrimitive(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(default = "submit".to_string(), into)] button_type: String,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Solid);
    let size = size.unwrap_or(ButtonSize::Md);
    let disabled = disabled.unwrap_or(false);

    view! {
        <button
            type=button_type
            disabled=disabled
            class=class
            data-variant=match variant {
                ButtonVariant::Solid => "solid",
                ButtonVariant::Outline => "outline",
                ButtonVariant::Ghost => "ghost",
            }
            data-size=match size {
                ButtonSize::Sm => "sm",
                ButtonSize::Md => "md",
                ButtonSize::Lg => "lg",
            }
        >
            {children()}
        </button>
    }
}
