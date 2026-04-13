use leptos::prelude::*;
use super::link_group_boundary::LinkGroup;
use crate::ui::nav_item::nav_item_boundary::NavItem;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn LinkGroupShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <LinkGroup label=std::sync::Arc::new(|| view! { "Product" }.into_any())>
                <NavItem label="Features"  href="/features"  active=true.into() />
                <NavItem label="Pricing"   href="/pricing" />
                <NavItem label="Changelog" href="/changelog" />
                <NavItem label="Roadmap"   href="/roadmap" />
            </LinkGroup>
            <p data-rs-showcase-preview-anchor="">
                "Grouped navigation structured with direction and labeling contract."
            </p>
        </Stack>
    }
}
