use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SidebarIslandItem {
    pub label:    String,
    pub href:     String,
    pub active:   bool,
    pub disabled: bool,
    pub group:    Option<String>,
}

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum SidebarIslandVariant {
    #[default]
    Default,
    Rail,
}

#[island]
pub fn SidebarIsland(
    items: Vec<SidebarIslandItem>,
    #[prop(optional)] variant: Option<SidebarIslandVariant>,
    #[prop(optional)] initial_open: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class   = class.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    let is_rail = variant == SidebarIslandVariant::Rail;
    let _ = is_rail;
    let (is_open, set_open) = signal(initial_open.unwrap_or(true));
    let _ = set_open;
    let (is_pinned, set_pinned) = signal(false);
    let _ = set_pinned;

    let initial_state = if initial_open.unwrap_or(true) { "expanded" } else { "collapsed" };
    let state = move || {
        if is_open.get() { "expanded" } else { "collapsed" }
    };

    #[cfg(feature = "hydrate")]
    {
        use leptos::web_sys;

        // Restore pin state from localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(val)) = storage.get_item("sidebar-pinned") {
                    if val == "true" {
                        set_pinned.set(true);
                        set_open.set(true);
                    }
                }
            }
        }
    }

    #[cfg(feature = "hydrate")]
    let on_toggle = move |_: leptos::ev::MouseEvent| {
        if is_pinned.get_untracked() { return; }
        set_open.update(|v| *v = !*v);
    };
    #[cfg(not(feature = "hydrate"))]
    let on_toggle = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_pin = move |_: leptos::ev::MouseEvent| {
        use leptos::web_sys;
        set_pinned.update(|v| {
            *v = !*v;
            if *v { set_open.set(true); }
        });
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("sidebar-pinned",
                    if is_pinned.get_untracked() { "true" } else { "false" });
            }
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_pin = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| {
        if is_rail { set_open.set(true); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| {
        if is_rail && !is_pinned.get_untracked() { set_open.set(false); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| {};

    let items_view = items.iter().map(|item| {
        let label    = item.label.clone();
        let href     = item.href.clone();
        let active   = item.active;
        let disabled = item.disabled;
        let state_item = if active { "active" } else if disabled { "disabled" } else { "inactive" };

        view! {
            <li data-rs-sidebar-menu-item="" data-rs-state=state_item>
                <a
                    data-rs-sidebar-menu-link=""
                    href=href
                    aria-current=if active { "page" } else { "false" }
                    aria-disabled=disabled.to_string()
                >
                    {label}
                </a>
            </li>
        }
    }).collect::<Vec<_>>();

    view! {
        <aside
            data-rs-sidebar=""
            data-rs-component="Sidebar"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            data-rs-pinned=move || is_pinned.get().to_string()
            class=class
            on:mouseenter=on_mouseenter
            on:mouseleave=on_mouseleave
        >
            <div data-rs-sidebar-header="">
                <button
                    type="button"
                    data-rs-sidebar-toggle=""
                    aria-label="Toggle sidebar"
                    on:click=on_toggle
                >
                    "☰"
                </button>
                <button
                    type="button"
                    data-rs-sidebar-pin-toggle=""
                    aria-label="Pin sidebar"
                    on:click=on_pin
                >
                    {move || if is_pinned.get() { "📌" } else { "📍" }}
                </button>
            </div>
            <div data-rs-sidebar-content="">
                <ul data-rs-sidebar-menu="">
                    {items_view}
                </ul>
            </div>
        </aside>
    }
}
