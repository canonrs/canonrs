use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NavMenuIslandLink {
    pub label: String,
    pub href:  String,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NavMenuIslandItem {
    pub trigger:  String,
    pub links:    Vec<NavMenuIslandLink>,
    pub href:     Option<String>,
}

#[island]
pub fn NavigationMenuIsland(
    items: Vec<NavMenuIslandItem>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let (active, set_active) = signal(Option::<usize>::None);
    let _ = set_active;
    let initial_state = "closed"; // CR-331 compliance

    #[cfg(feature = "hydrate")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys;

        // fechar ao clicar fora
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if active.get_untracked().is_some() { set_active.set(None); }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap()
            .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();

        // ESC fecha tudo
        let cb_esc = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" { set_active.set(None); }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap()
            .add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();
    }

    let items_view = items.iter().enumerate().map(|(idx, item)| {
        let trigger  = item.trigger.clone();
        let links    = item.links.clone();
        let href     = item.href.clone();
        let has_menu = !links.is_empty();

        let is_open  = move || active.get() == Some(idx);
        let is_open2 = move || active.get() == Some(idx);
        let state    = move || { let s = if is_open() { "open" } else { "closed" }; if s.is_empty() { initial_state } else { s } };
        let hidden   = move || !is_open2();

        #[cfg(feature = "hydrate")]
        let on_trigger = move |e: leptos::ev::MouseEvent| {
            e.stop_propagation();
            set_active.update(|v| {
                *v = if *v == Some(idx) { None } else { Some(idx) };
            });
        };
        #[cfg(not(feature = "hydrate"))]
        let on_trigger = move |_: leptos::ev::MouseEvent| {};

        let links_view = links.iter().map(|link| {
            let href  = link.href.clone();
            let label = link.label.clone();
            view! {
                <li data-rs-navigation-menu-sub-item="">
                    <a data-rs-navigation-menu-link="" href=href>{label}</a>
                </li>
            }
        }).collect::<Vec<_>>();

        view! {
            <li data-rs-navigation-menu-item="" data-rs-state=state>
                {if let Some(h) = href {
                    view! {
                        <a data-rs-navigation-menu-link="" href=h>{trigger}</a>
                    }.into_any()
                } else {
                    view! {
                        <button
                            type="button"
                            data-rs-navigation-menu-trigger=""
                            aria-expanded=move || is_open().to_string()
                            aria-haspopup="true"
                            on:click=on_trigger
                        >
                            {trigger}
                        </button>
                        {if has_menu {
                            view! {
                                <div
                                    data-rs-navigation-menu-content=""
                                    data-rs-state=move || if is_open2() { "open" } else { "closed" }
                                    hidden=hidden
                                >
                                    <ul>{links_view}</ul>
                                </div>
                            }.into_any()
                        } else {
                            view! { <></> }.into_any()
                        }}
                    }.into_any()
                }}
            </li>
        }
    }).collect::<Vec<_>>();

    view! {
        <nav
            data-rs-navigation-menu=""
            data-rs-component="NavigationMenu"
            class=class
        >
            <ul data-rs-navigation-menu-list="">
                {items_view}
            </ul>
        </nav>
    }
}
