use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContextMenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

#[island]
pub fn ContextMenuIsland(
    children: Children,
    items: Vec<ContextMenuIslandItem>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let (is_open, set_open) = signal(false);
    let _ = set_open;
    let (pos_x, set_x) = signal(0i32);
    let _ = set_x;
    let (pos_y, set_y) = signal(0i32);
    let _ = set_y;

    let state         = move || if is_open.get() { "open" } else { "closed" };
    let initial_state = "closed";
    let content_hidden = move || !is_open.get();
    let content_state  = move || if is_open.get() { "open" } else { "closed" };
    let left_style = move || format!("left:{}px;top:{}px;", pos_x.get(), pos_y.get());

    #[cfg(feature = "hydrate")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys;

        // click-outside fecha
        let cb_close = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if is_open.get_untracked() { set_open.set(false); }
        }) as Box<dyn FnMut(_)>);
        let doc = web_sys::window().unwrap().document().unwrap();
        doc.add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).ok();
        cb_close.forget();
    }

    #[cfg(feature = "hydrate")]
    let on_contextmenu = move |e: leptos::ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_x.set(e.client_x());
        set_y.set(e.client_y());
        set_open.set(true);
    };
    #[cfg(not(feature = "hydrate"))]
    let on_contextmenu = move |_: leptos::ev::MouseEvent| {};

    let items_view = items.iter().map(|item| {
        let disabled = item.disabled;
        let label    = item.label.clone();
        #[cfg(feature = "hydrate")]
        let on_click = move |_: leptos::ev::MouseEvent| {
            if !disabled { set_open.set(false); }
        };
        #[cfg(not(feature = "hydrate"))]
        let on_click = move |_: leptos::ev::MouseEvent| {};

        view! {
            <div
                data-rs-context-menu-item=""
                role="menuitem"
                aria-disabled=disabled.to_string()
                on:click=on_click
            >
                {label}
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-context-menu=""
            data-rs-component="ContextMenu"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            class=class
            on:contextmenu=on_contextmenu
        >
            <div data-rs-context-menu-trigger="">
                {children()}
            </div>
            <div
                data-rs-context-menu-content=""
                data-rs-state=content_state
                hidden=content_hidden
                role="menu"
                style=left_style
            >
                {items_view}
            </div>
        </div>
    }
}
