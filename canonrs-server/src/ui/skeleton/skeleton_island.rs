//! Skeleton Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::skeleton_ui::{Skeleton, SkeletonVariant};

#[component]
pub fn SkeletonIsland(
    #[prop(default = SkeletonVariant::Rectangle)] variant: SkeletonVariant,
    #[prop(into, default = String::new())] class:          String,
) -> impl IntoView {
    view! { <Skeleton variant=variant class=class /> }
}
