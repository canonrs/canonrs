//! Skeleton Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::skeleton_ui::{
    Skeleton as SkeletonUi,
    SkeletonVariant
};

#[component]
pub fn Skeleton(
    #[prop(default = SkeletonVariant::Rectangle)] variant: SkeletonVariant,
    #[prop(into, default = String::new())] class:          String,
) -> impl IntoView {
    view! { <SkeletonUi variant=variant class=class /> }
}
