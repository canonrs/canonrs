use leptos::prelude::*;
use super::modal_island::{
    ModalIsland, ModalTriggerIsland, ModalPortalIsland, ModalOverlayIsland,
    ModalContentIsland, ModalTitleIsland, ModalDescriptionIsland,
    ModalCloseIsland, ModalFooterIsland,
};
use crate::ui::button::button_island::ButtonIsland;
use canonrs_core::primitives::ButtonVariant;

#[component]
pub fn ModalShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ModalIsland>
                    <ModalTriggerIsland>"Open Modal"</ModalTriggerIsland>
                    <ModalPortalIsland>
                        <ModalOverlayIsland />
                        <ModalContentIsland>
                            <ModalTitleIsland>"Modal title"</ModalTitleIsland>
                            <ModalDescriptionIsland>"Modal content with overlay."</ModalDescriptionIsland>
                            <ModalFooterIsland>
                                <ModalCloseIsland>"Cancel"</ModalCloseIsland>
                                <ButtonIsland variant=ButtonVariant::Primary>"Confirm"</ButtonIsland>
                            </ModalFooterIsland>
                        </ModalContentIsland>
                    </ModalPortalIsland>
                </ModalIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Modal visibility and accessibility fully synchronized via state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With actions"</span>
                <div data-rs-showcase-preview-row="">
                    <ModalIsland>
                        <ModalTriggerIsland>"View details"</ModalTriggerIsland>
                        <ModalPortalIsland>
                            <ModalOverlayIsland />
                            <ModalContentIsland>
                                <ModalTitleIsland>"Details"</ModalTitleIsland>
                                <ModalDescriptionIsland>"Additional information displayed in a modal overlay."</ModalDescriptionIsland>
                                <ModalFooterIsland>
                                    <ModalCloseIsland>"Cancel"</ModalCloseIsland>
                                    <ButtonIsland variant=ButtonVariant::Primary>"Confirm"</ButtonIsland>
                                </ModalFooterIsland>
                            </ModalContentIsland>
                        </ModalPortalIsland>
                    </ModalIsland>
                </div>
            </div>
        </div>
    }
}
