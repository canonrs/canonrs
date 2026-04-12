use leptos::prelude::*;
use super::boundary::{
    SectionHeader, SectionTitle, SectionSubtitle,
    SectionBadge, SectionActions,
};

#[component]
pub fn SectionShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SectionHeader>
                    <SectionTitle>"Section Title"</SectionTitle>
                    <SectionSubtitle>"Section subtitle description."</SectionSubtitle>
                </SectionHeader>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Semantic layout components for composing section headers."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Header with badge"</span>
                <div data-rs-showcase-preview-row="">
                    <SectionHeader>
                        <SectionTitle>"Users"</SectionTitle>
                        <SectionBadge>"New"</SectionBadge>
                    </SectionHeader>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Header with actions"</span>
                <div data-rs-showcase-preview-row="">
                    <SectionHeader>
                        <SectionTitle>"Reports"</SectionTitle>
                        <SectionSubtitle>"Monthly overview."</SectionSubtitle>
                        <SectionActions>
                            <span>"Export"</span>
                        </SectionActions>
                    </SectionHeader>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Title only"</span>
                <div data-rs-showcase-preview-row="">
                    <SectionTitle>"Standalone Title"</SectionTitle>
                </div>
            </div>
        </div>
    }
}
