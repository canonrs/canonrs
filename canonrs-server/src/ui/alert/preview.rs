use leptos::prelude::*;
use super::alert_ui::{Alert, AlertTitle, AlertDescription, AlertCloseButton};
use canonrs_core::primitives::AlertVariant;

#[component]
pub fn AlertShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Alert variant=AlertVariant::Default>
                    <AlertTitle>"Info"</AlertTitle>
                    <AlertDescription>"This is a default informational alert message."</AlertDescription>
                </Alert>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Correct ARIA role and live region enforced by variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Alert variant=AlertVariant::Success>
                        <AlertTitle>"Success"</AlertTitle>
                        <AlertDescription>"Your changes have been saved."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Warning>
                        <AlertTitle>"Warning"</AlertTitle>
                        <AlertDescription>"Your session will expire in 5 minutes."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Default>
                        <AlertTitle>"Note"</AlertTitle>
                        <AlertDescription>"Failed to save changes. Please try again."</AlertDescription>
                    </Alert>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Dismissible"</span>
                <div data-rs-showcase-preview-row="">
                    <Alert variant=AlertVariant::Default>
                        <AlertTitle>"Update available"</AlertTitle>
                        <AlertDescription>"A new version of CanonRS is available."</AlertDescription>
                        <AlertCloseButton>"×"</AlertCloseButton>
                    </Alert>
                </div>
            </div>
        </div>
    }
}
