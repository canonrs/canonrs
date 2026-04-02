use leptos::prelude::*;
use super::toggle_ui::Toggle;

#[component]
pub fn ToggleShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;gap:var(--space-sm);">
                    <Toggle><span>{"Bold"}</span></Toggle>
                    <Toggle pressed=true><span>{"Italic"}</span></Toggle>
                    <Toggle><span>{"Underline"}</span></Toggle>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Pressed state mapped directly to DOM and interaction state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;gap:var(--space-sm);">
                    <Toggle><span>{"Unpressed"}</span></Toggle>
                    <Toggle pressed=true><span>{"Pressed"}</span></Toggle>
                </div>
            </div>
        </div>
    }
}
