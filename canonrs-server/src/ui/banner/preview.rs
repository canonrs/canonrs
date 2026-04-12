use leptos::prelude::*;
use super::banner_boundary::Banner;
use canonrs_core::primitives::BannerVariant;

#[component]
pub fn BannerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Banner content="System maintenance scheduled for Saturday at 2am UTC." />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Visibility and ARIA behavior enforced by state and variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Banner variant=BannerVariant::Success content="Your account has been verified." />
                    <Banner variant=BannerVariant::Warning content="Your subscription expires in 3 days." />
                    <Banner variant=BannerVariant::Error   content="Payment failed. Please update billing." />
                </div>
            </div>
        </div>
    }
}
