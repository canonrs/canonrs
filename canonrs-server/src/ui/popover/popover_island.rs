use leptos::prelude::*;

#[island]
pub fn PopoverIsland(
    #[prop(into)] trigger_label: String,
    #[prop(into)] content: String,
    #[prop(optional, into)] side: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let side  = side.unwrap_or_else(|| "bottom".to_string());
    let (is_open, set_open) = signal(false);
    let _ = set_open;

    let state = move || if is_open.get() { "open" } else { "closed" };

    #[cfg(feature = "hydrate")]
    let on_toggle = move |_: leptos::ev::MouseEvent| { set_open.update(|v| *v = !*v); };
    #[cfg(not(feature = "hydrate"))]
    let on_toggle = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_close = move |_: leptos::ev::MouseEvent| { set_open.set(false); };
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
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap()
            .add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();
    }

    view! {
        <span
            data-rs-popover=""
            data-rs-component="Popover"
            data-rs-state=move || state()
            class=class
        >
            <button
                type="button"
                data-rs-popover-trigger=""
                data-rs-button=""
                data-rs-variant="outline"
                aria-haspopup="true"
                aria-expanded=move || is_open.get().to_string()
                on:click=on_toggle
            >
                {trigger_label}
            </button>
            <div
                data-rs-popover-content=""
                data-rs-side=side
                data-rs-state=move || state()
                role="dialog"
                aria-modal="false"
            >
                <p>{content}</p>
                <button
                    type="button"
                    data-rs-popover-close=""
                    data-rs-button=""
                    data-rs-variant="ghost"
                    on:click=on_close
                >
                    "Close"
                </button>
            </div>
        </span>
    }
}
