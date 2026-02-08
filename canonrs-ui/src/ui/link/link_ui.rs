use leptos::prelude::*;
use crate::primitives::{LinkPrimitive, LinkVariant};

#[component]
pub fn Link(
    children: Children,
    #[prop(optional)] href: Option<String>,
    #[prop(default = LinkVariant::Default)] variant: LinkVariant,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] external: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <LinkPrimitive
            href={href.unwrap_or_default()}
            variant={variant}
            disabled={disabled}
            external={external}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {children()}
        </LinkPrimitive>
    }
}
