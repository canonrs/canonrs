//! NavigationMenu Island — Canon Rule #341
//! DOM-driven, zero state. Lógica via web_sys + Effect.

use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NavMenuIslandLink {
    pub label: String,
    pub href:  String,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NavMenuIslandItem {
    pub trigger: String,
    pub links:   Vec<NavMenuIslandLink>,
    pub href:    Option<String>,
}

#[island]
pub fn NavigationMenuIsland(
    items: Vec<NavMenuIslandItem>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    let node_ref = NodeRef::<leptos::html::Nav>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::HtmlElement = (*root_html).clone().unchecked_into();

        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        // click trigger → toggle content
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                e.stop_propagation();
                let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                let Some(trigger) = target.closest("[data-rs-navigation-menu-trigger]").ok().flatten() else { return };
                let Some(item) = trigger.closest("[data-rs-navigation-menu-item]").ok().flatten() else { return };
                let content = item.query_selector("[data-rs-navigation-menu-content]").ok().flatten();
                let is_open = item.get_attribute("data-rs-state").as_deref() == Some("open");

                // close all
                let mut idx = 0u32;
                loop {
                    let sel = format!("[data-rs-navigation-menu-item]:nth-of-type({})", idx + 1);
                    match root_cb.query_selector(&sel) {
                        Ok(Some(el)) => {
                            let _ = el.set_attribute("data-rs-state", "closed");
                            if let Ok(Some(c)) = el.query_selector("[data-rs-navigation-menu-content]") {
                                if let Ok(h) = c.dyn_into::<web_sys::HtmlElement>() { h.set_hidden(true); }
                            }
                            if let Ok(Some(t)) = el.query_selector("[data-rs-navigation-menu-trigger]") {
                                let _ = t.set_attribute("aria-expanded", "false");
                            }
                            idx += 1;
                        }
                        _ => break,
                    }
                }

                // open clicked if was closed
                if !is_open {
                    let _ = item.set_attribute("data-rs-state", "open");
                    let _ = trigger.set_attribute("aria-expanded", "true");
                    if let Some(c) = content {
                        let _ = c.set_attribute("data-rs-state", "open");
                        if let Ok(h) = c.dyn_into::<web_sys::HtmlElement>() { h.set_hidden(false); }
                    }
                }
            }));
            let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // click outside → close all
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if root_cb.contains(Some(&target)) { return; }
                let mut idx = 0u32;
                loop {
                    let sel = format!("[data-rs-navigation-menu-item]:nth-of-type({})", idx + 1);
                    match root_cb.query_selector(&sel) {
                        Ok(Some(el)) => {
                            let _ = el.set_attribute("data-rs-state", "closed");
                            if let Ok(Some(c)) = el.query_selector("[data-rs-navigation-menu-content]") {
                                if let Ok(h) = c.dyn_into::<web_sys::HtmlElement>() { h.set_hidden(true); }
                            }
                            idx += 1;
                        }
                        _ => break,
                    }
                }
            }));
            let win = match web_sys::window() { Some(w) => w, None => return };
            if let Some(doc) = win.document() {
                let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            }
            cb.forget();
        }

        // ESC → close all
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if e.key() != "Escape" { return; }
                let mut idx = 0u32;
                loop {
                    let sel = format!("[data-rs-navigation-menu-item]:nth-of-type({})", idx + 1);
                    match root_cb.query_selector(&sel) {
                        Ok(Some(el)) => {
                            let _ = el.set_attribute("data-rs-state", "closed");
                            if let Ok(Some(c)) = el.query_selector("[data-rs-navigation-menu-content]") {
                                if let Ok(h) = c.dyn_into::<web_sys::HtmlElement>() { h.set_hidden(true); }
                            }
                            idx += 1;
                        }
                        _ => break,
                    }
                }
            }));
            let win = match web_sys::window() { Some(w) => w, None => return };
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    let items_view = items.iter().map(|item| {
        let trigger  = item.trigger.clone();
        let links    = item.links.clone();
        let href     = item.href.clone();
        let has_menu = !links.is_empty();

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
            <li data-rs-navigation-menu-item="" data-rs-state="closed">
                {if let Some(h) = href {
                    view! {
                        <a data-rs-navigation-menu-link="" href=h>{trigger}</a>
                    }.into_any()
                } else {
                    view! {
                        <button
                            type="button"
                            data-rs-navigation-menu-trigger=""
                            aria-expanded="false"
                            aria-haspopup="true"
                        >
                            {trigger}
                        </button>
                        {if has_menu {
                            view! {
                                <div
                                    data-rs-navigation-menu-content=""
                                    data-rs-state="closed"
                                    hidden=true
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
            node_ref=node_ref
        >
            <ul data-rs-navigation-menu-list="">
                {items_view}
            </ul>
        </nav>
    }
}
