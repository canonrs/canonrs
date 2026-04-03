
use leptos::prelude::*;
use canonrs_core::primitives::{ButtonPrimitive, ButtonVariant as CoreVariant, ButtonSize as CoreSize};
use canonrs_core::DisabledState;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize { Xs, Sm, Md, Lg, Xl }

impl ButtonSize {
    pub fn to_core(&self) -> CoreSize {
        match self {
            Self::Xs => CoreSize::Xs,
            Self::Sm => CoreSize::Sm,
            Self::Lg => CoreSize::Lg,
            Self::Xl => CoreSize::Xl,
            Self::Md            => CoreSize::Md,
        }
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            class=class
            disabled=disabled
            aria_label=aria_label.unwrap_or_default()
            variant=variant.to_core()
            size=size.to_core()
        >
            {children()}
        </ButtonPrimitive>
    }
}

#[component]
pub fn ButtonPreview() -> impl IntoView {
    view! {
        <Button>"Primary"</Button>
        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
        <Button variant=ButtonVariant::Outline>"Outline"</Button>
        <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
        <Button disabled=DisabledState::Disabled>"Disabled"</Button>
    }
}
