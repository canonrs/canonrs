use leptos::prelude::*;
use super::popover_island::{PopoverIsland, PopoverTriggerIsland, PopoverContentIsland};
use canonrs_core::primitives::PopoverSide;

#[component]
pub fn PopoverShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <PopoverIsland>
                    <PopoverTriggerIsland>"Open Popover"</PopoverTriggerIsland>
                    <PopoverContentIsland>
                        <p>"This is a popover. Click outside to close."</p>
                    </PopoverContentIsland>
                </PopoverIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Popover with keyboard and click-outside dismiss."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="">
                    <PopoverIsland>
                        <PopoverTriggerIsland>"Bottom"</PopoverTriggerIsland>
                        <PopoverContentIsland side=PopoverSide::Bottom>"Opens below"</PopoverContentIsland>
                    </PopoverIsland>
                    <PopoverIsland>
                        <PopoverTriggerIsland>"Top"</PopoverTriggerIsland>
                        <PopoverContentIsland side=PopoverSide::Top>"Opens above"</PopoverContentIsland>
                    </PopoverIsland>
                    <PopoverIsland>
                        <PopoverTriggerIsland>"Right"</PopoverTriggerIsland>
                        <PopoverContentIsland side=PopoverSide::Right>"Opens right"</PopoverContentIsland>
                    </PopoverIsland>
                </div>
            </div>
        </div>
    }
}
