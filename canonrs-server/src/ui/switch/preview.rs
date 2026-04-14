use leptos::prelude::*;
use super::switch_boundary::Switch;

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
                    <div style="display:flex;flex-direction:column;align-items:center;gap:var(--space-xs)">
                        <Switch />
                        <span data-rs-showcase-preview-label="">"Off"</span>
                    </div>
                    <div style="display:flex;flex-direction:column;align-items:center;gap:var(--space-xs)">
                        <Switch checked=true />
                        <span data-rs-showcase-preview-label="">"On"</span>
                    </div>
                    <div style="display:flex;flex-direction:column;align-items:center;gap:var(--space-xs)">
                        <Switch disabled=true />
                        <span data-rs-showcase-preview-label="">"Disabled Off"</span>
                    </div>
                    <div style="display:flex;flex-direction:column;align-items:center;gap:var(--space-xs)">
                        <Switch checked=true disabled=true />
                        <span data-rs-showcase-preview-label="">"Disabled On"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
