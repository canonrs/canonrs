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
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = false)] external: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let target = if external { "_blank" } else { "" };
    let rel = if external { "noopener noreferrer" } else { "" };
    view! {
        <a
            data-rs-link=""
            data-rs-variant={variant.as_str()}
            data-rs-state={move || if disabled.get() { "disabled" } else { "default" }}
            href=href
            target=target
            rel=rel
            aria-disabled={move || if disabled.get() { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}
