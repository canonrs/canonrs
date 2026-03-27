//! @canon-id: button
//! @canon-label: Button
//! @canon-family: interactive
//! @canon-category: Action
//! @canon-intent: Trigger an action or event
//! @canon-description: Action button with variant and size
//! @canon-composable: false
//! @canon-capabilities: Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: button, action, submit, click, cta

use leptos::prelude::*;
use canonrs_core::primitives::{ButtonPrimitive, ButtonVariant as CoreVariant, ButtonSize as CoreSize};
use canonrs_core::DisabledState;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Primary, Solid, Secondary, Outline, Ghost, Link,
    Danger, Success, Warning, Info,
    Default, Subtle, Muted,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Primary | Self::Solid => "primary",
            Self::Secondary => "secondary",
            Self::Outline   => "outline",
            Self::Ghost     => "ghost",
            Self::Link      => "link",
            Self::Danger    => "danger",
            Self::Success   => "success",
            Self::Warning   => "warning",
            Self::Info      => "info",
            Self::Default   => "default",
            Self::Subtle    => "subtle",
            Self::Muted     => "muted",
        }
    }
    pub fn to_core(&self) -> CoreVariant {
        match self {
            Self::Primary | Self::Solid => CoreVariant::Primary,
            Self::Secondary             => CoreVariant::Secondary,
            Self::Outline               => CoreVariant::Outline,
            Self::Ghost                 => CoreVariant::Ghost,
            Self::Link                  => CoreVariant::Link,
            Self::Danger                => CoreVariant::Destructive,
            _                           => CoreVariant::Default,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize { Xs, Sm, Md, Lg, Xl }

impl ButtonSize {
    pub fn to_core(&self) -> CoreSize {
        match self {
            Self::Xs | Self::Sm => CoreSize::Sm,
            Self::Lg | Self::Xl => CoreSize::Lg,
            Self::Md            => CoreSize::Md,
        }
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            class=class
            disabled=DisabledState::from(disabled)
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
        <Button disabled=true>"Disabled"</Button>
    }
}
