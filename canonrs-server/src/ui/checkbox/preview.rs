use leptos::prelude::*;
use super::checkbox_ui::Checkbox;

#[component]
pub fn CheckboxShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Checkbox><span>{"Remember me"}</span></Checkbox>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Checked state mapped explicitly to activity state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Checkbox><span>{"Unchecked"}</span></Checkbox>
                    <Checkbox checked=true><span>{"Checked"}</span></Checkbox>
                    <Checkbox disabled=true><span>{"Disabled"}</span></Checkbox>
                    <Checkbox checked=true disabled=true><span>{"Checked + Disabled"}</span></Checkbox>
                </div>
            </div>
        </div>
    }
}
