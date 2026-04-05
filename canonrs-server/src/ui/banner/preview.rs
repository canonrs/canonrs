use leptos::prelude::*;
use super::banner_island::{BannerIsland, BannerIslandVariant};

#[component]
pub fn BannerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <BannerIsland content="System maintenance scheduled for Saturday at 2am UTC." />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Visibility and ARIA behavior enforced by state and variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <BannerIsland variant=BannerIslandVariant::Success content="Your account has been verified." />
                    <BannerIsland variant=BannerIslandVariant::Warning content="Your subscription expires in 3 days." />
                    <BannerIsland variant=BannerIslandVariant::Error   content="Payment failed. Please update billing." />
                </div>
            </div>
        </div>
    }
}
