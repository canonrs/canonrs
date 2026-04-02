use leptos::prelude::*;
use super::badge_ui::{Badge, BadgeVariant};
use canonrs_core::primitives::BadgeInteractivity;

#[component]
pub fn BadgeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Badge variant=BadgeVariant::Success>"Active"</Badge>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Interactivity explicitly defined and enforced by type."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Primary>"Primary"</Badge>
                    <Badge variant=BadgeVariant::Success>"Success"</Badge>
                    <Badge variant=BadgeVariant::Warning>"Warning"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Interactivity"</span>
                <div data-rs-showcase-preview-row="">
                    <Badge interactivity=BadgeInteractivity::Static>"Static"</Badge>
                    <Badge interactivity=BadgeInteractivity::Interactive>"Interactive"</Badge>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Count / status examples"</span>
                <div data-rs-showcase-preview-row="">
                    <Badge variant=BadgeVariant::Primary>"12"</Badge>
                    <Badge variant=BadgeVariant::Warning>"Pending"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Failed"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Draft"</Badge>
                    <Badge variant=BadgeVariant::Success>"Published"</Badge>
                </div>
            </div>
        </div>
    }
}
