use leptos::prelude::*;
use super::alert_dialog_island::AlertDialogIsland;

#[component]
pub fn AlertDialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <AlertDialogIsland
                    trigger_label="Delete account"
                    title="Are you absolutely sure?"
                    description="This action cannot be undone. Your account will be permanently deleted."
                    confirm_label="Delete"
                    cancel_label="Cancel"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Alertdialog role and accessibility guaranteed by component contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Confirm action"</span>
                <div data-rs-showcase-preview-row="">
                    <AlertDialogIsland
                        trigger_label="Clear all data"
                        title="Clear all data?"
                        description="All saved data will be removed. This cannot be recovered."
                        confirm_label="Confirm"
                        cancel_label="Cancel"
                    />
                </div>
            </div>
        </div>
    }
}
