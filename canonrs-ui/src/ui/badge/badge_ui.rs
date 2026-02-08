use leptos::prelude::*;
use crate::primitives::BadgePrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Error,
    Outline,
}

impl BadgeVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "default",
            BadgeVariant::Primary => "primary",
            BadgeVariant::Success => "success",
            BadgeVariant::Warning => "warning",
            BadgeVariant::Error => "error",
            BadgeVariant::Outline => "outline",
        }
    }
}

#[component]
pub fn Badge(
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = false)] interactive: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <BadgePrimitive
            attr:data-badge=""
            attr:data-variant={variant.as_str()}
            attr:data-interactive={if interactive { "true" } else { "" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </BadgePrimitive>
    }
}
