use leptos::prelude::*;
use super::navigation_menu_island::{NavigationMenuIsland, NavMenuIslandItem, NavMenuIslandLink};

#[component]
pub fn NavigationMenuShowcasePreview() -> impl IntoView {
    let items = vec![
        NavMenuIslandItem {
            trigger: "Products".into(),
            href: None,
            links: vec![
                NavMenuIslandLink { label: "CanonRS Core".into(),   href: "#".into() },
                NavMenuIslandLink { label: "CanonRS UI".into(),     href: "#".into() },
                NavMenuIslandLink { label: "CanonRS Tokens".into(), href: "#".into() },
            ],
        },
        NavMenuIslandItem {
            trigger: "Docs".into(),
            href: None,
            links: vec![
                NavMenuIslandLink { label: "Getting Started".into(), href: "#".into() },
                NavMenuIslandLink { label: "API Reference".into(),   href: "#".into() },
            ],
        },
        NavMenuIslandItem {
            trigger: "Pricing".into(),
            href: Some("#".into()),
            links: vec![],
        },
        NavMenuIslandItem {
            trigger: "Blog".into(),
            href: Some("#".into()),
            links: vec![],
        },
    ];

    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <NavigationMenuIsland items=items />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger-content relationship governed by signal — no id wiring needed."
            </p>
        </div>
    }
}
