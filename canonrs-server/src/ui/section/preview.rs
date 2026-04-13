use leptos::prelude::*;
use super::section_boundary::{
    SectionHeader, SectionTitle, SectionSubtitle,
    SectionBadge, SectionActions,
};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn SectionShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Semantic layout components for composing section headers."
            </p>
            <SectionHeader>
                <SectionTitle>"Section Title"</SectionTitle>
                <SectionSubtitle>"Section subtitle description."</SectionSubtitle>
            </SectionHeader>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Header with badge"</span>
                <SectionHeader>
                    <SectionTitle>"Users"</SectionTitle>
                    <SectionBadge>"New"</SectionBadge>
                </SectionHeader>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Header with actions"</span>
                <SectionHeader>
                    <SectionTitle>"Reports"</SectionTitle>
                    <SectionSubtitle>"Monthly overview."</SectionSubtitle>
                    <SectionActions>
                        <span>"Export"</span>
                    </SectionActions>
                </SectionHeader>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Title only"</span>
                <SectionTitle>"Standalone Title"</SectionTitle>
            </Stack>
        </Stack>
    }
}
