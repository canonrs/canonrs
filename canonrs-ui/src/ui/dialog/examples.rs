use leptos::prelude::*;
use super::dialog_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <DialogTrigger target_dialog_id="dialog-ex".to_string()>
                "Open Dialog"
            </DialogTrigger>

            <Dialog id="dialog-ex".to_string()>
                <DialogBackdrop />
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>"Dialog Title"</DialogTitle>
                    </DialogHeader>
                    <DialogBody>
                        <p>"Dialog content goes here."</p>
                    </DialogBody>
                    <DialogClose target_dialog_id="dialog-ex".to_string()>
                        "Close"
                    </DialogClose>
                </DialogContent>
            </Dialog>
        </div>
    }
}
