use leptos::prelude::*;
use super::badge_island::BadgeIsland;

#[component]
pub fn BadgeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <BadgeIsland variant="success">"Active"</BadgeIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Interactivity explicitly defined and enforced by type."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <BadgeIsland>"Default"</BadgeIsland>
                    <BadgeIsland variant="primary">"Primary"</BadgeIsland>
                    <BadgeIsland variant="success">"Success"</BadgeIsland>
                    <BadgeIsland variant="warning">"Warning"</BadgeIsland>
                    <BadgeIsland variant="destructive">"Destructive"</BadgeIsland>
                    <BadgeIsland variant="outline">"Outline"</BadgeIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Interactivity"</span>
                <div data-rs-showcase-preview-row="">
                    <BadgeIsland>"Static"</BadgeIsland>
                    <BadgeIsland interactivity="interactive">"Interactive"</BadgeIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Count / status examples"</span>
                <div data-rs-showcase-preview-row="">
                    <BadgeIsland variant="primary">"12"</BadgeIsland>
                    <BadgeIsland variant="warning">"Pending"</BadgeIsland>
                    <BadgeIsland variant="destructive">"Failed"</BadgeIsland>
                    <BadgeIsland variant="outline">"Draft"</BadgeIsland>
                    <BadgeIsland variant="success">"Published"</BadgeIsland>
                </div>
            </div>
        </div>
    }
}
