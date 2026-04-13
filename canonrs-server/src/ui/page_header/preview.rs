use leptos::prelude::*;
use super::page_header_boundary::{
    PageHeader, PageHeaderBreadcrumbs, PageHeaderContent,
    PageHeaderTitle, PageHeaderDescription, PageHeaderActions,
    PageHeaderTabs,
};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn PageHeaderShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Header structure enforced with explicit semantic regions."
            </p>
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
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Md>
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
                </Stack>
            </Stack>
        </Stack>
    }
}
