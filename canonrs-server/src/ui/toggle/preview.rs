use leptos::prelude::*;
use super::boundary::Toggle;

#[component]
pub fn ToggleShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Toggle><span>{"Bold"}</span></Toggle>
                <Toggle pressed=true><span>{"Italic"}</span></Toggle>
                <Toggle><span>{"Underline"}</span></Toggle>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Pressed state mapped directly to DOM and interaction state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <Toggle><span>{"Unpressed"}</span></Toggle>
                    <Toggle pressed=true><span>{"Pressed"}</span></Toggle>
                </div>
            </div>
        </div>
    }
}
