use leptos::prelude::*;
use super::alert_dialog_ui::*;
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Delete Account"</AlertDialogTrigger>
            <AlertDialogPortal>
                <AlertDialogOverlay />
                <AlertDialogContent>
                    <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "This action cannot be undone. This will permanently delete your
                        account and remove your data from our servers."
                    </AlertDialogDescription>
                    <div style="display:flex;gap:0.5rem;margin-top:1rem;justify-content:flex-end;">
                        <AlertDialogClose>"Cancel"</AlertDialogClose>
                        <Button variant=ButtonVariant::Danger>"Delete Account"</Button>
                    </div>
                </AlertDialogContent>
            </AlertDialogPortal>
        </AlertDialog>
    }
}
