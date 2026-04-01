use leptos::prelude::*;
use super::modal_ui::{Modal, ModalOverlay, ModalContent};

#[component]
pub fn ModalShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Modal>
                    <button type="button" data-rs-modal-trigger="" data-rs-button="" data-rs-variant="primary">"Open Modal"</button>
                    <ModalOverlay />
                    <ModalContent>
                        <h2>"Modal Title"</h2>
                        <p>"Modal content with overlay. Visibility and accessibility fully synchronized via state."</p>
                        <button type="button" data-rs-modal-close="" data-rs-button="" data-rs-variant="outline">"Close"</button>
                    </ModalContent>
                </Modal>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Modal visibility and accessibility fully synchronized via state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With actions"</span>
                <div data-rs-showcase-preview-row="">
                    <Modal>
                        <button type="button" data-rs-modal-trigger="" data-rs-button="" data-rs-variant="outline">"View details"</button>
                        <ModalOverlay />
                        <ModalContent>
                            <h2>"Details"</h2>
                            <p>"Additional information displayed in a modal overlay."</p>
                            <div style="display:flex;gap:0.5rem;margin-top:1rem;justify-content:flex-end;">
                                <button type="button" data-rs-modal-close="" data-rs-button="" data-rs-variant="outline">"Cancel"</button>
                                <button type="button" data-rs-modal-close="" data-rs-button="" data-rs-variant="primary">"Confirm"</button>
                            </div>
                        </ModalContent>
                    </Modal>
                </div>
            </div>
        </div>
    }
}
