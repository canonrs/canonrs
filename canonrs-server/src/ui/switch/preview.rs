use leptos::prelude::*;
use super::switch_island::SwitchIsland;

#[component]
pub fn SwitchShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SwitchIsland />
                <SwitchIsland checked=true />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Toggle state mapped directly to DOM and interaction state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <SwitchIsland />
                    <SwitchIsland checked=true />
                    <SwitchIsland disabled=true />
                    <SwitchIsland checked=true disabled=true />
                </div>
            </div>
        </div>
    }
}
