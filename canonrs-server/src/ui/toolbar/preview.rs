use leptos::prelude::*;
use super::toolbar_island::{ToolbarIsland, ToolbarItem};

#[component]
pub fn ToolbarShowcasePreview() -> impl IntoView {
    let formatting_items = vec![
        ToolbarItem { label: "B".into(),  value: "bold".into(),      disabled: false, separator_after: false },
        ToolbarItem { label: "I".into(),  value: "italic".into(),    disabled: false, separator_after: false },
        ToolbarItem { label: "U".into(),  value: "underline".into(), disabled: false, separator_after: true  },
        ToolbarItem { label: "←".into(), value: "left".into(),      disabled: false, separator_after: false },
        ToolbarItem { label: "↔".into(), value: "center".into(),    disabled: false, separator_after: false },
        ToolbarItem { label: "→".into(), value: "right".into(),     disabled: false, separator_after: true  },
        ToolbarItem { label: "🔗".into(), value: "link".into(),     disabled: false, separator_after: false },
    ];
    let vertical_items = vec![
        ToolbarItem { label: "✂".into(),  value: "cut".into(),    disabled: false, separator_after: false },
        ToolbarItem { label: "⎘".into(),  value: "copy".into(),   disabled: false, separator_after: false },
        ToolbarItem { label: "📋".into(), value: "paste".into(),  disabled: false, separator_after: true  },
        ToolbarItem { label: "🗑".into(), value: "delete".into(), disabled: false, separator_after: false },
    ];
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <ToolbarIsland items=formatting_items aria_label="Text formatting" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Toolbar role and orientation enforced via contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Vertical"</span>
                <div data-rs-showcase-preview-row="">
                    <ToolbarIsland items=vertical_items aria_label="Actions" orientation="vertical" />
                </div>
            </div>
        </div>
    }
}
