use leptos::prelude::*;
use super::menu_island::{MenuIsland, MenuItemIsland};

#[component]
pub fn MenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <MenuIsland aria_label="Main menu">
                    <MenuItemIsland value="new">"New file"</MenuItemIsland>
                    <MenuItemIsland value="open">"Open file"</MenuItemIsland>
                    <MenuItemIsland value="save" selected=true>"Save"</MenuItemIsland>
                    <MenuItemIsland value="export" disabled=true>"Export (disabled)"</MenuItemIsland>
                </MenuIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction fully governed via DOM — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Edit menu"</span>
                <div data-rs-showcase-preview-row="">
                    <MenuIsland aria_label="Edit menu">
                        <MenuItemIsland value="cut">"Cut"</MenuItemIsland>
                        <MenuItemIsland value="copy">"Copy"</MenuItemIsland>
                        <MenuItemIsland value="paste">"Paste"</MenuItemIsland>
                        <MenuItemIsland value="undo">"Undo"</MenuItemIsland>
                        <MenuItemIsland value="redo">"Redo"</MenuItemIsland>
                    </MenuIsland>
                </div>
            </div>
        </div>
    }
}
