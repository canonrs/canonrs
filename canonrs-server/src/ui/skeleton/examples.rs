use leptos::prelude::*;
use super::{Skeleton, SkeletonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:2rem;max-width:600px;">

            // Card Loading State
            <div style="display:flex;gap:1rem;">
                <Skeleton variant=SkeletonVariant::Circle class="w-12 h-12".to_string() />
                <div style="flex:1;display:flex;flex-direction:column;gap:0.5rem;">
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-3/4".to_string() />
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-full".to_string() />
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-2/3".to_string() />
                </div>
            </div>

            // Text Loading State
            <div style="display:flex;flex-direction:column;gap:0.5rem;">
                <Skeleton variant=SkeletonVariant::Text class="h-4 w-full".to_string() />
                <Skeleton variant=SkeletonVariant::Text class="h-4 w-full".to_string() />
                <Skeleton variant=SkeletonVariant::Text class="h-4 w-3/4".to_string() />
            </div>

            // Avatar + Name Loading
            <div style="display:flex;align-items:center;gap:1rem;">
                <Skeleton variant=SkeletonVariant::Circle class="w-16 h-16".to_string() />
                <div style="display:flex;flex-direction:column;gap:0.5rem;">
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-5 w-32".to_string() />
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-24".to_string() />
                </div>
            </div>

            // Table Row Loading
            <div style="display:flex;flex-direction:column;gap:0.75rem;">
                <div style="display:flex;gap:1rem;">
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                </div>
                <div style="display:flex;gap:1rem;">
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                    <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                </div>
            </div>

        </div>
    }
}
