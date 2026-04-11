use leptos::prelude::*;
use super::dropdown_menu_island::{
    DropdownMenuIsland, DropdownMenuItemIsland, DropdownMenuSeparatorIsland,
    DropdownMenuCheckboxItemIsland,
};
use canonrs_core::meta::{DisabledState, ToggleState};

#[component]
pub fn DropdownMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DropdownMenuIsland trigger_label="Options">
                    <DropdownMenuItemIsland>"Edit"</DropdownMenuItemIsland>
                    <DropdownMenuItemIsland>"Duplicate"</DropdownMenuItemIsland>
                    <DropdownMenuSeparatorIsland />
                    <DropdownMenuItemIsland disabled=DisabledState::Disabled>"Delete"</DropdownMenuItemIsland>
                </DropdownMenuIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Dropdown menu with keyboard navigation and disabled state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With checkboxes"</span>
                <div data-rs-showcase-preview-row="">
                    <DropdownMenuIsland trigger_label="View">
                        <DropdownMenuCheckboxItemIsland checked=ToggleState::On>"Show toolbar"</DropdownMenuCheckboxItemIsland>
                        <DropdownMenuCheckboxItemIsland>"Show sidebar"</DropdownMenuCheckboxItemIsland>
                        <DropdownMenuCheckboxItemIsland checked=ToggleState::On>"Show status bar"</DropdownMenuCheckboxItemIsland>
                    </DropdownMenuIsland>
                </div>
            </div>
        </div>
    }
}
