use leptos::prelude::*;
use super::dropdown_menu_boundary::{
    DropdownMenu, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuCheckboxItem,
};
use canonrs_core::meta::{DisabledState, ToggleState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn DropdownMenuShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <DropdownMenu trigger_label="Options">
                <DropdownMenuItem>"Edit"</DropdownMenuItem>
                <DropdownMenuItem>"Duplicate"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem disabled=DisabledState::Disabled>"Delete"</DropdownMenuItem>
            </DropdownMenu>
            <p data-rs-showcase-preview-anchor="">
                "Dropdown menu with keyboard navigation and disabled state."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With checkboxes"</span>
                <DropdownMenu trigger_label="View">
                    <DropdownMenuCheckboxItem checked=ToggleState::On>"Show toolbar"</DropdownMenuCheckboxItem>
                    <DropdownMenuCheckboxItem>"Show sidebar"</DropdownMenuCheckboxItem>
                    <DropdownMenuCheckboxItem checked=ToggleState::On>"Show status bar"</DropdownMenuCheckboxItem>
                </DropdownMenu>
            </Stack>
        </Stack>
    }
}
