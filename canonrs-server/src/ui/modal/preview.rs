use leptos::prelude::*;
use super::modal_boundary::{
    Modal, ModalTrigger, ModalPortal, ModalOverlay,
    ModalContent, ModalTitle, ModalDescription,
    ModalClose, ModalFooter,
};
use crate::ui::button::button_boundary::Button;
use canonrs_core::primitives::ButtonVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ModalShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Modal accessibility and lifecycle enforced via primitives."
            </p>
            <Modal>
                <ModalTrigger>"Open Modal"</ModalTrigger>
                <ModalPortal>
                <ModalOverlay />
                <ModalContent>
                    <ModalTitle>"Confirm action"</ModalTitle>
                    <ModalDescription>"Are you sure? This action cannot be undone."</ModalDescription>
                    <ModalFooter>
                        <ModalClose>"Cancel"</ModalClose>
                        <Button variant=ButtonVariant::Primary>"Confirm"</Button>
                    </ModalFooter>
                </ModalContent>
                </ModalPortal>
            </Modal>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Form dialog"</span>
                <Modal>
                    <ModalTrigger>"Edit profile"</ModalTrigger>
                    <ModalPortal>
                    <ModalOverlay />
                    <ModalContent>
                        <ModalTitle>"Edit profile"</ModalTitle>
                        <ModalDescription>"Update your profile information below."</ModalDescription>
                        <ModalFooter>
                            <ModalClose>"Cancel"</ModalClose>
                            <Button variant=ButtonVariant::Primary>"Save changes"</Button>
                        </ModalFooter>
                    </ModalContent>
                    </ModalPortal>
                </Modal>
            </Stack>
        </Stack>
    }
}
