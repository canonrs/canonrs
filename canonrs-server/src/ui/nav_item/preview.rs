use leptos::prelude::*;
use super::nav_item_boundary::NavItem;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn NavItemShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Active and disabled navigation states enforced structurally."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                <NavItem label="Dashboard" href="#" active=true.into() />
                <NavItem label="Components" href="#" />
                <NavItem label="Tokens" href="#" />
                <NavItem label="Settings" href="#" disabled=true.into() />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Inactive"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <NavItem label="Home" href="#" />
                    <NavItem label="About" href="#" />
                    <NavItem label="Contact" href="#" />
                </Stack>
            </Stack>
        </Stack>
    }
}
