#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::SkeletonPrimitive;
pub use canonrs_core::primitives::SkeletonVariant;

#[component]
pub fn Skeleton(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = SkeletonVariant::Rectangle)] variant: SkeletonVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SkeletonPrimitive variant=variant class=class>
            {children.map(|c| c())}
        </SkeletonPrimitive>
    }
}

