use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenubarIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenubarIslandMenu {
    pub trigger: String,
    pub items:   Vec<MenubarIslandItem>,
}

#[island]
pub fn MenubarIsland(
    menus: Vec<MenubarIslandMenu>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    // active_menu: index do menu aberto, None = todos fechados
    let (active_menu, set_active) = signal(Option::<usize>::None);
    let _ = set_active;
    let initial_state = "closed";

    #[cfg(feature = "hydrate")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys;

        // fechar ao clicar fora
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if active_menu.get_untracked().is_some() {
                set_active.set(None);
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap()
            .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();

        // fechar ao scroll
        let cb_scroll = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if active_menu.get_untracked().is_some() {
                set_active.set(None);
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap()
            .add_event_listener_with_callback("scroll", cb_scroll.as_ref().unchecked_ref()).ok();
        cb_scroll.forget();
    }

    let menus_view = menus.iter().enumerate().map(|(idx, menu)| {
        let trigger = menu.trigger.clone();
        let items   = menu.items.clone();
        let is_open  = move || active_menu.get() == Some(idx);
        let is_open2 = move || active_menu.get() == Some(idx);
        let is_open3 = move || active_menu.get() == Some(idx);
        let state    = move || if is_open() { "open" } else { "closed" };
        let hidden   = move || !is_open2();
        let (pos_x, set_px) = signal(0f64);
        let (pos_y, set_py) = signal(0f64);
        let _ = set_px; let _ = set_py;
        let trigger_ref = NodeRef::<leptos::html::Button>::new();
        let content_style = move || format!("position:fixed;left:{:.0}px;top:{:.0}px;", pos_x.get(), pos_y.get());

        #[cfg(feature = "hydrate")]
        let on_trigger = move |e: leptos::ev::MouseEvent| {
            e.stop_propagation();
            set_active.update(|v| {
                *v = if *v == Some(idx) { None } else { Some(idx) };
            });
            if let Some(el) = trigger_ref.get() {
                use leptos::wasm_bindgen::JsCast;
                let el = el.unchecked_into::<leptos::web_sys::HtmlElement>();
                let rect = el.get_bounding_client_rect();
                set_px.set(rect.left());
                set_py.set(rect.bottom() + 4.0);
            }
        };
        #[cfg(not(feature = "hydrate"))]
        let on_trigger = move |_: leptos::ev::MouseEvent| {};

        let items_view = items.iter().map(|item| {
            let disabled = item.disabled;
            let label    = item.label.clone();
            #[cfg(feature = "hydrate")]
            let on_item = move |_: leptos::ev::MouseEvent| {
                if !disabled { set_active.set(None); }
            };
            #[cfg(not(feature = "hydrate"))]
            let on_item = move |_: leptos::ev::MouseEvent| {};

            view! {
                <div
                    data-rs-menubar-item=""
                    role="menuitem"
                    aria-disabled=disabled.to_string()
                    on:click=on_item
                >
                    {label}
                </div>
            }
        }).collect::<Vec<_>>();

        view! {
            <div
                data-rs-menubar-menu=""
                data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            >
                <button
                    type="button"
                    data-rs-menubar-trigger=""
                    aria-expanded=move || is_open().to_string()
                    aria-haspopup="true"
                    node_ref=trigger_ref
                    on:click=on_trigger
                >
                    {trigger}
                </button>
                <div
                    data-rs-menubar-content=""
                    data-rs-state=move || if is_open2() { "open" } else { "closed" }
                    hidden=hidden
                    role="menu"
                    style=move || if is_open3() { content_style() } else { String::new() }
                >
                    {items_view}
                </div>
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-menubar=""
            data-rs-component="Menubar"
            role="menubar"
            class=class
        >
            {menus_view}
        </div>
    }
}
