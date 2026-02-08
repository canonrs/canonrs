use leptos::prelude::*;
use super::alert_dialog_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <button id="open-alert-dialog" data-button>"Show Alert"</button>
        </div>
    }
}
