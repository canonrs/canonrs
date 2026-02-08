use leptos::callback::Callback;
use leptos::prelude::*;
use canonrs_ui::primitives::dialog::{DialogPrimitive, DialogBackdropPrimitive, DialogPopupPrimitive, DialogHeaderPrimitive, DialogTitlePrimitive, DialogBodyPrimitive, DialogClosePrimitive};
use crate::ui::button::ButtonInteractive;

#[component]
pub fn ConfirmDialogInteractive(
    open: Signal<bool>,
    #[prop(default = "Confirm".to_string())] title: String,
    #[prop(default = "Are you sure you want to continue?".to_string())] message: String,
    #[prop(default = "Confirm".to_string())] confirm_text: String,
    #[prop(default = "Cancel".to_string())] cancel_text: String,
    #[prop(optional)] on_confirm: Option<Callback<()>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
    #[prop(default = false)] destructive: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
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
        <DialogPrimitive open=open class={class} id={id}>
            <DialogBackdropPrimitive />
            <DialogPopupPrimitive>
                <DialogHeaderPrimitive>
                    <DialogTitlePrimitive>{move || title.get_value()}</DialogTitlePrimitive>
                    <DialogClosePrimitive>"×"</DialogClosePrimitive>
                </DialogHeaderPrimitive>

                <DialogBodyPrimitive>
                    <p class="text-sm text-muted-foreground">{move || message.get_value()}</p>
                </DialogBodyPrimitive>

                <div class="flex gap-3 justify-end mt-6">
                    <ButtonInteractive
                        class="px-4 py-2".to_string()
                        on_click=Callback::new(move |_| {
                            if let Some(ref handler) = on_cancel {
                                handler.run(());
                            }
                        })
                    >
                        {move || cancel_text.get_value()}
                    </ButtonInteractive>
                    <ButtonInteractive
                        class={format!("px-4 py-2 {}", confirm_class())}
                        on_click=Callback::new(move |_| {
                            if let Some(ref handler) = on_confirm {
                                handler.run(());
                            }
                        })
                    >
                        {move || confirm_text.get_value()}
                    </ButtonInteractive>
                </div>
            </DialogPopupPrimitive>
        </DialogPrimitive>
    }
}
