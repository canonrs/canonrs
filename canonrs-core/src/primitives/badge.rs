//! @canon-level: strict
//! @canon-owner: primitives-team
//! Badge Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Primary     => "primary",
            Self::Success     => "success",
            Self::Warning     => "warning",
            Self::Destructive => "destructive",
            Self::Outline     => "outline",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeInteractivity {
    #[default]
    Static,
    Interactive,
}

impl BadgeInteractivity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Static      => "static",
            Self::Interactive => "interactive",
        }
    }
}

#[component]
pub fn BadgePrimitive(
    children: Children,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = BadgeInteractivity::Static)] interactivity: BadgeInteractivity,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-badge=""
            data-rs-component="Badge"
            data-rs-variant=variant.as_str()
            data-rs-interactivity=interactivity.as_str()
            aria-label=aria_label
            class=class
        >
            {children()}
        </span>
    }
}
