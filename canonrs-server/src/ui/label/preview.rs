use leptos::prelude::*;
use super::label_boundary::Label;
use crate::ui::input::input_boundary::Input;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn LabelShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                <Label for_id="label-input">"Username"</Label>
                <Input placeholder="johndoe" />
            </Stack>
            <p data-rs-showcase-preview-anchor="">
                "Label-to-input association enforced via explicit html_for contract."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Multiple labels"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <Label for_id="label-email"><span>"Email"</span></Label>
                        <Input placeholder="john@example.com" input_type="email" />
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <Label for_id="label-password"><span>"Password"</span></Label>
                        <Input placeholder="••••••••" input_type="password" />
                    </Stack>
                </Stack>
            </Stack>
        </Stack>
    }
}
