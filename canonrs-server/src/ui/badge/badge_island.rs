use leptos::prelude::*;
use super::badge_ui::{Badge, BadgeVariant};
use canonrs_core::primitives::BadgeInteractivity;

#[component]
pub fn BadgeIsland(
    children: Children,
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] interactivity: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant_val = match variant.as_deref() {
        Some("primary")     => BadgeVariant::Primary,
        Some("success")     => BadgeVariant::Success,
        Some("warning")     => BadgeVariant::Warning,
        Some("destructive") => BadgeVariant::Destructive,
        Some("outline")     => BadgeVariant::Outline,
        _                   => BadgeVariant::Default,
    };
    let interactivity_val = match interactivity.as_deref() {
        Some("interactive") => BadgeInteractivity::Interactive,
        _                   => BadgeInteractivity::Static,
    };
    view! {
        <Badge variant=variant_val interactivity=interactivity_val class=class.unwrap_or_default()>
            {children()}
        </Badge>
    }
}
