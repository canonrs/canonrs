use leptos::prelude::*;
use super::section_island::{
    SectionHeaderIsland, SectionTitleIsland, SectionSubtitleIsland,
    SectionBadgeIsland, SectionActionsIsland,
};

#[component]
pub fn SectionShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SectionHeaderIsland>
                    <SectionTitleIsland>"Section Title"</SectionTitleIsland>
                    <SectionSubtitleIsland>"Section subtitle description."</SectionSubtitleIsland>
                </SectionHeaderIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Semantic layout components for composing section headers."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Header with badge"</span>
                <div data-rs-showcase-preview-row="">
                    <SectionHeaderIsland>
                        <SectionTitleIsland>"Users"</SectionTitleIsland>
                        <SectionBadgeIsland>"New"</SectionBadgeIsland>
                    </SectionHeaderIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Header with actions"</span>
                <div data-rs-showcase-preview-row="">
                    <SectionHeaderIsland>
                        <SectionTitleIsland>"Reports"</SectionTitleIsland>
                        <SectionSubtitleIsland>"Monthly overview."</SectionSubtitleIsland>
                        <SectionActionsIsland>
                            <span>"Export"</span>
                        </SectionActionsIsland>
                    </SectionHeaderIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Title only"</span>
                <div data-rs-showcase-preview-row="">
                    <SectionTitleIsland>"Standalone Title"</SectionTitleIsland>
                </div>
            </div>
        </div>
    }
}
