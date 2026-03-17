use leptos::prelude::*;
use canonrs_core::primitives::ButtonPrimitive;

#[derive(Clone, Copy, Debug)]
pub enum ButtonVariant {
    Primary,
    #[allow(dead_code)] Solid,
    Secondary, Outline, Ghost, Link,
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
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            class={class}
            id={id.unwrap_or_default()}
            disabled=disabled
            aria_label={aria_label.unwrap_or_default()}
            data_variant={variant.as_str().to_string()}
            data_size={size.as_str().to_string()}
        >
            {children()}
        </ButtonPrimitive>
    }
}
