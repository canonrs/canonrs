use leptos::prelude::*;
use canonrs_core::primitives::{
    DialogPrimitive, DialogOverlayPrimitive,
    DialogContentPrimitive, DialogTitlePrimitive,
    DialogDescriptionPrimitive, DialogClosePrimitive,
};
use crate::ui::button::Button;

#[component]
pub fn ConfirmDialog(
    #[prop(default = "Confirm".to_string(), into)] title: String,
    #[prop(default = "Are you sure you want to continue?".to_string(), into)] message: String,
    #[prop(default = "Confirm".to_string(), into)] confirm_text: String,
    #[prop(default = "Cancel".to_string(), into)] cancel_text: String,
    #[prop(default = false)] destructive: bool,
    #[prop(into, default = String::new())] class: String,
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
        <DialogPrimitive class=class>
            <DialogOverlayPrimitive />
            <DialogContentPrimitive>
                <div class="flex items-center justify-between mb-4">
                    <DialogTitlePrimitive>
                        {move || title.get_value()}
                    </DialogTitlePrimitive>
                    <DialogClosePrimitive>"×"</DialogClosePrimitive>
                </div>
                <DialogDescriptionPrimitive>
                    {move || message.get_value()}
                </DialogDescriptionPrimitive>
                <div class="flex gap-3 justify-end mt-6">
                    <DialogClosePrimitive class="px-4 py-2".to_string()>
                        {move || cancel_text.get_value()}
                    </DialogClosePrimitive>
                    <Button class=format!("px-4 py-2 {}", confirm_class())>
                        {move || confirm_text.get_value()}
                    </Button>
                </div>
            </DialogContentPrimitive>
        </DialogPrimitive>
    }
}

#[component]
pub fn ConfirmDialogPreview() -> impl IntoView {
    view! { <ConfirmDialog /> }
}
