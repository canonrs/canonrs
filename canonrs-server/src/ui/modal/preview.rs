use leptos::prelude::*;
use super::boundary::{
    Modal, ModalTrigger, ModalOverlay,
    ModalContent, ModalTitle, ModalDescription,
    ModalClose, ModalFooter,
};
use crate::ui::button::Button;
use canonrs_core::primitives::ButtonVariant;

#[component]
pub fn ModalShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Modal>
                    <ModalTrigger>"Open Modal"</ModalTrigger>
                    <ModalOverlay />
                    <ModalContent>
                        <ModalTitle>"Confirm action"</ModalTitle>
                        <ModalDescription>"Are you sure? This action cannot be undone."</ModalDescription>
                        <ModalFooter>
                            <ModalClose>"Cancel"</ModalClose>
                            <Button variant=ButtonVariant::Primary>"Confirm"</Button>
                        </ModalFooter>
                    </ModalContent>
                </Modal>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Modal accessibility and lifecycle enforced via primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Form dialog"</span>
                <div data-rs-showcase-preview-row="">
                    <Modal>
                        <ModalTrigger>"Edit profile"</ModalTrigger>
                        <ModalOverlay />
                        <ModalContent>
                            <ModalTitle>"Edit profile"</ModalTitle>
                            <ModalDescription>"Update your profile information below."</ModalDescription>
                            <ModalFooter>
                                <ModalClose>"Cancel"</ModalClose>
                                <Button variant=ButtonVariant::Primary>"Save changes"</Button>
                            </ModalFooter>
                        </ModalContent>
                    </Modal>
                </div>
            </div>
        </div>
    }
}
