use leptos::prelude::*;
use super::dialog_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <DialogTrigger target_dialog_id="dialog-basic">"Open Dialog"</DialogTrigger>
            <Dialog id="dialog-basic">
                <DialogOverlay />
                <DialogContent
                    labelledby="dialog-basic-title"
                    describedby="dialog-basic-desc"
                >
                    <DialogTitle id="dialog-basic-title">"Confirm Action"</DialogTitle>
                    <DialogDescription id="dialog-basic-desc">
                        "Are you sure you want to proceed? This action cannot be undone."
                    </DialogDescription>
                    <div style="display:flex;gap:0.5rem;margin-top:1rem;">
                        <DialogClose target_dialog_id="dialog-basic">"Cancel"</DialogClose>
                        <button data-button data-ui-variant="solid">"Confirm"</button>
                    </div>
                </DialogContent>
            </Dialog>
        </div>
    }
}
