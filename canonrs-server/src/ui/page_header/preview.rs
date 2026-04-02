use leptos::prelude::*;
use super::page_header_ui::{
    PageHeader, PageHeaderBreadcrumbs, PageHeaderContent,
    PageHeaderTitle, PageHeaderDescription, PageHeaderActions, PageHeaderTabs,
};

#[component]
pub fn PageHeaderShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <PageHeader>
                    <PageHeaderBreadcrumbs>
                        <span>"Home"</span>
                        <span>" / "</span>
                        <span>"Components"</span>
                        <span>" / "</span>
                        <span>"PageHeader"</span>
                    </PageHeaderBreadcrumbs>
                    <PageHeaderContent>
                        <PageHeaderTitle>"Page Title"</PageHeaderTitle>
                        <PageHeaderDescription>"Header structure enforced with explicit semantic regions."</PageHeaderDescription>
                    </PageHeaderContent>
                    <PageHeaderActions>
                        <span data-rs-button="">"Action"</span>
                    </PageHeaderActions>
                </PageHeader>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Header structure enforced with explicit semantic regions."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-md);width:100%;">
                    <PageHeader>
                        <PageHeaderContent>
                            <PageHeaderTitle>"Title only"</PageHeaderTitle>
                        </PageHeaderContent>
                    </PageHeader>
                    <PageHeader>
                        <PageHeaderContent>
                            <PageHeaderTitle>"With description"</PageHeaderTitle>
                            <PageHeaderDescription>"A short description of this page."</PageHeaderDescription>
                        </PageHeaderContent>
                    </PageHeader>
                    <PageHeader>
                        <PageHeaderContent>
                            <PageHeaderTitle>"With tabs"</PageHeaderTitle>
                        </PageHeaderContent>
                        <PageHeaderTabs>
                            <span>"Overview"</span>
                            <span>"Settings"</span>
                            <span>"Members"</span>
                        </PageHeaderTabs>
                    </PageHeader>
                </div>
            </div>
        </div>
    }
}
