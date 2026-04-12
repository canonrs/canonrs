use leptos::prelude::*;
use super::boundary::Logo;
use super::logo_ui::{LogoSize, LogoVariant};

#[component]
pub fn LogoShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Logo />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Brand identity structure and navigation behavior enforced in a single contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-lg);">
                    <Logo size=LogoSize::Sm />
                    <Logo size=LogoSize::Md />
                    <Logo size=LogoSize::Lg />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-lg);">
                    <Logo variant=LogoVariant::Brand />
                    <Logo variant=LogoVariant::Neutral />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With wordmark"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-lg);">
                    <Logo
                        wordmark=leptos::children::ToChildren::to_children(|| view! { "CanonRS" })
                    />
                    <Logo
                        wordmark=leptos::children::ToChildren::to_children(|| view! { "CanonRS" })
                        tagline=leptos::children::ToChildren::to_children(|| view! { "Design System" })
                    />
                </div>
            </div>
        </div>
    }
}
