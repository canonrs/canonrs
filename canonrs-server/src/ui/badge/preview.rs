use leptos::prelude::*;
use super::badge_island::BadgeIsland;
use canonrs_core::primitives::{BadgeVariant, BadgeInteractivity};

#[component]
pub fn BadgeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <BadgeIsland variant=BadgeVariant::Success>"Active"</BadgeIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Interactivity explicitly defined and enforced by type."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <BadgeIsland>"Default"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Primary>"Primary"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Success>"Success"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Warning>"Warning"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Destructive>"Destructive"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Outline>"Outline"</BadgeIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Interactivity"</span>
                <div data-rs-showcase-preview-row="">
                    <BadgeIsland>"Static"</BadgeIsland>
                    <BadgeIsland interactivity=BadgeInteractivity::Interactive>"Interactive"</BadgeIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Count / status examples"</span>
                <div data-rs-showcase-preview-row="">
                    <BadgeIsland variant=BadgeVariant::Primary>"12"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Warning>"Pending"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Destructive>"Failed"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Outline>"Draft"</BadgeIsland>
                    <BadgeIsland variant=BadgeVariant::Success>"Published"</BadgeIsland>
                </div>
            </div>
        </div>
    }
}
