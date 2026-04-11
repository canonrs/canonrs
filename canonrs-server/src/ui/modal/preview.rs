use leptos::prelude::*;
use super::modal_island::{
    ModalIsland, ModalTriggerIsland, ModalOverlayIsland,
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
                    <ModalOverlayIsland />
                    <ModalContentIsland>
                        <ModalTitleIsland>"Confirm action"</ModalTitleIsland>
                        <ModalDescriptionIsland>"Are you sure? This action cannot be undone."</ModalDescriptionIsland>
                        <ModalFooterIsland>
                            <ModalCloseIsland>"Cancel"</ModalCloseIsland>
                            <ButtonIsland variant=ButtonVariant::Primary>"Confirm"</ButtonIsland>
                        </ModalFooterIsland>
                    </ModalContentIsland>
                </ModalIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Modal accessibility and lifecycle enforced via primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Form dialog"</span>
                <div data-rs-showcase-preview-row="">
                    <ModalIsland>
                        <ModalTriggerIsland>"Edit profile"</ModalTriggerIsland>
                        <ModalOverlayIsland />
                        <ModalContentIsland>
                            <ModalTitleIsland>"Edit profile"</ModalTitleIsland>
                            <ModalDescriptionIsland>"Update your profile information below."</ModalDescriptionIsland>
                            <ModalFooterIsland>
                                <ModalCloseIsland>"Cancel"</ModalCloseIsland>
                                <ButtonIsland variant=ButtonVariant::Primary>"Save changes"</ButtonIsland>
                            </ModalFooterIsland>
                        </ModalContentIsland>
                    </ModalIsland>
                </div>
            </div>
        </div>
    }
}
