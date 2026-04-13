use leptos::prelude::*;
use super::drawer_boundary::Drawer;
use canonrs_core::primitives::DrawerSide;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn DrawerShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Drawer
                trigger_label="Open Drawer"
                title="Drawer Title"
                description="Slides in from the side. State governed via DOM."
                close_label="Close"
            />
            <p data-rs-showcase-preview-anchor="">
                "Drawer visibility and overlay fully governed via shared state."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Left side"</span>
                <Drawer
                    trigger_label="Open left"
                    title="Left Drawer"
                    close_label="Close"
                    side=DrawerSide::Left
                />
            </Stack>
        </Stack>
    }
}
