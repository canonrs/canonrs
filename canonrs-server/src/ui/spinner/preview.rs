use leptos::prelude::*;
use super::spinner_island::SpinnerIsland;
use canonrs_core::primitives::SpinnerSize;

#[component]
pub fn SpinnerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SpinnerIsland />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Loading state enforced by primitive."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <SpinnerIsland size=SpinnerSize::Small />
                    <SpinnerIsland size=SpinnerSize::Medium />
                    <SpinnerIsland size=SpinnerSize::Large />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Paused"</span>
                <div data-rs-showcase-preview-row="">
                    <SpinnerIsland paused=true />
                </div>
            </div>
        </div>
    }
}
