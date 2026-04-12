use leptos::prelude::*;
use canonrs_core::primitives::{ButtonPrimitive, ButtonVariant as CoreVariant, ButtonSize as CoreSize};
use canonrs_core::DisabledState;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonVariant {
    #[default] Default,
    Primary,
    Secondary,
    Outline,
    Ghost,
    Link,
    Destructive,
}

impl ButtonVariant {
    pub fn to_core(&self) -> CoreVariant {
        match self {
            Self::Default     => CoreVariant::Default,
            Self::Primary     => CoreVariant::Primary,
            Self::Secondary   => CoreVariant::Secondary,
            Self::Outline     => CoreVariant::Outline,
            Self::Ghost       => CoreVariant::Ghost,
            Self::Link        => CoreVariant::Link,
            Self::Destructive => CoreVariant::Destructive,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonSize { Xs, Sm, #[default] Md, Lg, Xl }

impl ButtonSize {
    pub fn to_core(&self) -> CoreSize {
        match self {
            Self::Xs => CoreSize::Xs,
            Self::Sm => CoreSize::Sm,
            Self::Md => CoreSize::Md,
            Self::Lg => CoreSize::Lg,
            Self::Xl => CoreSize::Xl,
        }
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };

    view! {
        <ButtonPrimitive
            class=class.unwrap_or_default()
            disabled=disabled_state
            aria_label=aria_label.unwrap_or_default()
            variant=variant.to_core()
            size=size.to_core()
        >
            {children()}
        </ButtonPrimitive>
    }
}
