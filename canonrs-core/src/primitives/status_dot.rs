//! @canon-level: strict
//! @canon-owner: primitives-team
//!
//! StatusDot Primitive - User Presence Indicator
//!
//! Domain: User presence/availability ONLY
//! NOT semantic feedback (success/error/warning)
//! Use Badge for semantic states
//! Use InlineNotice for feedback states

use leptos::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub enum StatusDotVariant {
    Online,
    #[default]
    Offline,
    Away,
    Busy,
    DoNotDisturb,
}

impl StatusDotVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online        => "online",
            Self::Offline       => "offline",
            Self::Away          => "away",
            Self::Busy          => "busy",
            Self::DoNotDisturb  => "do-not-disturb",
        }
    }

    pub fn state(&self) -> &'static str {
        match self {
            Self::Online => "active",
            _            => "inactive",
        }
    }
    pub fn aria_label(&self) -> &'static str {
        match self {
            Self::Online       => "Online",
            Self::Offline      => "Offline",
            Self::Away         => "Away",
            Self::Busy         => "Busy",
            Self::DoNotDisturb => "Do not disturb",
        }
    }
}

#[component]
pub fn StatusDotPrimitive(
    children: Children,
    #[prop(default = StatusDotVariant::Offline)] variant: StatusDotVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-status-dot=""
            data-rs-uid=crate::infra::uid::generate("sd")
            data-rs-interaction="init"
            data-rs-component="StatusDot"
            data-rs-variant=variant.as_str()
            role="img"
            aria-label=variant.aria_label()
            class=class
        >
            {children()}
        </span>
    }
}
