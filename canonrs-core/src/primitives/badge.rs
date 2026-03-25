//! @canon-level: strict
//! @canon-owner: primitives-team
//! Badge Primitive - HTML puro

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
            Self::Default => "default",
            Self::Primary => "primary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Destructive => "destructive",
            Self::Outline => "outline",
        }
    }
}

#[component]
pub fn BadgePrimitive(
    children: Children,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = false)] interactive: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-badge=""
            data-rs-variant=variant.as_str()
            data-rs-interactive=if interactive { Some("true") } else { None }
            class=class
        >
            {children()}
        </span>
    }
}
