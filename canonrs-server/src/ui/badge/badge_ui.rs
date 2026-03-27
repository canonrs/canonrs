//! @canon-id: badge
//! @canon-label: Badge
//! @canon-family: data_display
//! @canon-category: Display
//! @canon-intent: Display status, count or label
//! @canon-description: Status badge label
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: badge, tag, status, label, notification

use leptos::prelude::*;
use canonrs_core::primitives::{BadgePrimitive, BadgeInteractivity};
pub use canonrs_core::primitives::BadgeVariant;

#[component]
pub fn Badge(
    children: Children,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = false)] interactive: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let interactivity = if interactive {
        BadgeInteractivity::Interactive
    } else {
        BadgeInteractivity::Static
    };
    view! {
        <BadgePrimitive variant=variant interactivity=interactivity class={class.unwrap_or_default()}>
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
