//! Link Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::link_ui::Link as LinkUi;
pub use canonrs_core::primitives::LinkVariant;

#[component]
pub fn Link(
    children: Children,
    #[prop(into, default = String::new())] href:    String,
    #[prop(default = LinkVariant::Default)] variant: LinkVariant,
    #[prop(default = false)] disabled:               bool,
    #[prop(default = false)] external:               bool,
    #[prop(into, default = String::new())] class:   String,
) -> impl IntoView {
    view! {
        <LinkUi href=href variant=variant disabled=disabled external=external class=class>
            {children()
};
        </LinkUi>
    }
}
