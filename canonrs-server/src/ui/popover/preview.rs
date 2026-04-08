use leptos::prelude::*;
use super::popover_island::{PopoverIsland, PopoverContentIsland};

#[component]
pub fn PopoverShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <PopoverIsland>
                    <button type="button" data-rs-popover-trigger="">"Open Popover"</button>
                    <PopoverContentIsland>
                        "Configure your preferences here."
                    </PopoverContentIsland>
                </PopoverIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger and content visibility governed by shared state contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With filter"</span>
                <div data-rs-showcase-preview-row="">
                    <PopoverIsland>
                        <button type="button" data-rs-popover-trigger="">"Filter"</button>
                        <PopoverContentIsland>
                            "Apply filters to narrow your search."
                        </PopoverContentIsland>
                    </PopoverIsland>
                </div>
            </div>
        </div>
    }
}
