use leptos::prelude::*;
use super::switch_ui::Switch;

#[component]
pub fn SwitchShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Switch><span>{"Off"}</span></Switch>
                    <Switch checked=true><span>{"On"}</span></Switch>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Toggle state mapped directly to DOM and interaction state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Switch><span>{"Unchecked"}</span></Switch>
                    <Switch checked=true><span>{"Checked"}</span></Switch>
                    <Switch disabled=true><span>{"Disabled"}</span></Switch>
                    <Switch checked=true disabled=true><span>{"Checked + Disabled"}</span></Switch>
                </div>
            </div>
        </div>
    }
}
