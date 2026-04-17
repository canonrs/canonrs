use leptos::prelude::*;
use super::copy_button_boundary::CopyButton;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CopyButtonShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <CopyButton text="cargo add canonrs" aria_label="Copy to clipboard" />
            <p data-rs-showcase-preview-anchor="">
                "Click to copy — state machine: idle → copied → idle"
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With target selector"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <code id="snippet-1">"npm install canonrs"</code>
                    <CopyButton target="snippet-1" aria_label="Copy npm command" />
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Error state (no text — click to trigger)"</span>
                <CopyButton aria_label="Copy empty — triggers error" />
            </Stack>
        </Stack>
    }
}
