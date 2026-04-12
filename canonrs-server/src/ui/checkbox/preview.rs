use leptos::prelude::*;
use super::boundary::Checkbox;

#[component]
pub fn CheckboxShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Checkbox checked=true>
                    <span>{"Remember me"}</span>
                </Checkbox>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Checked state mapped explicitly to activity state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <Checkbox>
                        <span>{"Unchecked"}</span>
                    </Checkbox>
                    <Checkbox checked=true>
                        <span>{"Checked"}</span>
                    </Checkbox>
                    <Checkbox disabled=true>
                        <span>{"Disabled"}</span>
                    </Checkbox>
                    <Checkbox checked=true disabled=true>
                        <span>{"Checked + Disabled"}</span>
                    </Checkbox>
                </div>
            </div>
        </div>
    }
}
