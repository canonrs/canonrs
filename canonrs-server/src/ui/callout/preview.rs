use leptos::prelude::*;
use super::callout_ui::{Callout, CalloutIcon, CalloutTitle, CalloutDescription};
use canonrs_core::primitives::CalloutVariant;

#[component]
pub fn CalloutShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Callout variant=CalloutVariant::Info>
                    <CalloutTitle>"Information"</CalloutTitle>
                    <CalloutDescription>"New features are available in the latest release of CanonRS."</CalloutDescription>
                </Callout>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Semantic role and urgency enforced via variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Callout variant=CalloutVariant::Success>
                        <CalloutIcon>"✓"</CalloutIcon>
                        <CalloutTitle>"Success"</CalloutTitle>
                        <CalloutDescription>"Your changes have been deployed to production."</CalloutDescription>
                    </Callout>
                    <Callout variant=CalloutVariant::Warning>
                        <CalloutIcon>"⚠"</CalloutIcon>
                        <CalloutTitle>"Warning"</CalloutTitle>
                        <CalloutDescription>"This action cannot be undone. Proceed with caution."</CalloutDescription>
                    </Callout>
                    <Callout variant=CalloutVariant::Error>
                        <CalloutIcon>"✕"</CalloutIcon>
                        <CalloutTitle>"Error"</CalloutTitle>
                        <CalloutDescription>"Build failed due to type errors. Check the logs."</CalloutDescription>
                    </Callout>
                </div>
            </div>
        </div>
    }
}
