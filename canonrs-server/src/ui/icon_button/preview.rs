use leptos::prelude::*;
use super::icon_button_ui::{IconButton, IconButtonVariant};
use crate::ui::icon::IconSize;
use canonrs_core::meta::{DisabledState, LoadingState};

#[component]
pub fn IconButtonShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <IconButton
                    aria_label="Close".to_string()
                    variant=IconButtonVariant::Solid
                    size=IconSize::Lg>
                    "✕"
                </IconButton>
            </div>
            <p data-rs-showcase-preview-anchor="">"Accessibility and loading state enforced in button contract."</p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <IconButton aria_label="Default".to_string() variant=IconButtonVariant::Default>"★"</IconButton>
                    <IconButton aria_label="Ghost".to_string() variant=IconButtonVariant::Ghost>"★"</IconButton>
                    <IconButton aria_label="Outline".to_string() variant=IconButtonVariant::Outline>"★"</IconButton>
                    <IconButton aria_label="Solid".to_string() variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Destructive".to_string() variant=IconButtonVariant::Destructive>"★"</IconButton>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="">
<IconButton aria_label="Sm".to_string() size=IconSize::Sm>"★"</IconButton>
                    <IconButton aria_label="Md".to_string() size=IconSize::Md>"★"</IconButton>
                    <IconButton aria_label="Lg".to_string() size=IconSize::Lg>"★"</IconButton>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <IconButton aria_label="Active".to_string()>"★"</IconButton>
                    <IconButton aria_label="Disabled".to_string() disabled=DisabledState::Disabled>"★"</IconButton>
                    <IconButton aria_label="Loading".to_string() loading=LoadingState::Loading>"★"</IconButton>
                </div>
            </div>
        </div>
    }
}
