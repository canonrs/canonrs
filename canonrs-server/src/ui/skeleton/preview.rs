use leptos::prelude::*;
use super::skeleton_island::SkeletonIsland;
use super::skeleton_ui::SkeletonVariant;

#[component]
pub fn SkeletonShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);width:100%;">
                    <SkeletonIsland variant=SkeletonVariant::Rectangle />
                    <SkeletonIsland variant=SkeletonVariant::Text />
                    <SkeletonIsland variant=SkeletonVariant::Circle />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Placeholder de carregamento com variantes padronizadas via enum."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);width:100%;">
                    <SkeletonIsland variant=SkeletonVariant::Rectangle />
                    <SkeletonIsland variant=SkeletonVariant::Text />
                    <SkeletonIsland variant=SkeletonVariant::Circle />
                </div>
            </div>
        </div>
    }
}
