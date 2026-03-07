use leptos::prelude::*;
use super::ConfirmDialog;

#[component]
pub fn basic_example() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div>
            <button on:click=move |_| open.set(true)>
                "Open Confirm Dialog"
            </button>
            <ConfirmDialog
                id="confirm-dialog-ex".to_string()
                title="Confirm Action".to_string()
                message="Are you sure you want to proceed?".to_string()
            />
        </div>
    }
}

#[component]
pub fn destructive_example() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div>
            <button on:click=move |_| open.set(true)>
                "Delete Item"
            </button>
            <ConfirmDialog
                id="confirm-dialog-ex".to_string()
                title="Delete Item".to_string()
                message="This action cannot be undone. Are you sure?".to_string()
                confirm_text="Delete".to_string()
                cancel_text="Cancel".to_string()
                destructive=true
            />
        </div>
    }
}

#[component]
pub fn custom_text_example() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div>
            <button on:click=move |_| open.set(true)>
                "Save Changes"
            </button>
            <ConfirmDialog
                id="confirm-dialog-ex".to_string()
                title="Save Changes".to_string()
                message="Do you want to save your changes before leaving?".to_string()
                confirm_text="Save".to_string()
                cancel_text="Discard".to_string()
            />
        </div>
    }
}
