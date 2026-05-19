use leptos::prelude::*;
use super::separator_boundary::Separator;
use canonrs_core::Orientation;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn SeparatorShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Separator semantics enforced via orientation and role contract."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Md>
                <span>"Above"</span>
                <Separator />
                <span>"Below"</span>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Vertical"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <span>"Left"</span>
                    <Separator orientation=Orientation::Vertical />
                    <span>"Center"</span>
                    <Separator orientation=Orientation::Vertical />
                    <span>"Right"</span>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Semantic"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Md>
                    <span>"Section A"</span>
                    <Separator decorative=false aria_label="Between sections" />
                    <span>"Section B"</span>
                </Stack>
            </Stack>
        </Stack>
    }
}
