use leptos::prelude::*;
use super::boundary::IconButton;
use canonrs_core::primitives::{IconButtonVariant, IconButtonSize};

#[component]
pub fn IconButtonShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <IconButton
                    aria_label="Close"
                    variant=IconButtonVariant::Solid
                    size=IconButtonSize::Xl>
                    "✕"
                </IconButton>
            </div>
            <p data-rs-showcase-preview-anchor="">"Accessibility and loading state enforced in button contract."</p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <IconButton aria_label="Default" variant=IconButtonVariant::Default>"★"</IconButton>
                    <IconButton aria_label="Ghost" variant=IconButtonVariant::Ghost>"★"</IconButton>
                    <IconButton aria_label="Outline" variant=IconButtonVariant::Outline>"★"</IconButton>
                    <IconButton aria_label="Solid" variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Subtle" variant=IconButtonVariant::Subtle>"★"</IconButton>
                    <IconButton aria_label="Destructive" variant=IconButtonVariant::Destructive>"★"</IconButton>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="">
                    <IconButton aria_label="Xs" size=IconButtonSize::Xs variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Sm" size=IconButtonSize::Sm variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Md" size=IconButtonSize::Md variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Lg" size=IconButtonSize::Lg variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Xl" size=IconButtonSize::Xl variant=IconButtonVariant::Solid>"★"</IconButton>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Default"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButton aria_label="Default" variant=IconButtonVariant::Solid>"★"</IconButton>
                        </div>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Pressed"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButton aria_label="Pressed" variant=IconButtonVariant::Solid pressed=true>"★"</IconButton>
                        </div>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Disabled"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButton aria_label="Disabled" variant=IconButtonVariant::Solid disabled=true>"★"</IconButton>
                        </div>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Loading"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButton aria_label="Loading" variant=IconButtonVariant::Solid loading=true>"★"</IconButton>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    }
}
