use leptos::prelude::*;
use super::modal_ui::*;
use crate::ui::button::button_ui::{Button, ButtonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Modal>
            <Button variant=ButtonVariant::Primary attr:data-rs-modal-trigger="">"Open Modal"</Button>
            <ModalOverlay />
            <ModalContent>
                <h2>"Modal Title"</h2>
                <p>"Modal content"</p>
                <div style="display:flex;gap:0.5rem;margin-top:1rem;">
                    <Button variant=ButtonVariant::Outline attr:data-rs-modal-close="">"Cancel"</Button>
                    <Button variant=ButtonVariant::Primary>"Confirm"</Button>
                </div>
            </ModalContent>
        </Modal>
    }
}
