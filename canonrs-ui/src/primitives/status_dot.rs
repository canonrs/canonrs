//! @canon-level: strict
//! @canon-owner: primitives-team
//! StatusDot Primitive - User presence indicator

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Away => "away",
            Self::Busy => "busy",
            Self::DoNotDisturb => "donotdisturb",
        }
    }
}

#[component]
pub fn StatusDotPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = StatusDotVariant::Offline)] variant: StatusDotVariant,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <span
            data-status-dot=""
            data-variant={variant.as_str()}
            role="img"
            aria-label=aria_label
            class=class
            id=id
        >
            {children.map(|c| c())}
        </span>
    }
}
