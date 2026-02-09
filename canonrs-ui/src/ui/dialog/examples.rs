use leptos::prelude::*;
use super::dialog_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <DialogTrigger target_dialog_id="dialog-ex">
                <button data-button data-ui-variant="solid">"Open Dialog"</button>
            </DialogTrigger>
            <Dialog id="dialog-ex".to_string()>
                <DialogOverlay />
                <DialogContent>
                    <h2>"Dialog Title"</h2>
                    <p>"Dialog content"</p>
                    <button data-button data-ui-variant="outline" onclick="document.getElementById('dialog-ex').setAttribute('data-state', 'closed')">"Close"</button>
                </DialogContent>
            </Dialog>
        </div>
    }
}
