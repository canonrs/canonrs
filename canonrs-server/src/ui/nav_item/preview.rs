use leptos::prelude::*;
use super::nav_item_boundary::{NavItem, NavGroup};
use canonrs_core::meta::{ActivityState, DisabledState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn NavItemShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Active and disabled navigation states enforced structurally."
            </p>

            // Vertical (sidebar)
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Vertical (sidebar)"</span>
                <NavGroup>
                    <NavItem label="Dashboard" href="#" active=ActivityState::Active />
                    <NavItem label="Components" href="#" />
                    <NavItem label="Tokens" href="#" />
                    <NavItem label="Settings" href="#" />
                </NavGroup>
            </Stack>

            // Horizontal (inline nav)
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Horizontal (inline)"</span>
                <NavGroup direction="horizontal">
                    <NavItem label="Home" href="#" active=ActivityState::Active />
                    <NavItem label="About" href="#" />
                    <NavItem label="Contact" href="#" />
                </NavGroup>
            </Stack>

            // States
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States"</span>
                <NavGroup>
                    <NavItem label="Active" href="#" active=ActivityState::Active />
                    <NavItem label="Inactive" href="#" />
                    <NavItem label="Disabled" href="#" disabled=DisabledState::Disabled />
                </NavGroup>
            </Stack>
        </Stack>
    }
}
