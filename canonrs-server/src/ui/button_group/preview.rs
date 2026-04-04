use leptos::prelude::*;
use super::button_group_island::{ButtonGroupIsland, ButtonGroupAttached};
use crate::ui::button::button_island::{ButtonIsland, ButtonVariant, ButtonSize, ButtonStateHint};

#[component]
pub fn ButtonGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ButtonGroupIsland attached=ButtonGroupAttached::Attached aria_label="Actions">
                    <ButtonIsland variant=ButtonVariant::Secondary size=ButtonSize::Md>"Left"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Secondary size=ButtonSize::Md>"Center"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Secondary size=ButtonSize::Md>"Right"</ButtonIsland>
                </ButtonGroupIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">"Grouped actions. One contract."</p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary>"A"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary>"B"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary>"C"</ButtonIsland>
                    </ButtonGroupIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupIsland attached=ButtonGroupAttached::Attached>
                        <ButtonIsland variant=ButtonVariant::Primary>"Save"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary>"Cancel"</ButtonIsland>
                    </ButtonGroupIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — first/last radius"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupIsland attached=ButtonGroupAttached::Attached aria_label="First last demo">
                        <ButtonIsland variant=ButtonVariant::Secondary state_hint=ButtonStateHint::First>"First"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary>"Middle"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Last>"Last"</ButtonIsland>
                    </ButtonGroupIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — hover z-index"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupIsland attached=ButtonGroupAttached::Attached aria_label="Hover z-index demo">
                        <ButtonIsland variant=ButtonVariant::Secondary>"One"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Hover>"Hover"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary>"Three"</ButtonIsland>
                    </ButtonGroupIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — focus z-index"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupIsland attached=ButtonGroupAttached::Attached aria_label="Focus z-index demo">
                        <ButtonIsland variant=ButtonVariant::Secondary>"One"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Focus>"Focus"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Secondary>"Three"</ButtonIsland>
                    </ButtonGroupIsland>
                </div>
            </div>

        </div>
    }
}
