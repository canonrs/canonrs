use leptos::prelude::*;
use super::sidebar_island::{SidebarIsland, SidebarIslandItem};

fn main_items() -> Vec<SidebarIslandItem> {
    vec![
        SidebarIslandItem { label: "Dashboard".into(),  href: "#".into(), active: true,  disabled: false, group: Some("Main".into()) },
        SidebarIslandItem { label: "Components".into(), href: "#".into(), active: false, disabled: false, group: Some("Main".into()) },
        SidebarIslandItem { label: "Tokens".into(),     href: "#".into(), active: false, disabled: false, group: Some("Main".into()) },
        SidebarIslandItem { label: "Preferences".into(),href: "#".into(), active: false, disabled: false, group: Some("Settings".into()) },
        SidebarIslandItem { label: "Team".into(),       href: "#".into(), active: false, disabled: false, group: Some("Settings".into()) },
    ]
}

fn collapsed_items() -> Vec<SidebarIslandItem> {
    vec![
        SidebarIslandItem { label: "Home".into(),  href: "#".into(), active: false, disabled: false, group: None },
        SidebarIslandItem { label: "About".into(), href: "#".into(), active: false, disabled: false, group: None },
    ]
}

#[component]
pub fn SidebarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SidebarIsland items=main_items() initial_open=true />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and structure governed by signal — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Collapsed"</span>
                <div data-rs-showcase-preview-row="">
                    <SidebarIsland items=collapsed_items() initial_open=false />
                </div>
            </div>
        </div>
    }
}
