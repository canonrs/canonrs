use leptos::prelude::*;
use super::icon_ui::{Icon, IconSize, IconVariant};

#[component]
pub fn IconShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <Icon size=IconSize::Lg variant=IconVariant::Primary>"★"</Icon>
                    <Icon size=IconSize::Lg variant=IconVariant::Default>"★"</Icon>
                    <Icon size=IconSize::Lg variant=IconVariant::Muted>"★"</Icon>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Icon size and variant enforced via typed enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Icon size=IconSize::Sm>"★"</Icon>
                    <Icon size=IconSize::Md>"★"</Icon>
                    <Icon size=IconSize::Lg>"★"</Icon>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Icon variant=IconVariant::Default>"★"</Icon>
                    <Icon variant=IconVariant::Muted>"★"</Icon>
                    <Icon variant=IconVariant::Primary>"★"</Icon>
                    <Icon variant=IconVariant::Destructive>"★"</Icon>
                    <Icon variant=IconVariant::Success>"★"</Icon>
                    <Icon variant=IconVariant::Warning>"★"</Icon>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Spin"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Icon spin=true>"⟳"</Icon>
                </div>
            </div>
        </div>
    }
}
