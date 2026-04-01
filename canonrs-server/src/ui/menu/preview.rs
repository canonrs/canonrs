use leptos::prelude::*;
use super::menu_ui::{Menu, MenuItem, MenuGroup, MenuLabel, MenuSeparator};
use canonrs_core::meta::{DisabledState, SelectionState};

#[component]
pub fn MenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Menu aria_label="Main menu">
                    <MenuLabel>"Actions"</MenuLabel>
                    <MenuGroup>
                        <MenuItem>"New file"</MenuItem>
                        <MenuItem>"Open file"</MenuItem>
                        <MenuItem selected=SelectionState::Selected>"Save"</MenuItem>
                    </MenuGroup>
                    <MenuSeparator />
                    <MenuItem disabled=DisabledState::Disabled>"Export (disabled)"</MenuItem>
                </Menu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction fully governed via structured ARIA and state attributes."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With groups"</span>
                <div data-rs-showcase-preview-row="">
                    <Menu aria_label="Edit menu">
                        <MenuLabel>"Edit"</MenuLabel>
                        <MenuGroup>
                            <MenuItem>"Cut"</MenuItem>
                            <MenuItem>"Copy"</MenuItem>
                            <MenuItem>"Paste"</MenuItem>
                        </MenuGroup>
                        <MenuSeparator />
                        <MenuLabel>"History"</MenuLabel>
                        <MenuGroup>
                            <MenuItem>"Undo"</MenuItem>
                            <MenuItem>"Redo"</MenuItem>
                        </MenuGroup>
                    </Menu>
                </div>
            </div>
        </div>
    }
}
