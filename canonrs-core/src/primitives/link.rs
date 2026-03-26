//! @canon-level: strict
//! @canon-owner: primitives-team
//! Link Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::DisabledState;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum LinkVariant {
    #[default]
    Default,
    Muted,
    Underline,
}
impl LinkVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default   => "default",
            Self::Muted     => "muted",
            Self::Underline => "underline",
        }
    }
}

#[component]
pub fn LinkPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = LinkVariant::Default)] variant: LinkVariant,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = false)] external: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let target = if external { "_blank" } else { "" };
    let rel    = if external { "noopener noreferrer" } else { "" };
    view! {
        <a
            data-rs-link=""
            data-rs-component="Link"
            data-rs-behavior="action"
            data-rs-variant=variant.as_str()
            data-rs-state=disabled.as_str()
            href=href
            target=target
            rel=rel
            aria-disabled=disabled.aria()
            class=class
        >
            {children()}
        </a>
    }
}
