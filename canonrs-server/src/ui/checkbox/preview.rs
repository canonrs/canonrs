use leptos::prelude::*;
use super::checkbox_boundary::Checkbox;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CheckboxShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Checkbox checked=true>
                <span>"Remember me"</span>
            </Checkbox>
            <p data-rs-showcase-preview-anchor="">
                "Checked state mapped explicitly to activity state."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Checkbox><span>"Unchecked"</span></Checkbox>
                    <Checkbox checked=true><span>"Checked"</span></Checkbox>
                    <Checkbox disabled=true><span>"Disabled"</span></Checkbox>
                    <Checkbox checked=true disabled=true><span>"Checked + Disabled"</span></Checkbox>
                </Stack>
            </Stack>
        </Stack>
    }
}
