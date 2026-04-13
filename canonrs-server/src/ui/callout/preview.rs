use leptos::prelude::*;
use super::callout_boundary::Callout;
use canonrs_core::primitives::CalloutVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CalloutShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Callout variant=CalloutVariant::Info title="Information" description="New features are available in the latest release." />
            <p data-rs-showcase-preview-anchor="">"Semantic role and urgency enforced via variant."</p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <Callout variant=CalloutVariant::Success icon="✓" title="Success"  description="Your changes have been deployed." />
                    <Callout variant=CalloutVariant::Warning icon="⚠" title="Warning"  description="This action cannot be undone." />
                    <Callout variant=CalloutVariant::Warning icon="✕" title="Error"    description="Build failed due to type errors." />
                </Stack>
            </Stack>
        </Stack>
    }
}
