use leptos::prelude::*;
use super::banner_boundary::Banner;
use canonrs_core::primitives::BannerVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn BannerShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Banner content="System maintenance scheduled for Saturday at 2am UTC." dismissible=true />
            <p data-rs-showcase-preview-anchor="">
                "Visibility and ARIA behavior enforced by state and variant."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <Banner variant=BannerVariant::Success content="Your account has been verified." dismissible=true />
                    <Banner variant=BannerVariant::Warning content="Your subscription expires in 3 days." dismissible=true />
                    <Banner variant=BannerVariant::Error content="Payment failed. Please update billing." dismissible=true />
                </Stack>
            </Stack>
        </Stack>
    }
}
