use leptos::prelude::*;
use super::menu_island::{MenuIsland, MenuIslandItem};

fn main_items() -> Vec<MenuIslandItem> {
    vec![
        MenuIslandItem { label: "New file".into(),         value: "new".into(),      disabled: false },
        MenuIslandItem { label: "Open file".into(),        value: "open".into(),     disabled: false },
        MenuIslandItem { label: "Save".into(),             value: "save".into(),     disabled: false },
        MenuIslandItem { label: "Export (disabled)".into(), value: "export".into(), disabled: true  },
    ]
}

fn edit_items() -> Vec<MenuIslandItem> {
    vec![
        MenuIslandItem { label: "Cut".into(),   value: "cut".into(),   disabled: false },
        MenuIslandItem { label: "Copy".into(),  value: "copy".into(),  disabled: false },
        MenuIslandItem { label: "Paste".into(), value: "paste".into(), disabled: false },
        MenuIslandItem { label: "Undo".into(),  value: "undo".into(),  disabled: false },
        MenuIslandItem { label: "Redo".into(),  value: "redo".into(),  disabled: false },
    ]
}

#[component]
pub fn MenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <MenuIsland items=main_items() selected="save" aria_label="Main menu" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction fully governed via signal — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Edit menu"</span>
                <div data-rs-showcase-preview-row="">
                    <MenuIsland items=edit_items() aria_label="Edit menu" />
                </div>
            </div>
        </div>
    }
}
