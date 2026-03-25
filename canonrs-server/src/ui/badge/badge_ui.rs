//! @canon-level: ui
//! Badge - Declarative UI wrapper

use leptos::prelude::*;
use canonrs_core::primitives::BadgePrimitive;
pub use canonrs_core::primitives::BadgeVariant;

#[component]
pub fn Badge(
    children: Children,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = false)] interactive: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BadgePrimitive variant=variant interactive=interactive class={class.unwrap_or_default()}>
            {children()}
        </BadgePrimitive>
    }
}

#[component]
pub fn BadgePreview() -> impl IntoView {
    view! {
        <Badge variant=BadgeVariant::Default>"Default"</Badge>
    }
}
