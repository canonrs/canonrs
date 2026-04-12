use leptos::prelude::*;
use super::boundary::LinkGroup;
use crate::ui::nav_item::NavItem;

#[component]
pub fn LinkGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <LinkGroup label=std::sync::Arc::new(|| view! { "Product" }.into_any())>
                    <NavItem label="Features" href="/features" active=true.into() />
                    <NavItem label="Pricing" href="/pricing" />
                    <NavItem label="Changelog" href="/changelog" />
                    <NavItem label="Roadmap" href="/roadmap" />
                </LinkGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Grouped navigation structured with direction and labeling contract."
            </p>
        </div>
    }
}
