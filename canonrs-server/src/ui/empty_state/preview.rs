use canonrs_core::primitives::EmptyStateVariant;
use leptos::prelude::*;
use super::boundary::{
    EmptyState, EmptyStateIcon, EmptyStateTitle,
    EmptyStateDescription, EmptyStateAction,
};

#[component]
pub fn EmptyStateShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <EmptyState>
                    <EmptyStateIcon>"📭"</EmptyStateIcon>
                    <EmptyStateTitle>"No items found"</EmptyStateTitle>
                    <EmptyStateDescription>"Try adjusting your search or filters."</EmptyStateDescription>
                    <EmptyStateAction>"Clear filters"</EmptyStateAction>
                </EmptyState>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Empty state intent and variant enforced via contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <EmptyState>
                        <EmptyStateIcon>"🗂️"</EmptyStateIcon>
                        <EmptyStateTitle>"Default"</EmptyStateTitle>
                        <EmptyStateDescription>"Nothing here yet."</EmptyStateDescription>
                    </EmptyState>
                    <EmptyState variant=EmptyStateVariant::NoResults>
                        <EmptyStateIcon>"🔍"</EmptyStateIcon>
                        <EmptyStateTitle>"No results"</EmptyStateTitle>
                        <EmptyStateDescription>"No matches for your query."</EmptyStateDescription>
                    </EmptyState>
                    <EmptyState variant=EmptyStateVariant::NoData>
                        <EmptyStateIcon>"📊"</EmptyStateIcon>
                        <EmptyStateTitle>"No data"</EmptyStateTitle>
                        <EmptyStateDescription>"Start by adding some entries."</EmptyStateDescription>
                    </EmptyState>
                    <EmptyState variant=EmptyStateVariant::Error>
                        <EmptyStateIcon>"⚠️"</EmptyStateIcon>
                        <EmptyStateTitle>"Something went wrong"</EmptyStateTitle>
                        <EmptyStateDescription>"Please try again later."</EmptyStateDescription>
                        <EmptyStateAction>"Retry"</EmptyStateAction>
                    </EmptyState>
                </div>
            </div>
        </div>
    }
}
