use leptos::prelude::*;
use canonrs_core::primitives::ButtonPrimitive;

#[derive(Clone, Copy, Debug)]
pub enum ButtonVariant {
    // Core
    Primary,
    #[allow(dead_code)] Solid,   // alias for Primary — backwards compat
    Secondary, Outline, Ghost, Link,
    // Semantic
    Danger, Success, Warning, Info,
    // Neutral
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
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonSize {
    Xs, Sm, Md, Lg, Xl,
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
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            attr:data-button=""
            attr:data-ui-variant={variant.as_str()}
            attr:data-ui-size={size.as_str()}
            class={class}
            id={id.unwrap_or_default()}
            disabled={disabled}
            aria_label={aria_label.unwrap_or_default()}
        >
            {children()}
        </ButtonPrimitive>
    }
}
