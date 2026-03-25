use leptos::prelude::*;
use super::dialog_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Open Dialog"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay />
                <DialogContent>
                    <DialogTitle>"Confirm Action"</DialogTitle>
                    <DialogDescription>
                        "Are you sure you want to proceed? This action cannot be undone."
                    </DialogDescription>
                    <div style="display:flex;gap:0.5rem;margin-top:1rem;">
                        <DialogClose>"Cancel"</DialogClose>
                        <button type="button">"Confirm"</button>
                    </div>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
