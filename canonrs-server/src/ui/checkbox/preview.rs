use leptos::prelude::*;
use super::checkbox_ui::Checkbox;
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn CheckboxShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Checkbox checked=ActivityState::Active>"Remember me"</Checkbox>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Checked state mapped explicitly to activity state."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Unchecked"</span>
                        <Checkbox>"Unchecked"</Checkbox>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Checked"</span>
                        <Checkbox checked=ActivityState::Active>"Checked"</Checkbox>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Disabled"</span>
                        <Checkbox disabled=DisabledState::Disabled>"Disabled"</Checkbox>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Checked + Disabled"</span>
                        <Checkbox checked=ActivityState::Active disabled=DisabledState::Disabled>"Checked + Disabled"</Checkbox>
                    </div>
                </div>
            </div>

        </div>
    }
}
