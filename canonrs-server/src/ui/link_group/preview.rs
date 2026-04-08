use leptos::prelude::*;
use super::link_group_island::LinkGroupIsland;
use crate::ui::nav_item::nav_item_island::NavItemIsland;

#[component]
pub fn LinkGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <LinkGroupIsland label=std::sync::Arc::new(|| view! { "Product" }.into_any())>
                    <NavItemIsland label="Features" href="/features" active=true />
                    <NavItemIsland label="Pricing" href="/pricing" />
                    <NavItemIsland label="Changelog" href="/changelog" />
                    <NavItemIsland label="Roadmap" href="/roadmap" />
                </LinkGroupIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Grouped navigation structured with direction and labeling contract."
            </p>
        </div>
    }
}
