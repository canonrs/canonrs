use leptos::prelude::*;
use super::hover_card_island::HoverCardIsland;

#[component]
pub fn HoverCardShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <HoverCardIsland
                    trigger="@canonrs"
                    content="A design system built in Rust and Leptos. Joined January 2025."
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Hover state and positioning enforced via visibility contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Rich content"</span>
                <div data-rs-showcase-preview-row="">
                    <HoverCardIsland
                        trigger="Component docs"
                        content="Button — Triggers an action or event."
                    />
                </div>
            </div>
        </div>
    }
}
