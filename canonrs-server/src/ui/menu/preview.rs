use leptos::prelude::*;
use super::menu_boundary::{Menu, MenuItem};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn MenuShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction fully governed via DOM — SSR-safe, hydration-safe."
            </p>
            <Menu aria_label="Main menu">
                <MenuItem value="new">"New file"</MenuItem>
                <MenuItem value="open">"Open file"</MenuItem>
                <MenuItem value="save" selected=true>"Save"</MenuItem>
                <MenuItem value="export" disabled=true>"Export (disabled)"</MenuItem>
            </Menu>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Edit menu"</span>
                <Menu aria_label="Edit menu">
                    <MenuItem value="cut">"Cut"</MenuItem>
                    <MenuItem value="copy">"Copy"</MenuItem>
                    <MenuItem value="paste">"Paste"</MenuItem>
                    <MenuItem value="undo">"Undo"</MenuItem>
                    <MenuItem value="redo">"Redo"</MenuItem>
                </Menu>
            </Stack>
        </Stack>
    }
}
