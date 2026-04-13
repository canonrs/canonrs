use leptos::prelude::*;
use super::empty_state_boundary::{
    EmptyState, EmptyStateIcon, EmptyStateTitle,
    EmptyStateDescription, EmptyStateAction,
};
use canonrs_core::primitives::EmptyStateVariant;
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn EmptyStateShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <EmptyState>
                <EmptyStateIcon>"📭"</EmptyStateIcon>
                <EmptyStateTitle>"No items found"</EmptyStateTitle>
                <EmptyStateDescription>"Try adjusting your search or filters."</EmptyStateDescription>
                <EmptyStateAction>"Clear filters"</EmptyStateAction>
            </EmptyState>
            <p data-rs-showcase-preview-anchor="">
                "Empty state intent and variant enforced via contract."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Grid cols=GridCols::Two>
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
                </Grid>
            </Stack>
        </Stack>
    }
}
