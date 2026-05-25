use leptos::prelude::*;
use super::badge_ui::Badge as BadgeUi;
pub use canonrs_core::primitives::{
    BadgeVariant,
    BadgeInteractivity
};

#[component]
pub fn Badge(
    children: Children,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = BadgeInteractivity::Static)] interactivity: BadgeInteractivity,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] hidden: bool,
) -> impl IntoView {
    view! {
        <BadgeUi variant=variant interactivity=interactivity class=class attr:hidden=hidden.then(|| "")>
            {children()}
        </BadgeUi>
    }
}
