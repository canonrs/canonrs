use leptos::prelude::*;
use super::context_menu_island::{ContextMenuIsland, ContextMenuIslandItem};

fn main_items() -> Vec<ContextMenuIslandItem> {
    vec![
        ContextMenuIslandItem { label: "View".into(),      value: "view".into(),      disabled: false },
        ContextMenuIslandItem { label: "Edit".into(),      value: "edit".into(),      disabled: false },
        ContextMenuIslandItem { label: "Duplicate".into(), value: "duplicate".into(), disabled: false },
        ContextMenuIslandItem { label: "Delete".into(),    value: "delete".into(),    disabled: false },
    ]
}

fn file_items() -> Vec<ContextMenuIslandItem> {
    vec![
        ContextMenuIslandItem { label: "Open".into(),          value: "open".into(),          disabled: false },
        ContextMenuIslandItem { label: "Open with...".into(),  value: "open-with".into(),     disabled: false },
        ContextMenuIslandItem { label: "Cut".into(),           value: "cut".into(),           disabled: false },
        ContextMenuIslandItem { label: "Copy".into(),          value: "copy".into(),          disabled: false },
        ContextMenuIslandItem { label: "Paste".into(),         value: "paste".into(),         disabled: false },
        ContextMenuIslandItem { label: "Rename".into(),        value: "rename".into(),        disabled: false },
        ContextMenuIslandItem { label: "Move to trash".into(), value: "trash".into(),         disabled: false },
    ]
}

#[component]
pub fn ContextMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ContextMenuIsland items=main_items()>
                    <div style="padding:2rem;border:1px dashed currentColor;border-radius:var(--radius-md);text-align:center;cursor:context-menu;">
                        "Right-click here"
                    </div>
                </ContextMenuIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Context menu interaction and roles governed by signal — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"File actions"</span>
                <div data-rs-showcase-preview-row="">
                    <ContextMenuIsland items=file_items()>
                        <div style="padding:2rem;border:1px dashed currentColor;border-radius:var(--radius-md);text-align:center;cursor:context-menu;">
                            "Right-click for file actions"
                        </div>
                    </ContextMenuIsland>
                </div>
            </div>
        </div>
    }
}
