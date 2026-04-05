use leptos::prelude::*;
use super::dropdown_menu_island::{
    DropdownMenuIsland, DropdownMenuTriggerIsland,
    DropdownMenuContentIsland, DropdownMenuItemIsland,
};

#[component]
pub fn DropdownMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DropdownMenuIsland>
                    <DropdownMenuTriggerIsland>"Options ▼"</DropdownMenuTriggerIsland>
                    <DropdownMenuContentIsland>
                        <DropdownMenuItemIsland>"Profile"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland>"Settings"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland>"Billing"</DropdownMenuItemIsland>
                        <DropdownMenuItemIsland>"Sign out"</DropdownMenuItemIsland>
                    </DropdownMenuContentIsland>
                </DropdownMenuIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction and state governed by signal — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With groups"</span>
                <div data-rs-showcase-preview-row="">
                    <DropdownMenuIsland>
                        <DropdownMenuTriggerIsland>"Actions ▼"</DropdownMenuTriggerIsland>
                        <DropdownMenuContentIsland>
                            <DropdownMenuItemIsland>"New"</DropdownMenuItemIsland>
                            <DropdownMenuItemIsland>"Open"</DropdownMenuItemIsland>
                            <DropdownMenuItemIsland>"Save"</DropdownMenuItemIsland>
                            <DropdownMenuItemIsland>"Cut"</DropdownMenuItemIsland>
                            <DropdownMenuItemIsland>"Copy"</DropdownMenuItemIsland>
                            <DropdownMenuItemIsland>"Paste"</DropdownMenuItemIsland>
                        </DropdownMenuContentIsland>
                    </DropdownMenuIsland>
                </div>
            </div>
        </div>
    }
}
