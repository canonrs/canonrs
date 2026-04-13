use leptos::prelude::*;
use super::alert_boundary::Alert;
use canonrs_core::primitives::AlertVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn AlertShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Alert title="Info" description="This is a default informational alert." />
            <p data-rs-showcase-preview-anchor="">
                "Correct ARIA role and live region enforced by variant."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Alert variant=AlertVariant::Success     title="Success"     description="Your changes have been saved." />
                <Alert variant=AlertVariant::Warning     title="Warning"     description="Session expires in 5 minutes." />
                <Alert variant=AlertVariant::Destructive title="Error"       description="Failed to save changes." />
                <Alert variant=AlertVariant::Default     title="Info"        description="A new version is available." />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Dismissible"</span>
                <Alert title="Update available" description="New version ready." dismissible=true />
            </Stack>
        </Stack>
    }
}
