use leptos::prelude::*;

#[island]
pub fn AlertDialogIsland(
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] confirm_label: Option<String>,
    #[prop(optional, into)] cancel_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Delete".to_string());
    let confirm_label = confirm_label.unwrap_or_else(|| "Confirm".to_string());
    let cancel_label  = cancel_label.unwrap_or_else(|| "Cancel".to_string());
    let (is_open, set_open) = signal(false);
    let initial_state = "closed";
    let state = move || if is_open.get() { "open" } else { "closed" };
    let _ = set_open;

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
    let on_confirm = move |_: leptos::ev::MouseEvent| {
        set_open.set(false);
        if let Some(body) = leptos::web_sys::window().unwrap().document().unwrap().body() {
            body.style().remove_property("overflow").ok();
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_confirm = move |_: leptos::ev::MouseEvent| {};

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
            data-rs-alert-dialog=""
            data-rs-component="AlertDialog"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            class=class
        >
            <button
                type="button"
                data-rs-alert-dialog-trigger=""
                data-rs-button=""
                data-rs-variant="destructive"
                aria-haspopup="alertdialog"
                on:click=on_open
            >
                {trigger_label}
            </button>
            <div data-rs-alert-dialog-portal="">
                <div data-rs-alert-dialog-overlay="" on:click=on_close></div>
                <div
                    data-rs-alert-dialog-content=""
                    role="alertdialog"
                    aria-modal="true"
                    aria-live="assertive"
                    tabindex="-1"
                >
                    {title.map(|t| view! { <h2 data-rs-alert-dialog-title="">{t}</h2> })}
                    {description.map(|d| view! { <p data-rs-alert-dialog-description="">{d}</p> })}
                    <div data-rs-alert-dialog-actions="">
                        <button type="button" data-rs-button="" data-rs-variant="outline" on:click=on_close>
                            {cancel_label}
                        </button>
                        <button type="button" data-rs-button="" data-rs-variant="destructive" on:click=on_confirm>
                            {confirm_label}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
