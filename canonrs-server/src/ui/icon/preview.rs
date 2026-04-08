use leptos::prelude::*;
use super::icon_island::IconIsland;
use super::icon_ui::{IconSize, IconVariant};

#[component]
pub fn IconShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland size=IconSize::Lg variant=IconVariant::Primary>"★"</IconIsland>
                    <IconIsland size=IconSize::Lg>"★"</IconIsland>
                    <IconIsland size=IconSize::Lg variant=IconVariant::Muted>"★"</IconIsland>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Icon size and variant enforced via typed enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland size=IconSize::Sm>"★"</IconIsland>
                    <IconIsland size=IconSize::Md>"★"</IconIsland>
                    <IconIsland size=IconSize::Lg>"★"</IconIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland>"★"</IconIsland>
                    <IconIsland variant=IconVariant::Muted>"★"</IconIsland>
                    <IconIsland variant=IconVariant::Primary>"★"</IconIsland>
                    <IconIsland variant=IconVariant::Destructive>"★"</IconIsland>
                    <IconIsland variant=IconVariant::Success>"★"</IconIsland>
                    <IconIsland variant=IconVariant::Warning>"★"</IconIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Spin"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <IconIsland spin=true>"⟳"</IconIsland>
                </div>
            </div>
        </div>
    }
}
