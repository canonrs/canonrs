use leptos::prelude::*;
use super::badge_boundary::Badge;
use canonrs_core::primitives::{BadgeVariant, BadgeInteractivity};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn BadgeShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Badge variant=BadgeVariant::Success>"Active"</Badge>
            <p data-rs-showcase-preview-anchor="">
                "Interactivity explicitly defined and enforced by type."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Badge>"Default"</Badge>
                    <Badge variant=BadgeVariant::Primary>"Primary"</Badge>
                    <Badge variant=BadgeVariant::Success>"Success"</Badge>
                    <Badge variant=BadgeVariant::Warning>"Warning"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Interactivity"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Badge>"Static"</Badge>
                    <Badge interactivity=BadgeInteractivity::Interactive>"Interactive"</Badge>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Count / status examples"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Badge variant=BadgeVariant::Primary>"12"</Badge>
                    <Badge variant=BadgeVariant::Warning>"Pending"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Failed"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Draft"</Badge>
                    <Badge variant=BadgeVariant::Success>"Published"</Badge>
                </Stack>
            </Stack>
        </Stack>
    }
}
