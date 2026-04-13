use leptos::prelude::*;
use super::kbd_boundary::{Kbd, KbdGroup, KbdSeparator, KbdSize, KbdVariant};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn KbdShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <KbdGroup>
                <Kbd>"Ctrl"</Kbd>
                <KbdSeparator />
                <Kbd>"K"</Kbd>
            </KbdGroup>
            <p data-rs-showcase-preview-anchor="">
                "Shortcut representation standardized via size and variant enums."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Size variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Kbd size=KbdSize::Sm>"Sm"</Kbd>
                    <Kbd size=KbdSize::Md>"Md"</Kbd>
                    <Kbd size=KbdSize::Lg>"Lg"</Kbd>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variant"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Kbd>"Default"</Kbd>
                    <Kbd variant=KbdVariant::Outline>"Outline"</Kbd>
                    <Kbd variant=KbdVariant::Ghost>"Ghost"</Kbd>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Common shortcuts"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd><KbdSeparator /><Kbd>"C"</Kbd>
                    </KbdGroup>
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd><KbdSeparator /><Kbd>"Shift"</Kbd><KbdSeparator /><Kbd>"P"</Kbd>
                    </KbdGroup>
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd><KbdSeparator /><Kbd>"Z"</Kbd>
                    </KbdGroup>
                </Stack>
            </Stack>
        </Stack>
    }
}
