//! @canon-level: strict
//! @canon-owner: primitives-team
//! Link Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::DisabledState;

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Serialize, serde::Deserialize)]
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
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::A>>,
) -> impl IntoView {
    let uid_lnk = crate::infra::uid::generate("lnk");
    let target = if external { "_blank" } else { "" };
    let rel    = if external { "noopener noreferrer" } else { "" };
    view! {
        <a
            data-rs-link=""
            data-rs-uid=uid_lnk
            data-rs-variant=variant.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            href=href
            target=target
            rel=rel
            aria-disabled=disabled.aria_disabled()
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </a>
    }
}
