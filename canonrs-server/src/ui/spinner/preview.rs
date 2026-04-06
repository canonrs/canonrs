use leptos::prelude::*;
use super::spinner_island::SpinnerIsland;

#[component]
pub fn SpinnerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <SpinnerIsland />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "State and size strictly controlled via enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <SpinnerIsland size="small" />
                    <SpinnerIsland size="medium" />
                    <SpinnerIsland size="large" />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <SpinnerIsland paused=false />
                    <SpinnerIsland paused=true />
                </div>
            </div>
        </div>
    }
}
