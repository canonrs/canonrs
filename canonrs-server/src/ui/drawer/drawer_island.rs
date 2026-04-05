use leptos::prelude::*;

#[island]
pub fn DrawerIsland(
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] close_label: Option<String>,
    #[prop(optional, into)] side: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Open".to_string());
    let close_label   = close_label.unwrap_or_else(|| "Close".to_string());
    let side          = side.unwrap_or_else(|| "left".to_string());
    let initial_state = "closed";
    let (is_open, set_open) = signal(false);
    let _ = set_open;

    let state = move || if is_open.get() { "open" } else { "closed" };

    #[cfg(feature = "hydrate")]
    let on_open = {
        let side = side.clone();
        move |_: leptos::ev::MouseEvent| {
            set_open.set(true);
            if let Some(body) = leptos::web_sys::window().unwrap().document().unwrap().body() {
                body.style().set_property("overflow", "hidden").ok();
            }
            let _ = &side;
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
            data-rs-drawer=""
            data-rs-component="Drawer"
            data-rs-side=side.clone()
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            class=class
        >
            <button
                type="button"
                data-rs-drawer-trigger=""
                data-rs-button=""
                data-rs-variant="primary"
                aria-haspopup="dialog"
                aria-expanded=move || is_open.get().to_string()
                on:click=on_open
            >
                {trigger_label}
            </button>
            <div data-rs-drawer-portal="">
                <div
                    data-rs-drawer-overlay=""
                    on:click=on_close
                ></div>
                <div
                    data-rs-drawer-content=""
                    role="dialog"
                    aria-modal="true"
                    tabindex="-1"
                >
                    {title.map(|t| view! { <h2 data-rs-drawer-title="">{t}</h2> })}
                    {description.map(|d| view! { <p data-rs-drawer-description="">{d}</p> })}
                    <button
                        type="button"
                        data-rs-drawer-close=""
                        data-rs-button=""
                        data-rs-variant="outline"
                        on:click=on_close
                    >
                        {close_label}
                    </button>
                </div>
            </div>
        </div>
    }
}
