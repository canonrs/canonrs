use leptos::prelude::*;
use super::alert_dialog_boundary::AlertDialog;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn AlertDialogShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <AlertDialog
                trigger_label="Delete Account"
                title="Are you absolutely sure?"
                description="This action cannot be undone. This will permanently delete your account."
                confirm_label="Delete"
                cancel_label="Cancel"
            />
            <p data-rs-showcase-preview-anchor="">
                "Alert dialog enforces destructive action confirmation via ARIA alertdialog."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Custom trigger"</span>
                <AlertDialog
                    trigger_label="Remove item"
                    title="Remove this item?"
                    description="This item will be permanently removed from your list."
                    confirm_label="Remove"
                    cancel_label="Keep it"
                />
            </Stack>
        </Stack>
    }
}
