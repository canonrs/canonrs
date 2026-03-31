
use leptos::prelude::*;
use canonrs_core::primitives::{LinkPrimitive, LinkVariant};
use canonrs_core::meta::DisabledState;

#[component]
pub fn Link(
    children: Children,
    href: String,
    #[prop(default = LinkVariant::Default)] variant: LinkVariant,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] external: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let disabled_state = DisabledState::from(disabled);
    view! {
        <LinkPrimitive
            href=href
            variant=variant
            disabled=disabled_state
            external=external
            class=class
        >
            {children()}
        </LinkPrimitive>
    }
}
