use leptos::prelude::*;
use super::boundary::{Popover, PopoverTrigger, PopoverContent};
use canonrs_core::primitives::PopoverSide;

#[component]
pub fn PopoverShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Popover>
                    <PopoverTrigger>"Open Popover"</PopoverTrigger>
                    <PopoverContent>
                        <p>"This is a popover. Click outside to close."</p>
                    </PopoverContent>
                </Popover>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Popover with keyboard and click-outside dismiss."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="">
                    <Popover>
                        <PopoverTrigger>"Bottom"</PopoverTrigger>
                        <PopoverContent side=PopoverSide::Bottom>"Opens below"</PopoverContent>
                    </Popover>
                    <Popover>
                        <PopoverTrigger>"Top"</PopoverTrigger>
                        <PopoverContent side=PopoverSide::Top>"Opens above"</PopoverContent>
                    </Popover>
                    <Popover>
                        <PopoverTrigger>"Right"</PopoverTrigger>
                        <PopoverContent side=PopoverSide::Right>"Opens right"</PopoverContent>
                    </Popover>
                </div>
            </div>
        </div>
    }
}
