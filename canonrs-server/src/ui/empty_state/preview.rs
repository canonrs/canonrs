use canonrs_core::primitives::EmptyStateVariant;
use leptos::prelude::*;
use super::empty_state_island::{
    EmptyStateIsland, EmptyStateIconIsland, EmptyStateTitleIsland,
    EmptyStateDescriptionIsland, EmptyStateActionIsland,
};

#[component]
pub fn EmptyStateShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <EmptyStateIsland>
                    <EmptyStateIconIsland>"📭"</EmptyStateIconIsland>
                    <EmptyStateTitleIsland>"No items found"</EmptyStateTitleIsland>
                    <EmptyStateDescriptionIsland>"Try adjusting your search or filters."</EmptyStateDescriptionIsland>
                    <EmptyStateActionIsland>"Clear filters"</EmptyStateActionIsland>
                </EmptyStateIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Empty state intent and variant enforced via contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <EmptyStateIsland>
                        <EmptyStateIconIsland>"🗂️"</EmptyStateIconIsland>
                        <EmptyStateTitleIsland>"Default"</EmptyStateTitleIsland>
                        <EmptyStateDescriptionIsland>"Nothing here yet."</EmptyStateDescriptionIsland>
                    </EmptyStateIsland>
                    <EmptyStateIsland variant=EmptyStateVariant::NoResults>
                        <EmptyStateIconIsland>"🔍"</EmptyStateIconIsland>
                        <EmptyStateTitleIsland>"No results"</EmptyStateTitleIsland>
                        <EmptyStateDescriptionIsland>"No matches for your query."</EmptyStateDescriptionIsland>
                    </EmptyStateIsland>
                    <EmptyStateIsland variant=EmptyStateVariant::NoData>
                        <EmptyStateIconIsland>"📊"</EmptyStateIconIsland>
                        <EmptyStateTitleIsland>"No data"</EmptyStateTitleIsland>
                        <EmptyStateDescriptionIsland>"Start by adding some entries."</EmptyStateDescriptionIsland>
                    </EmptyStateIsland>
                    <EmptyStateIsland variant=EmptyStateVariant::Error>
                        <EmptyStateIconIsland>"⚠️"</EmptyStateIconIsland>
                        <EmptyStateTitleIsland>"Something went wrong"</EmptyStateTitleIsland>
                        <EmptyStateDescriptionIsland>"Please try again later."</EmptyStateDescriptionIsland>
                        <EmptyStateActionIsland>"Retry"</EmptyStateActionIsland>
                    </EmptyStateIsland>
                </div>
            </div>
        </div>
    }
}
