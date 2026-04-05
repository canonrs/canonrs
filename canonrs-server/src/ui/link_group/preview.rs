use leptos::prelude::*;
use super::link_group_island::{LinkGroupIsland, LinkGroupIslandItem, LinkGroupIslandDirection};

#[component]
pub fn LinkGroupShowcasePreview() -> impl IntoView {
    let product_items = vec![
        LinkGroupIslandItem { label: "Features".into(),  href: "/features".into(),  active: true,  disabled: false },
        LinkGroupIslandItem { label: "Pricing".into(),   href: "/pricing".into(),   active: false, disabled: false },
        LinkGroupIslandItem { label: "Changelog".into(), href: "/changelog".into(), active: false, disabled: false },
        LinkGroupIslandItem { label: "Roadmap".into(),   href: "/roadmap".into(),   active: false, disabled: false },
    ];
    let company_items = vec![
        LinkGroupIslandItem { label: "About".into(),   href: "/about".into(),   active: false, disabled: false },
        LinkGroupIslandItem { label: "Blog".into(),    href: "/blog".into(),    active: false, disabled: false },
        LinkGroupIslandItem { label: "Careers".into(), href: "/careers".into(), active: false, disabled: false },
    ];

    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <LinkGroupIsland items=product_items label="Product" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Grouped navigation structured with direction and labeling contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Horizontal"</span>
                <div data-rs-showcase-preview-row="">
                    <LinkGroupIsland
                        items=company_items
                        label="Company"
                        direction=LinkGroupIslandDirection::Horizontal
                    />
                </div>
            </div>
        </div>
    }
}
