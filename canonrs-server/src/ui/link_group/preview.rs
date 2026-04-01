use leptos::prelude::*;
use super::link_group_ui::{LinkGroup, LinkGroupDirection};
use crate::ui::nav_item::NavItem;
use canonrs_core::meta::ActivityState;

#[component]
pub fn LinkGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <LinkGroup label=std::sync::Arc::new(|| view! { "Product" }.into_any())>
                    <NavItem label="Features".to_string() href="/features".to_string() active=ActivityState::Active />
                    <NavItem label="Pricing".to_string() href="/pricing".to_string() />
                    <NavItem label="Changelog".to_string() href="/changelog".to_string() />
                    <NavItem label="Roadmap".to_string() href="/roadmap".to_string() />
                </LinkGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Grouped navigation structured with direction and labeling contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Horizontal"</span>
                <div data-rs-showcase-preview-row="">
                    <LinkGroup
                        direction=LinkGroupDirection::Horizontal
                        label=std::sync::Arc::new(|| view! { "Company" }.into_any())
                    >
                        <NavItem label="About".to_string() href="/about".to_string() />
                        <NavItem label="Blog".to_string() href="/blog".to_string() />
                        <NavItem label="Careers".to_string() href="/careers".to_string() />
                    </LinkGroup>
                </div>
            </div>
        </div>
    }
}
