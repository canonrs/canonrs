use leptos::prelude::*;
use super::dialog_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <DialogTrigger target_dialog_id="dialog-ex".to_string()>
                <button data-button data-ui-variant="default">"Open Dialog"</button>
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
                        <button data-button data-ui-variant="outline">"Close"</button>
                    </DialogClose>
                </DialogContent>
            </Dialog>
        </div>
    }
}
