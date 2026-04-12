use leptos::prelude::*;
use super::dropdown_menu_boundary::{
    DropdownMenu, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuCheckboxItem,
};
use canonrs_core::meta::{DisabledState, ToggleState};

#[component]
pub fn DropdownMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DropdownMenu trigger_label="Options">
                    <DropdownMenuItem>"Edit"</DropdownMenuItem>
                    <DropdownMenuItem>"Duplicate"</DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem disabled=DisabledState::Disabled>"Delete"</DropdownMenuItem>
                </DropdownMenu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Dropdown menu with keyboard navigation and disabled state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With checkboxes"</span>
                <div data-rs-showcase-preview-row="">
                    <DropdownMenu trigger_label="View">
                        <DropdownMenuCheckboxItem checked=ToggleState::On>"Show toolbar"</DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem>"Show sidebar"</DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem checked=ToggleState::On>"Show status bar"</DropdownMenuCheckboxItem>
                    </DropdownMenu>
                </div>
            </div>
        </div>
    }
}
