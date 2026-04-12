use leptos::prelude::*;
use super::boundary::Switch;

#[component]
pub fn SwitchShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Switch />
                <Switch checked=true />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Toggle state mapped directly to DOM and interaction state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <Switch />
                    <Switch checked=true />
                    <Switch disabled=true />
                    <Switch checked=true disabled=true />
                </div>
            </div>
        </div>
    }
}
