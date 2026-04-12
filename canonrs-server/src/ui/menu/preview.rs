use leptos::prelude::*;
use super::menu_boundary::{Menu, MenuItem};

#[component]
pub fn MenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Menu aria_label="Main menu">
                    <MenuItem value="new">"New file"</MenuItem>
                    <MenuItem value="open">"Open file"</MenuItem>
                    <MenuItem value="save" selected=true>"Save"</MenuItem>
                    <MenuItem value="export" disabled=true>"Export (disabled)"</MenuItem>
                </Menu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction fully governed via DOM — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Edit menu"</span>
                <div data-rs-showcase-preview-row="">
                    <Menu aria_label="Edit menu">
                        <MenuItem value="cut">"Cut"</MenuItem>
                        <MenuItem value="copy">"Copy"</MenuItem>
                        <MenuItem value="paste">"Paste"</MenuItem>
                        <MenuItem value="undo">"Undo"</MenuItem>
                        <MenuItem value="redo">"Redo"</MenuItem>
                    </Menu>
                </div>
            </div>
        </div>
    }
}
