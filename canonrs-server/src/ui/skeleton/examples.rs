use leptos::prelude::*;
use super::{Skeleton, SkeletonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem; max-width: 600px;">
            // Card Loading State
            <div>
                <p class="text-sm font-semibold mb-3".to_string()>"Loading Card"</p>
                <div class="border rounded-lg p-4".to_string() style="display: flex; gap: 1rem;">
                    <Skeleton variant=SkeletonVariant::Circle class="w-12 h-12".to_string() />
                    <div style="flex: 1; display: flex; flex-direction: column; gap: 0.5rem;">
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-3/4".to_string() />
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-full".to_string() />
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-2/3".to_string() />
                    </div>
                </div>
            </div>

            // Text Loading State
            <div>
                <p class="text-sm font-semibold mb-3".to_string()>"Loading Paragraph"</p>
                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    <Skeleton variant=SkeletonVariant::Text class="h-4 w-full".to_string() />
                    <Skeleton variant=SkeletonVariant::Text class="h-4 w-full".to_string() />
                    <Skeleton variant=SkeletonVariant::Text class="h-4 w-3/4".to_string() />
                </div>
            </div>

            // Avatar + Name Loading
            <div>
                <p class="text-sm font-semibold mb-3".to_string()>"Loading User Profile"</p>
                <div style="display: flex; align-items: center; gap: 1rem;">
                    <Skeleton variant=SkeletonVariant::Circle class="w-16 h-16".to_string() />
                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-5 w-32".to_string() />
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-4 w-24".to_string() />
                    </div>
                </div>
            </div>

            // Table Row Loading
            <div>
                <p class="text-sm font-semibold mb-3".to_string()>"Loading Table Rows"</p>
                <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                    <div style="display: flex; gap: 1rem;">
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                    </div>
                    <div style="display: flex; gap: 1rem;">
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                        <Skeleton variant=SkeletonVariant::Rectangle class="h-10 flex-1".to_string() />
                    </div>
                </div>
            </div>
        </div>
    }
}
