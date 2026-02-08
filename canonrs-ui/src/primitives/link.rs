//! @canon-level: strict
//! @canon-owner: primitives-team
//! Link Primitive - HTML puro

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum LinkVariant {
    Default,
    Muted,
    Underline,
}

impl LinkVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkVariant::Default => "default",
            LinkVariant::Muted => "muted",
            LinkVariant::Underline => "underline",
        }
    }
}

#[component]
pub fn LinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = LinkVariant::Default)] variant: LinkVariant,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] external: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        
        <a
            attr:data-link=""
            attr:data-variant={variant.as_str()}
            attr:data-disabled={disabled}
            href=href
            target={if external { "_blank" } else { "" }}
            rel={if external { "noopener noreferrer" } else { "" }}
            attr:aria-disabled={if disabled { "true" } else { "false" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </a>
    }
}
