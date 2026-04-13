use leptos::prelude::*;
use super::radio_group_boundary::{RadioGroup, RadioGroupItem};
use canonrs_core::meta::{SelectionState, DisabledState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn RadioGroupShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Selection state mapped directly to DOM and ARIA."
            </p>
            <RadioGroup>
                <RadioGroupItem value="leptos" name="framework">"Leptos"</RadioGroupItem>
                <RadioGroupItem value="dioxus" name="framework">"Dioxus"</RadioGroupItem>
                <RadioGroupItem value="yew" name="framework">"Yew"</RadioGroupItem>
            </RadioGroup>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <RadioGroup>
                    <RadioGroupItem value="sm" name="size">"Small"</RadioGroupItem>
                    <RadioGroupItem value="md" name="size" selected=SelectionState::Selected>"Medium"</RadioGroupItem>
                    <RadioGroupItem value="lg" name="size">"Large"</RadioGroupItem>
                </RadioGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <RadioGroup disabled=DisabledState::Disabled>
                    <RadioGroupItem value="a" name="dis">"Option A"</RadioGroupItem>
                    <RadioGroupItem value="b" name="dis">"Option B"</RadioGroupItem>
                </RadioGroup>
            </Stack>
        </Stack>
    }
}
