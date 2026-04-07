use leptos::prelude::*;
use super::badge_ui::Badge;
use canonrs_core::primitives::{BadgeVariant, BadgeInteractivity};

#[component]
pub fn BadgeIsland(
    children: Children,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = BadgeInteractivity::Static)] interactivity: BadgeInteractivity,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Badge variant=variant interactivity=interactivity class=class>
            {children()}
        </Badge>
    }
}
