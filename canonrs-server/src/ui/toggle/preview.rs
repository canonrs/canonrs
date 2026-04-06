use leptos::prelude::*;
use super::toggle_island::ToggleIsland;

#[component]
pub fn ToggleShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ToggleIsland><span>{"Bold"}</span></ToggleIsland>
                <ToggleIsland pressed=true><span>{"Italic"}</span></ToggleIsland>
                <ToggleIsland><span>{"Underline"}</span></ToggleIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Pressed state mapped directly to DOM and interaction state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleIsland><span>{"Unpressed"}</span></ToggleIsland>
                    <ToggleIsland pressed=true><span>{"Pressed"}</span></ToggleIsland>
                </div>
            </div>
        </div>
    }
}
