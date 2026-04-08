//! @canon-level: strict
//! ConfirmDialog Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::confirm_dialog_ui::ConfirmDialog;

#[island]
pub fn ConfirmDialogInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::confirm_dialog::init_all();
        });
    }
    view! { <></> }
}

#[component]
pub fn ConfirmDialogIsland(
    #[prop(into, default = "Confirm".to_string())] title: String,
    #[prop(into, default = "Are you sure you want to continue?".to_string())] message: String,
    #[prop(into, default = "Confirm".to_string())] confirm_text: String,
    #[prop(into, default = "Cancel".to_string())] cancel_text: String,
    #[prop(default = false)] destructive: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ConfirmDialogInit />
        <ConfirmDialog
            title=title
            message=message
            confirm_text=confirm_text
            cancel_text=cancel_text
            destructive=destructive
            class=class
        />
    }
}
