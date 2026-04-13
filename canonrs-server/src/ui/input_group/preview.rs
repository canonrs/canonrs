use leptos::prelude::*;
use super::input_group_boundary::InputGroup;
use crate::ui::input::input_boundary::Input;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn InputGroupShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <InputGroup merge_radius=true>
                <span data-rs-input-group-addon="">"@"</span>
                <Input placeholder="username" name="username" />
            </InputGroup>
            <p data-rs-showcase-preview-anchor="">
                "Grouped inputs maintain consistent structure and visual merging."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <InputGroup>
                    <span data-rs-input-group-addon="">"@"</span>
                    <Input placeholder="username" name="username-detached" />
                </InputGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"URL input"</span>
                <InputGroup merge_radius=true>
                    <span data-rs-input-group-addon="">"https://"</span>
                    <Input placeholder="example.com" name="url" />
                </InputGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With suffix"</span>
                <InputGroup merge_radius=true>
                    <Input placeholder="0.00" name="amount" />
                    <span data-rs-input-group-addon="">"USD"</span>
                </InputGroup>
            </Stack>
        </Stack>
    }
}
