use leptos::prelude::*;
use super::copy_button_boundary::CopyButton;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CopyButtonShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <CopyButton text="cargo add canonrs" aria_label="Copy to clipboard" />
            <p data-rs-showcase-preview-anchor="">
                "Copy state lifecycle fully encoded in DOM state machine."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States (idle → copied → reset)"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <CopyButton text="npm install react" aria_label="Copy npm" />
                    <CopyButton text="cargo add leptos"  aria_label="Copy cargo" />
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Error state (no text)"</span>
                <CopyButton aria_label="Copy empty" />
            </Stack>
        </Stack>
    }
}
