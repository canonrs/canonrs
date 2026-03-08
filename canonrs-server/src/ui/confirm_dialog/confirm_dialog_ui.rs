use leptos::prelude::*;
use canonrs_core::primitives::{
    DialogPrimitive, DialogOverlayPrimitive,
    DialogContentPrimitive, DialogTitlePrimitive, DialogDescriptionPrimitive,
};
use crate::ui::button::Button;

#[component]
pub fn ConfirmDialog(
    #[prop(into)] id: String,
    #[prop(default = "Confirm".to_string())] title: String,
    #[prop(default = "Are you sure you want to continue?".to_string())] message: String,
    #[prop(default = "Confirm".to_string())] confirm_text: String,
    #[prop(default = "Cancel".to_string())] cancel_text: String,
    #[prop(default = false)] destructive: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let title = StoredValue::new(title);
    let message = StoredValue::new(message);
    let confirm_text = StoredValue::new(confirm_text);
    let cancel_text = StoredValue::new(cancel_text);

    let confirm_class = move || {
        if destructive {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90"
        } else {
            "bg-primary text-primary-foreground hover:bg-primary/90"
        }
    };

    view! {
        <DialogPrimitive class=class id=id.clone()>
            <DialogOverlayPrimitive />
            <DialogContentPrimitive labelledby=format!("{}-title", id) describedby=format!("{}-desc", id)>
                <div class="flex items-center justify-between mb-4">
                    <DialogTitlePrimitive id=format!("{}-title", id)>{move || title.get_value()}</DialogTitlePrimitive>
                    <button data-dialog-close=id.clone() type="button">"×"</button>
                </div>
                <DialogDescriptionPrimitive id=format!("{}-desc", id)>
                    {move || message.get_value()}
                </DialogDescriptionPrimitive>
                <div class="flex gap-3 justify-end mt-6">
                    <Button class="px-4 py-2".to_string()>{move || cancel_text.get_value()}</Button>
                    <Button class=format!("px-4 py-2 {}", confirm_class())>{move || confirm_text.get_value()}</Button>
                </div>
            </DialogContentPrimitive>
        </DialogPrimitive>
    }
}

#[component]
pub fn ConfirmDialogPreview() -> impl IntoView {
    view! { <ConfirmDialog id="confirm-preview".to_string() /> }
}
