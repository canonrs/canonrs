use leptos::prelude::*;
use super::confirm_dialog_island::ConfirmDialogIsland;

#[component]
pub fn ConfirmDialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ConfirmDialogIsland
                    trigger_label="Delete item"
                    title="Delete item"
                    message="Are you sure you want to delete this item? This action cannot be undone."
                    confirm_text="Delete"
                    cancel_text="Cancel"
                    destructive=true
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Confirmation semantics and intent enforced via variant and structure."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Non-destructive"</span>
                <div data-rs-showcase-preview-row="">
                    <ConfirmDialogIsland
                        trigger_label="Save changes"
                        title="Save changes"
                        message="Do you want to save your changes before leaving?"
                        confirm_text="Save"
                        cancel_text="Discard"
                    />
                </div>
            </div>
        </div>
    }
}
