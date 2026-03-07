use leptos::prelude::*;
use super::alert_dialog_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <AlertDialogTrigger target_dialog_id="alert-dialog-basic">
                "Delete Account"
            </AlertDialogTrigger>
            <AlertDialog id="alert-dialog-basic">
                <AlertDialogOverlay />
                <AlertDialogContent
                    labelledby="alert-dialog-title"
                    describedby="alert-dialog-desc"
                >
                    <AlertDialogTitle id="alert-dialog-title">
                        "Are you absolutely sure?"
                    </AlertDialogTitle>
                    <AlertDialogDescription id="alert-dialog-desc">
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    </AlertDialogDescription>
                    <div style="display:flex;gap:0.5rem;margin-top:1rem;justify-content:flex-end;">
                        <AlertDialogClose target_dialog_id="alert-dialog-basic">
                            "Cancel"
                        </AlertDialogClose>
                        <button data-button data-ui-variant="solid" data-variant="destructive">
                            "Delete Account"
                        </button>
                    </div>
                </AlertDialogContent>
            </AlertDialog>
        </div>
    }
}
