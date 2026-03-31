
use leptos::prelude::*;
use canonrs_core::primitives::{BadgePrimitive, BadgeInteractivity};
pub use canonrs_core::primitives::BadgeVariant;

#[component]
pub fn Badge(
    children: Children,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = BadgeInteractivity::Static)] interactivity: BadgeInteractivity,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BadgePrimitive variant=variant interactivity=interactivity class=class>
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
