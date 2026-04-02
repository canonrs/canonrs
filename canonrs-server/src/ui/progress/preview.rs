use leptos::prelude::*;
use super::progress_ui::Progress;

#[component]
pub fn ProgressShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);width:100%;">
                    <Progress value=43.0 />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Value always clamped between 0–100 and ARIA-compliant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);width:100%;">
                    <Progress value=0.0 />
                    <Progress value=25.0 />
                    <Progress value=50.0 />
                    <Progress value=75.0 />
                    <Progress value=100.0 />
                </div>
            </div>
        </div>
    }
}
