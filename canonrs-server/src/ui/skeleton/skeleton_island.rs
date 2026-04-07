use leptos::prelude::*;
use super::skeleton_ui::{Skeleton, SkeletonVariant};

#[component]
pub fn SkeletonIsland(
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant = match variant.as_deref() {
        Some("text")    => SkeletonVariant::Text,
        Some("circle")  => SkeletonVariant::Circle,
        _               => SkeletonVariant::Rectangle,
    };
    let cls = class.unwrap_or_default();

    view! {
        <Skeleton variant=variant class=cls />
    }
}
