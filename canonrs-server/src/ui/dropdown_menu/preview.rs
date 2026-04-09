use leptos::prelude::*;
use super::dropdown_menu_island::{DropdownMenuIsland, DropdownMenuItemIsland};
use canonrs_core::meta::DisabledState;

#[component]
pub fn DropdownMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DropdownMenuIsland trigger_label="Options ▼">
                    <DropdownMenuItemIsland>"Profile"</DropdownMenuItemIsland>
                    <DropdownMenuItemIsland>"Settings"</DropdownMenuItemIsland>
                    <DropdownMenuItemIsland>"Billing"</DropdownMenuItemIsland>
                    <DropdownMenuItemIsland>"Sign out"</DropdownMenuItemIsland>
                </DropdownMenuIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction governed by DOM — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With actions"</span>
                <div data-rs-showcase-preview-row="">
                    <DropdownMenuIsland trigger_label="Actions ▼">
                        <DropdownMenuItemIsland>"New"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland>"Open"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland>"Save"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland disabled=DisabledState::Disabled>"Cut"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland>"Copy"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland>"Paste"</DropdownMenuItemIsland>
                    </DropdownMenuIsland>
                </div>
            </div>
        </div>
    }
}
