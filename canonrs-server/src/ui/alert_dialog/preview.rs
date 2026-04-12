use leptos::prelude::*;
use super::boundary::AlertDialog;

#[component]
pub fn AlertDialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <AlertDialog
                    trigger_label="Delete Account"
                    title="Are you absolutely sure?"
                    description="This action cannot be undone. This will permanently delete your account."
                    confirm_label="Delete"
                    cancel_label="Cancel"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Alert dialog enforces destructive action confirmation via ARIA alertdialog."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Custom trigger"</span>
                <div data-rs-showcase-preview-row="">
                    <AlertDialog
                        trigger_label="Remove item"
                        title="Remove this item?"
                        description="This item will be permanently removed from your list."
                        confirm_label="Remove"
                        cancel_label="Keep it"
                    />
                </div>
            </div>
        </div>
    }
}
