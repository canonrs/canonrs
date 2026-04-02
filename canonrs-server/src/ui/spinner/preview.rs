use leptos::prelude::*;
use super::spinner_ui::{Spinner, SpinnerSize};

#[component]
pub fn SpinnerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <Spinner size=SpinnerSize::Medium />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "State and size strictly controlled via enums."
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
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Spinner paused=false />
                    <Spinner paused=true />
                </div>
            </div>
        </div>
    }
}
