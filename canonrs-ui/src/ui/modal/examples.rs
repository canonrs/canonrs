use leptos::prelude::*;
use super::modal_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Modal id="modal-ex".to_string()>
            <ModalContent>
                <ModalTitle>"Modal Title"</ModalTitle>
                <ModalBody>"Modal content"</ModalBody>
            </ModalContent>
        </Modal>
    }
}
