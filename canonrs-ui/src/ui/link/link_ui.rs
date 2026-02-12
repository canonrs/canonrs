use leptos::prelude::*;
use crate::primitives::{LinkPrimitive, LinkVariant};

#[component]
pub fn Link(
    children: Children,
    href: String,
    #[prop(default = LinkVariant::Default)] variant: LinkVariant,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] external: bool,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <LinkPrimitive
            href={href}
            variant={variant}
            disabled={disabled}
            external={external}
            class={class}
            id={id.unwrap_or_default()}
        >
            {children()}
        </LinkPrimitive>
    }
}
