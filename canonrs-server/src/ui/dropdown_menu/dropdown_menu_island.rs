use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropdownMenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

#[island]
pub fn DropdownMenuIsland(
    items: Vec<DropdownMenuIslandItem>,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Options".to_string());
    let (is_open, set_open) = signal(false);
    let _ = set_open; // usado em closures cfg(hydrate)

    let state        = move || if is_open.get() { "open" } else { "closed" };
    let initial_state = "closed";
    let is_expanded  = move || is_open.get();
    let content_hidden = move || !is_open.get();
    let content_state  = move || if is_open.get() { "open" } else { "closed" };

    #[cfg(feature = "hydrate")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys;

        let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if !is_open.get_untracked() { return; }
            if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                if target.closest("[data-rs-dropdown-menu]").ok().flatten().is_none() {
                    set_open.set(false);
                }
            }
        }) as Box<dyn FnMut(_)>);
        let window = web_sys::window().unwrap();
        let doc = window.document().unwrap();
        doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    #[cfg(feature = "hydrate")]
    let on_trigger_click = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        set_open.update(|v| *v = !*v);
    };
    #[cfg(not(feature = "hydrate"))]
    let on_trigger_click = move |_: leptos::ev::MouseEvent| {};

    let items_view = items.iter().map(|item| {
        let disabled = item.disabled;
        let label    = item.label.clone();
        #[cfg(feature = "hydrate")]
        let on_item_click = move |_: leptos::ev::MouseEvent| {
            if !disabled { set_open.set(false); }
        };
        #[cfg(not(feature = "hydrate"))]
        let on_item_click = move |_: leptos::ev::MouseEvent| {};

        view! {
            <div
                data-rs-dropdown-menu-item=""
                role="menuitem"
                aria-disabled=disabled.to_string()
                on:click=on_item_click
            >
                {label}
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-dropdown-menu=""
            data-rs-component="DropdownMenu"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            class=class
        >
            <button
                type="button"
                data-rs-dropdown-menu-trigger=""
                aria-expanded=move || is_expanded().to_string()
                aria-haspopup="true"
                on:click=on_trigger_click
            >
                {trigger_label}
            </button>
            <div
                data-rs-dropdown-menu-content=""
                data-rs-state=content_state
                hidden=content_hidden
                role="menu"
            >
                {items_view}
            </div>
        </div>
    }
}
