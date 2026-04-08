use leptos::prelude::*;
use super::logo_island::LogoIsland;
use super::logo_ui::{LogoSize, LogoVariant};

#[component]
pub fn LogoShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <LogoIsland />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Brand identity structure and navigation behavior enforced in a single contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-lg);">
                    <LogoIsland size=LogoSize::Sm />
                    <LogoIsland size=LogoSize::Md />
                    <LogoIsland size=LogoSize::Lg />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-lg);">
                    <LogoIsland variant=LogoVariant::Brand />
                    <LogoIsland variant=LogoVariant::Neutral />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With wordmark"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-lg);">
                    <LogoIsland
                        wordmark=leptos::children::ToChildren::to_children(|| view! { "CanonRS" })
                    />
                    <LogoIsland
                        wordmark=leptos::children::ToChildren::to_children(|| view! { "CanonRS" })
                        tagline=leptos::children::ToChildren::to_children(|| view! { "Design System" })
                    />
                </div>
            </div>
        </div>
    }
}
