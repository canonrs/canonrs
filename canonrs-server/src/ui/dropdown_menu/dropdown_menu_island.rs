use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropdownMenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
    pub checked:  bool,
}

#[derive(Clone, Copy)]
pub struct DropdownMenuContext {
    pub is_open:  ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
}

#[island]
pub fn DropdownMenuIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let (is_open, set_open) = signal(false);

    provide_context(DropdownMenuContext { is_open, set_open });

    let state        = move || if is_open.get() { "open" } else { "closed" };
    let initial_state = "closed";

    #[cfg(feature = "hydrate")]
    {
        use leptos::prelude::Effect;
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys;

        Effect::new(move |_| {
            if !is_open.get() { return; }
            let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
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
        });
    }

    view! {
        <div
            data-rs-dropdown-menu=""
            data-rs-component="DropdownMenu"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            class=class
        >
            {children()}
        </div>
    }
}

#[island]
pub fn DropdownMenuTriggerIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let ctx   = use_context::<DropdownMenuContext>();
    let is_expanded = move || ctx.map(|c| c.is_open.get()).unwrap_or(false);

    #[cfg(feature = "hydrate")]
    let on_click = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        if let Some(c) = ctx { c.set_open.update(|v| *v = !*v); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_click = move |_: leptos::ev::MouseEvent| {};

    view! {
        <button
            type="button"
            data-rs-dropdown-menu-trigger=""
            aria-expanded=move || is_expanded().to_string()
            aria-haspopup="true"
            class=class
            on:click=on_click
        >
            {children()}
        </button>
    }
}

#[island]
pub fn DropdownMenuContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class   = class.unwrap_or_default();
    let ctx     = use_context::<DropdownMenuContext>();
    let is_open = move || ctx.map(|c| c.is_open.get()).unwrap_or(false);
    let is_open2 = move || ctx.map(|c| c.is_open.get()).unwrap_or(false);
    let state   = move || if is_open()  { "open" } else { "closed" };
    let hidden  = move || !is_open2();

    view! {
        <div
            data-rs-dropdown-menu-content=""
            data-rs-state=state
            hidden=hidden
            role="menu"
            class=class
        >
            {children()}
        </div>
    }
}

#[island]
pub fn DropdownMenuItemIsland(
    children: Children,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class    = class.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let _ctx = use_context::<DropdownMenuContext>();

    #[cfg(feature = "hydrate")]
    let on_click = move |_: leptos::ev::MouseEvent| {
        if disabled { return; }
        if let Some(c) = _ctx { c.set_open.set(false); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_click = move |_: leptos::ev::MouseEvent| {};

    view! {
        <div
            data-rs-dropdown-menu-item=""
            role="menuitem"
            aria-disabled=disabled.to_string()
            class=class
            on:click=on_click
        >
            {children()}
        </div>
    }
}
