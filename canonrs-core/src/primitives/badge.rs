//! @canon-level: strict
//! @canon-owner: primitives-team
//! Badge Primitive - HTML puro

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
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
            BadgeVariant::Default => "default",
            BadgeVariant::Primary => "primary",
            BadgeVariant::Success => "success",
            BadgeVariant::Warning => "warning",
            BadgeVariant::Destructive => "destructive",
            BadgeVariant::Outline => "outline",
        }
    }
}

#[component]
pub fn BadgePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = false)] interactive: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    let data_interactive = if interactive { Some("true") } else { None };
    view! {
        <span
            data-badge=""
            data-variant={variant.as_str()}
            data-interactive=data_interactive
            class=class
            id=id
        >
            {children.map(|c| c())}
        </span>
    }
}
