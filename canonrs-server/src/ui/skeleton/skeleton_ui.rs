//! @canon-id: skeleton
//! @canon-label: Skeleton
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Show placeholder while content loads
//! @canon-description: Loading skeleton placeholder
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: skeleton, loading, placeholder, shimmer

use leptos::prelude::*;
use canonrs_core::primitives::SkeletonPrimitive;
pub use canonrs_core::primitives::SkeletonVariant;

#[component]
pub fn Skeleton(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = SkeletonVariant::Rectangle)] variant: SkeletonVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <SkeletonPrimitive variant=variant class={class.unwrap_or_default()}>
            {children.map(|c| c())}
        </SkeletonPrimitive>
    }
}

#[component]
pub fn SkeletonPreview() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:0.5rem;max-width:300px;">
            <Skeleton variant=SkeletonVariant::Rectangle />
            <Skeleton variant=SkeletonVariant::Text />
            <Skeleton variant=SkeletonVariant::Circle />
        </div>
    }
}
