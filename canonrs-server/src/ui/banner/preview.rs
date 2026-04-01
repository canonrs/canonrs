use leptos::prelude::*;
use super::banner_ui::{Banner, BannerContent, BannerActions, BannerClose};
use canonrs_core::primitives::BannerVariant;

#[component]
pub fn BannerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Banner variant=BannerVariant::Info>
                    <BannerContent>"System maintenance scheduled for Saturday at 2am UTC."</BannerContent>
                    <BannerClose>"×"</BannerClose>
                </Banner>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Visibility and ARIA behavior enforced by state and variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Banner variant=BannerVariant::Success>
                        <BannerContent>"Your account has been verified successfully."</BannerContent>
                        <BannerClose>"×"</BannerClose>
                    </Banner>
                    <Banner variant=BannerVariant::Warning>
                        <BannerContent>"Your subscription expires in 3 days."</BannerContent>
                        <BannerActions>
                            <a href="#">"Renew now"</a>
                        </BannerActions>
                        <BannerClose>"×"</BannerClose>
                    </Banner>
                    <Banner variant=BannerVariant::Error>
                        <BannerContent>"Payment failed. Please update your billing details."</BannerContent>
                        <BannerActions>
                            <a href="#">"Update billing"</a>
                        </BannerActions>
                        <BannerClose>"×"</BannerClose>
                    </Banner>
                </div>
            </div>
        </div>
    }
}
