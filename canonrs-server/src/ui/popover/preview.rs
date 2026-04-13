use leptos::prelude::*;
use super::popover_boundary::{Popover, PopoverTrigger, PopoverContent};
use canonrs_core::primitives::PopoverSide;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn PopoverShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Popover with keyboard and click-outside dismiss."
            </p>
            <Popover>
                <PopoverTrigger>"Open Popover"</PopoverTrigger>
                <PopoverContent>
                    <p>"This is a popover. Click outside to close."</p>
                </PopoverContent>
            </Popover>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
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
                </Stack>
            </Stack>
        </Stack>
    }
}
