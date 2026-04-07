//! @canon-level: strict
//! @canon-owner: primitives-team
//! Link Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::infra::state_engine::disabled_attrs;

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
    let d = disabled_attrs(disabled);
    let target = if external { "_blank" } else { "" };
    let rel    = if external { "noopener noreferrer" } else { "" };
    let state_str: Option<&'static str> = if disabled == DisabledState::Disabled { Some("disabled") } else { None };
    view! {
        <a
            data-rs-link=""
            data-rs-component="Link"
            data-rs-behavior="link"
            data-rs-variant=variant.as_str()
            data-rs-disabled=d.data_rs_disabled
            data-rs-state=state_str
            href=href
            target=target
            rel=rel
            aria-disabled=d.aria_disabled
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </a>
    }
}
