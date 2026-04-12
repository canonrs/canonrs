#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{LogoPrimitive, LogoIconPrimitive, LogoWordmarkPrimitive, LogoTaglinePrimitive};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum LogoSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl LogoSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum LogoVariant {
    #[default]
    Brand,
    Neutral,
}

impl LogoVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand   => "brand",
            Self::Neutral => "neutral",
        }
    }
}

#[component]
pub fn Logo(
    #[prop(default = LogoSize::Md)] size: LogoSize,
    #[prop(default = LogoVariant::Brand)] variant: LogoVariant,
    #[prop(optional)] wordmark: Option<ChildrenFn>,
    #[prop(optional)] tagline: Option<ChildrenFn>,
    #[prop(into, default = "/".to_string())] href: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <LogoPrimitive
            href=href
            aria_label=aria_label.unwrap_or_default()
            size=size.as_str().to_string()
            variant=variant.as_str().to_string()
            class=class
        >
            <LogoIconPrimitive src="/logo_canonrs.svg".to_string() />
            {wordmark.map(|w| view! {
                <LogoWordmarkPrimitive>{w()}</LogoWordmarkPrimitive>
            })}
            {tagline.map(|t| view! {
                <LogoTaglinePrimitive>{t()}</LogoTaglinePrimitive>
            })}
        </LogoPrimitive>
    }
}
