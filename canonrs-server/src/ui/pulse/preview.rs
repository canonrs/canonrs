use leptos::prelude::*;
use super::pulse_boundary::{Pulse, PulseVariant, PulseSize, PulseSpeed};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn PulseShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Animation variant, size and speed strictly typed."
            </p>
            <Pulse />
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Pulse variant=PulseVariant::Subtle />
                    <Pulse />
                    <Pulse variant=PulseVariant::Emphasized />
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Pulse size=PulseSize::Small />
                    <Pulse size=PulseSize::Medium />
                    <Pulse size=PulseSize::Large />
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Speed"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Pulse speed=PulseSpeed::Slow />
                    <Pulse speed=PulseSpeed::Normal />
                    <Pulse speed=PulseSpeed::Fast />
                </Stack>
            </Stack>
        </Stack>
    }
}
