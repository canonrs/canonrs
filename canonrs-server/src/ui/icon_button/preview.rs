use leptos::prelude::*;
use super::icon_button_island::{IconButtonIsland, IconButtonVariant, IconButtonSize};

#[component]
pub fn IconButtonShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <IconButtonIsland
                    aria_label="Close"
                    variant=IconButtonVariant::Solid
                    size=IconButtonSize::Xl>
                    "✕"
                </IconButtonIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">"Accessibility and loading state enforced in button contract."</p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <IconButtonIsland aria_label="Default" variant=IconButtonVariant::Default>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Ghost" variant=IconButtonVariant::Ghost>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Outline" variant=IconButtonVariant::Outline>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Solid" variant=IconButtonVariant::Solid>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Subtle" variant=IconButtonVariant::Subtle>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Destructive" variant=IconButtonVariant::Destructive>"★"</IconButtonIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="">
                    <IconButtonIsland aria_label="Xs" size=IconButtonSize::Xs variant=IconButtonVariant::Solid>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Sm" size=IconButtonSize::Sm variant=IconButtonVariant::Solid>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Md" size=IconButtonSize::Md variant=IconButtonVariant::Solid>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Lg" size=IconButtonSize::Lg variant=IconButtonVariant::Solid>"★"</IconButtonIsland>
                    <IconButtonIsland aria_label="Xl" size=IconButtonSize::Xl variant=IconButtonVariant::Solid>"★"</IconButtonIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Default"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButtonIsland aria_label="Default" variant=IconButtonVariant::Solid>"★"</IconButtonIsland>
                        </div>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Pressed"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButtonIsland aria_label="Pressed" variant=IconButtonVariant::Solid pressed=true>"★"</IconButtonIsland>
                        </div>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Disabled"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButtonIsland aria_label="Disabled" variant=IconButtonVariant::Solid disabled=true>"★"</IconButtonIsland>
                        </div>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Loading"</span>
                        <div data-rs-showcase-preview-row="">
                            <IconButtonIsland aria_label="Loading" variant=IconButtonVariant::Solid loading=true>"★"</IconButtonIsland>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    }
}
