use leptos::prelude::*;
use super::checkbox_island::CheckboxIsland;

#[component]
pub fn CheckboxShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CheckboxIsland checked=true>
                    <span>{"Remember me"}</span>
                </CheckboxIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Checked state mapped explicitly to activity state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <CheckboxIsland>
                        <span>{"Unchecked"}</span>
                    </CheckboxIsland>
                    <CheckboxIsland checked=true>
                        <span>{"Checked"}</span>
                    </CheckboxIsland>
                    <CheckboxIsland disabled=true>
                        <span>{"Disabled"}</span>
                    </CheckboxIsland>
                    <CheckboxIsland checked=true disabled=true>
                        <span>{"Checked + Disabled"}</span>
                    </CheckboxIsland>
                </div>
            </div>
        </div>
    }
}
