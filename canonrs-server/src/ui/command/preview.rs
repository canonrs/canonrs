use leptos::prelude::*;
use super::command_island::{CommandIsland, CommandIslandItem};

fn all_items() -> Vec<CommandIslandItem> {
    vec![
        CommandIslandItem { label: "Calendar".into(), value: "calendar".into(), group: Some("Suggestions".into()) },
        CommandIslandItem { label: "Search".into(),   value: "search".into(),   group: Some("Suggestions".into()) },
        CommandIslandItem { label: "Settings".into(), value: "settings".into(), group: Some("Suggestions".into()) },
        CommandIslandItem { label: "New file".into(), value: "new".into(),      group: Some("Actions".into()) },
        CommandIslandItem { label: "Open file".into(), value: "open".into(),    group: Some("Actions".into()) },
        CommandIslandItem { label: "Save".into(),     value: "save".into(),     group: Some("Actions".into()) },
    ]
}

#[component]
pub fn CommandShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CommandIsland
                    items=all_items()
                    placeholder="Search commands..."
                    empty_text="No results found."
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Command palette — typeahead filter governed by signal."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Empty state"</span>
                <div data-rs-showcase-preview-row="">
                    <CommandIsland
                        items=vec![]
                        placeholder="Type to search..."
                        empty_text="No results found."
                    />
                </div>
            </div>
        </div>
    }
}
