use leptos::prelude::*;
use super::alert_dialog_ui::{
    AlertDialog, AlertDialogTrigger, AlertDialogPortal, AlertDialogOverlay,
    AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogClose,
};
use crate::ui::button::ButtonVariant;

#[component]
pub fn AlertDialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <AlertDialog>
                    <AlertDialogTrigger>"Delete account"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay />
                        <AlertDialogContent>
                            <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                            <AlertDialogDescription>
                                "This action cannot be undone. Your account and all data will be permanently deleted."
                            </AlertDialogDescription>
                            <div style="display:flex;gap:0.5rem;margin-top:1rem;justify-content:flex-end;">
                                <AlertDialogClose>"Cancel"</AlertDialogClose>
                                <AlertDialogClose variant=ButtonVariant::Destructive>"Delete"</AlertDialogClose>
                            </div>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Alertdialog role and accessibility guaranteed by component contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Confirm action"</span>
                <div data-rs-showcase-preview-row="">
                    <AlertDialog>
                        <AlertDialogTrigger variant=ButtonVariant::Outline>"Clear all data"</AlertDialogTrigger>
                        <AlertDialogPortal>
                            <AlertDialogOverlay />
                            <AlertDialogContent>
                                <AlertDialogTitle>"Clear all data?"</AlertDialogTitle>
                                <AlertDialogDescription>
                                    "All saved data will be removed. This cannot be recovered."
                                </AlertDialogDescription>
                                <div style="display:flex;gap:0.5rem;margin-top:1rem;justify-content:flex-end;">
                                    <AlertDialogClose>"Cancel"</AlertDialogClose>
                                    <AlertDialogClose variant=ButtonVariant::Primary>"Confirm"</AlertDialogClose>
                                </div>
                            </AlertDialogContent>
                        </AlertDialogPortal>
                    </AlertDialog>
                </div>
            </div>
        </div>
    }
}
