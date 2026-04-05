use leptos::prelude::*;

#[island]
pub fn ConfirmDialogIsland(
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] message: Option<String>,
    #[prop(optional, into)] confirm_text: Option<String>,
    #[prop(optional, into)] cancel_text: Option<String>,
    #[prop(optional, into)] destructive: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Open".to_string());
    let title         = title.unwrap_or_else(|| "Confirm".to_string());
    let message       = message.unwrap_or_else(|| "Are you sure you want to continue?".to_string());
    let confirm_text  = confirm_text.unwrap_or_else(|| "Confirm".to_string());
    let cancel_text   = cancel_text.unwrap_or_else(|| "Cancel".to_string());
    let initial_state = "closed";
    let (is_open, set_open) = signal(false);
    let _ = set_open;

    let state = move || if is_open.get() { "open" } else { "closed" };

    let confirm_variant = if destructive.unwrap_or(false) { "destructive" } else { "primary" };

    #[cfg(feature = "hydrate")]
    let on_open = move |_: leptos::ev::MouseEvent| {
        set_open.set(true);
        if let Some(body) = leptos::web_sys::window().unwrap().document().unwrap().body() {
            body.style().set_property("overflow", "hidden").ok();
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_open = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_close = move |_: leptos::ev::MouseEvent| {
        set_open.set(false);
        if let Some(body) = leptos::web_sys::window().unwrap().document().unwrap().body() {
            body.style().remove_property("overflow").ok();
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_close = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;
        let cb_esc = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" && is_open.get_untracked() {
                set_open.set(false);
                if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                    body.style().remove_property("overflow").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap()
            .add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();
    }

    view! {
        <div
            data-rs-confirm-dialog=""
            data-rs-component="ConfirmDialog"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            class=class
        >
            <button
                type="button"
                data-rs-confirm-dialog-trigger=""
                data-rs-button=""
                data-rs-variant=confirm_variant
                aria-haspopup="dialog"
                aria-expanded=move || is_open.get().to_string()
                on:click=on_open
            >
                {trigger_label}
            </button>
            <div data-rs-confirm-dialog-portal="">
                <div
                    data-rs-confirm-dialog-overlay=""
                    on:click=on_close
                ></div>
                <div
                    data-rs-confirm-dialog-content=""
                    role="alertdialog"
                    aria-modal="true"
                    tabindex="-1"
                >
                    <h2 data-rs-confirm-dialog-title="">{title}</h2>
                    <p data-rs-confirm-dialog-description="">{message}</p>
                    <div data-rs-confirm-dialog-footer="">
                        <button
                            type="button"
                            data-rs-confirm-dialog-cancel=""
                            data-rs-button=""
                            data-rs-variant="outline"
                            on:click=on_close
                        >
                            {cancel_text}
                        </button>
                        <button
                            type="button"
                            data-rs-confirm-dialog-confirm=""
                            data-rs-button=""
                            data-rs-variant=confirm_variant
                            on:click=on_close
                        >
                            {confirm_text}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
