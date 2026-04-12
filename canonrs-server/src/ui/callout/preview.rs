use leptos::prelude::*;
use super::boundary::Callout;
use canonrs_core::primitives::CalloutVariant;

#[component]
pub fn CalloutShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Callout variant=CalloutVariant::Info title="Information" description="New features are available in the latest release." />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Semantic role and urgency enforced via variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Callout variant=CalloutVariant::Success     icon="✓" title="Success"     description="Your changes have been deployed." />
                    <Callout variant=CalloutVariant::Warning     icon="⚠" title="Warning"     description="This action cannot be undone." />
                    <Callout variant=CalloutVariant::Warning icon="✕" title="Error"       description="Build failed due to type errors." />
                </div>
            </div>
        </div>
    }
}
