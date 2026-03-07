//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pulse Primitive - Motion indicator

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PulseVariant {
    #[default]
    Default,
    Subtle,
    Emphasized,
}

impl PulseVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Subtle => "subtle",
            Self::Emphasized => "emphasized",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PulseSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl PulseSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PulseSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

impl PulseSpeed {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Slow => "slow",
            Self::Normal => "normal",
            Self::Fast => "fast",
        }
    }
}

#[component]
pub fn PulsePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = PulseVariant::Default)] variant: PulseVariant,
    #[prop(default = PulseSize::Medium)] size: PulseSize,
    #[prop(default = PulseSpeed::Normal)] speed: PulseSpeed,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <span
            data-pulse=""
            data-variant={variant.as_str()}
            data-size={size.as_str()}
            data-speed={speed.as_str()}
            aria-hidden="true"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </span>
    }
}
