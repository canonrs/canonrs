use leptos::prelude::*;
use super::spinner_boundary::Spinner;
use canonrs_core::primitives::SpinnerSize;

#[component]
pub fn SpinnerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Spinner />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Loading state enforced by primitive."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Spinner size=SpinnerSize::Small />
                    <Spinner size=SpinnerSize::Medium />
                    <Spinner size=SpinnerSize::Large />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Paused"</span>
                <div data-rs-showcase-preview-row="">
                    <Spinner paused=true />
                </div>
            </div>
        </div>
    }
}
