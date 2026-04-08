use leptos::prelude::*;
use super::breadcrumb_island::{
    BreadcrumbIsland, BreadcrumbItemIsland, BreadcrumbLinkIsland,
    BreadcrumbPageIsland, BreadcrumbSeparatorIsland,
};

#[component]
pub fn BreadcrumbShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <BreadcrumbIsland>
                    <BreadcrumbItemIsland>
                        <BreadcrumbLinkIsland href="/">"Home"</BreadcrumbLinkIsland>
                    </BreadcrumbItemIsland>
                    <BreadcrumbSeparatorIsland>"/"</BreadcrumbSeparatorIsland>
                    <BreadcrumbItemIsland>
                        <BreadcrumbLinkIsland href="/components">"Components"</BreadcrumbLinkIsland>
                    </BreadcrumbItemIsland>
                    <BreadcrumbSeparatorIsland>"/"</BreadcrumbSeparatorIsland>
                    <BreadcrumbItemIsland>
                        <BreadcrumbPageIsland>"Breadcrumb"</BreadcrumbPageIsland>
                    </BreadcrumbItemIsland>
                </BreadcrumbIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Current page state enforced via activity state mapping."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Custom separator"</span>
                <div data-rs-showcase-preview-row="">
                    <BreadcrumbIsland>
                        <BreadcrumbItemIsland>
                            <BreadcrumbLinkIsland href="/">"Home"</BreadcrumbLinkIsland>
                        </BreadcrumbItemIsland>
                        <BreadcrumbSeparatorIsland>"›"</BreadcrumbSeparatorIsland>
                        <BreadcrumbItemIsland>
                            <BreadcrumbPageIsland>"Current"</BreadcrumbPageIsland>
                        </BreadcrumbItemIsland>
                    </BreadcrumbIsland>
                </div>
            </div>
        </div>
    }
}
