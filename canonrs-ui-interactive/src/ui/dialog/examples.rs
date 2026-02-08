use leptos::prelude::*;
use super::DialogInteractive;

pub fn basic_example() -> impl IntoView {
    view! {
        <DialogInteractive
            title="Welcome Dialog"
            trigger_text="Open Dialog"
        >
            <p>"This is an interactive dialog with real open/close functionality."</p>
            <p>"Click outside or press the close button to dismiss."</p>
        </DialogInteractive>
    }
}
