use leptos::prelude::*;
use super::modal_island::ModalIsland;

#[component]
pub fn ModalShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ModalIsland
                    trigger_label="Open Modal"
                    description="Modal content with overlay. Visibility and accessibility fully synchronized via state."
                    close_label="Close"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Modal visibility and accessibility fully synchronized via state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With actions"</span>
                <div data-rs-showcase-preview-row="">
                    <ModalIsland
                        trigger_label="View details"
                        description="Additional information displayed in a modal overlay."
                        close_label="Confirm"
                    />
                </div>
            </div>
        </div>
    }
}
