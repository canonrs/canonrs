use leptos::prelude::*;
use super::dropdown_menu_island::{DropdownMenuIsland, DropdownMenuIslandItem};

fn main_items() -> Vec<DropdownMenuIslandItem> {
    vec![
        DropdownMenuIslandItem { label: "Profile".into(),  value: "profile".into(),  disabled: false },
        DropdownMenuIslandItem { label: "Settings".into(), value: "settings".into(), disabled: false },
        DropdownMenuIslandItem { label: "Billing".into(),  value: "billing".into(),  disabled: false },
        DropdownMenuIslandItem { label: "Sign out".into(), value: "signout".into(),  disabled: false },
    ]
}

fn action_items() -> Vec<DropdownMenuIslandItem> {
    vec![
        DropdownMenuIslandItem { label: "New".into(),   value: "new".into(),   disabled: false },
        DropdownMenuIslandItem { label: "Open".into(),  value: "open".into(),  disabled: false },
        DropdownMenuIslandItem { label: "Save".into(),  value: "save".into(),  disabled: false },
        DropdownMenuIslandItem { label: "Cut".into(),   value: "cut".into(),   disabled: false },
        DropdownMenuIslandItem { label: "Copy".into(),  value: "copy".into(),  disabled: false },
        DropdownMenuIslandItem { label: "Paste".into(), value: "paste".into(), disabled: false },
    ]
}

#[component]
pub fn DropdownMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DropdownMenuIsland items=main_items() trigger_label="Options ▼" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction governed by DOM — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With actions"</span>
                <div data-rs-showcase-preview-row="">
                    <DropdownMenuIsland items=action_items() trigger_label="Actions ▼" />
                </div>
            </div>
        </div>
    }
}
