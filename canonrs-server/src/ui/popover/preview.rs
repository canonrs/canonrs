use leptos::prelude::*;
use super::popover_island::PopoverIsland;

#[component]
pub fn PopoverShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <PopoverIsland
                    trigger_label="Open Popover"
                    content="Configure your preferences here."
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger and content visibility governed by shared state contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With filter"</span>
                <div data-rs-showcase-preview-row="">
                    <PopoverIsland
                        trigger_label="Filter"
                        content="Apply filters to narrow your search."
                        side="bottom"
                    />
                </div>
            </div>
        </div>
    }
}
