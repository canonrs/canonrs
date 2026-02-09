use leptos::prelude::*;
use super::modal_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <ModalTrigger target_modal_id="modal-ex">
                <button data-button data-ui-variant="solid">"Open Modal"</button>
            </ModalTrigger>
            <Modal id="modal-ex">
                <ModalOverlay />
                <ModalContent>
                    <h2>"Modal Title"</h2>
                    <p>"Modal content"</p>
                    <button data-button data-ui-variant="outline" onclick="document.getElementById('modal-ex').setAttribute('data-state', 'closed')">"Close"</button>
                </ModalContent>
            </Modal>
        </div>
    }
}
