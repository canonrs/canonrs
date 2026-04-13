use leptos::prelude::*;
use super::radio_boundary::{RadioGroup, RadioGroupItem};
use canonrs_core::meta::{SelectionState, DisabledState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn RadioShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Selection state mapped directly to DOM and ARIA."
            </p>
            <RadioGroup>
                <RadioGroupItem value="leptos" name="framework">"Leptos"</RadioGroupItem>
                <RadioGroupItem value="dioxus" name="framework" selected=SelectionState::Selected>"Dioxus"</RadioGroupItem>
                <RadioGroupItem value="yew" name="framework">"Yew"</RadioGroupItem>
            </RadioGroup>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <RadioGroup disabled=DisabledState::Disabled>
                    <RadioGroupItem value="a" name="disabled">"Option A"</RadioGroupItem>
                    <RadioGroupItem value="b" name="disabled">"Option B"</RadioGroupItem>
                </RadioGroup>
            </Stack>
        </Stack>
    }
}
