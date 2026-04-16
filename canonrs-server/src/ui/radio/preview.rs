use leptos::prelude::*;
use super::radio_boundary::Radio;
use canonrs_core::meta::{SelectionState, DisabledState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn RadioShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Selection state mapped directly to DOM and ARIA."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <Radio value="leptos" name="framework">"Leptos"</Radio>
                <Radio value="dioxus" name="framework" selected=SelectionState::Selected>"Dioxus"</Radio>
                <Radio value="yew" name="framework">"Yew"</Radio>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <Radio value="a" name="disabled-radio" disabled=DisabledState::Disabled>"Option A"</Radio>
                <Radio value="b" name="disabled-radio" disabled=DisabledState::Disabled>"Option B"</Radio>
            </Stack>
        </Stack>
    }
}
