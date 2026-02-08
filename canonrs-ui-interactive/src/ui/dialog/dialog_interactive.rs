use leptos::prelude::*;
use leptos::callback::Callback;
use canonrs_ui::ui::dialog::*;

#[component]
pub fn DialogInteractive(
    /// Dialog title
    #[prop(into)]
    title: String,
    /// Dialog body content
    children: Children,
    /// Optional trigger button text
    #[prop(default = "Open".into(), into)]
    trigger_text: String,
    /// Optional close button text
    #[prop(default = "Close".into(), into)]
    close_text: String,
    /// Callback when dialog opens
    #[prop(optional)]
    on_open: Option<Callback<()>>,
    /// Callback when dialog closes
    #[prop(optional)]
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    let open = create_rw_signal(false);
    let dialog_id = format!("dialog-interactive");
    let trigger_id = format!("{}-trigger", dialog_id);
    let close_id = format!("{}-close", dialog_id);

    let handle_open = move |_| {
        open.set(true);
        if let Some(cb) = on_open {
            cb.run(());
        }
    };

    let handle_close = move |_| {
        open.set(false);
        if let Some(cb) = on_close {
            cb.run(());
        }
    };

    view! {
        <div>
            <button
                id=trigger_id
                data-button
                data-ui-variant="default"
                on:click=handle_open
            >
                {trigger_text}
            </button>

            <Dialog id=dialog_id.clone()>
                <DialogBackdrop on:click=handle_close />
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>{title}</DialogTitle>
                    </DialogHeader>
                    <DialogBody>
                        {children()}
                    </DialogBody>
                    <DialogClose target_dialog_id=dialog_id.clone() id=close_id>
                        <button
                            data-button
                            data-ui-variant="outline"
                            on:click=handle_close
                        >
                            {close_text}
                        </button>
                    </DialogClose>
                </DialogContent>
            </Dialog>
        </div>
    }
}
