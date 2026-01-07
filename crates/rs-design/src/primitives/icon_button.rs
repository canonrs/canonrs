//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum IconButtonVariant {
    Solid,
    Outline,
    Ghost,
}

#[derive(Clone, Copy, PartialEq)]
pub enum IconButtonSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn IconButtonPrimitive(
    #[prop(optional)] variant: Option<IconButtonVariant>,
    #[prop(optional)] size: Option<IconButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(IconButtonVariant::Solid);
    let size = size.unwrap_or(IconButtonSize::Md);
    let disabled = disabled.unwrap_or(false);

    view! {
        <button
            disabled=disabled
            aria-label=aria_label
            data-variant=match variant {
                IconButtonVariant::Solid => "solid",
                IconButtonVariant::Outline => "outline",
                IconButtonVariant::Ghost => "ghost",
            }
            data-size=match size {
                IconButtonSize::Sm => "sm",
                IconButtonSize::Md => "md",
                IconButtonSize::Lg => "lg",
            }
        >
            {children()}
        </button>
    }
}
