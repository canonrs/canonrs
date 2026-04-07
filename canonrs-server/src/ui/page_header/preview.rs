use leptos::prelude::*;
use super::page_header_island::{
    PageHeaderIsland, PageHeaderBreadcrumbsIsland, PageHeaderContentIsland,
    PageHeaderTitleIsland, PageHeaderDescriptionIsland, PageHeaderActionsIsland,
    PageHeaderTabsIsland,
};

#[component]
pub fn PageHeaderShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <PageHeaderIsland>
                    <PageHeaderBreadcrumbsIsland>
                        <span>"Home"</span>
                        <span>" / "</span>
                        <span>"Components"</span>
                        <span>" / "</span>
                        <span>"PageHeader"</span>
                    </PageHeaderBreadcrumbsIsland>
                    <PageHeaderContentIsland>
                        <PageHeaderTitleIsland>"Page Title"</PageHeaderTitleIsland>
                        <PageHeaderDescriptionIsland>"Header structure enforced with explicit semantic regions."</PageHeaderDescriptionIsland>
                    </PageHeaderContentIsland>
                    <PageHeaderActionsIsland>
                        <span data-rs-button="">"Action"</span>
                    </PageHeaderActionsIsland>
                </PageHeaderIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Header structure enforced with explicit semantic regions."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-md);width:100%;">
                    <PageHeaderIsland>
                        <PageHeaderContentIsland>
                            <PageHeaderTitleIsland>"Title only"</PageHeaderTitleIsland>
                        </PageHeaderContentIsland>
                    </PageHeaderIsland>
                    <PageHeaderIsland>
                        <PageHeaderContentIsland>
                            <PageHeaderTitleIsland>"With description"</PageHeaderTitleIsland>
                            <PageHeaderDescriptionIsland>"A short description of this page."</PageHeaderDescriptionIsland>
                        </PageHeaderContentIsland>
                    </PageHeaderIsland>
                    <PageHeaderIsland>
                        <PageHeaderContentIsland>
                            <PageHeaderTitleIsland>"With tabs"</PageHeaderTitleIsland>
                        </PageHeaderContentIsland>
                        <PageHeaderTabsIsland>
                            <span>"Overview"</span>
                            <span>"Settings"</span>
                            <span>"Members"</span>
                        </PageHeaderTabsIsland>
                    </PageHeaderIsland>
                </div>
            </div>
        </div>
    }
}
