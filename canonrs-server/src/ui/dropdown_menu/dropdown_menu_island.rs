//! @canon-level: strict
//! DropdownMenu Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;

use super::dropdown_menu_ui::{
    DropdownMenu, DropdownMenuTrigger, DropdownMenuContent,
    DropdownMenuItem,
};
use canonrs_core::meta::DisabledState;

// ---------------------------------------------------------------------------
// Helpers de estado — multi-state safe
// ---------------------------------------------------------------------------

fn add_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let next = format!("{} {}", current, token).trim().to_string();
    let _ = el.set_attribute("data-rs-state", &next);
}

fn remove_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace()
        .filter(|t| *t != token)
        .collect::<Vec<_>>()
        .join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

// ---------------------------------------------------------------------------
// Helpers DOM
// ---------------------------------------------------------------------------

fn is_disabled(root: &leptos::web_sys::Element) -> bool {
    root.get_attribute("data-rs-state")
        .map(|s: String| s.contains("disabled"))
        .unwrap_or(false)
}

fn is_open(root: &leptos::web_sys::Element) -> bool {
    root.get_attribute("data-rs-state")
        .map(|s: String| s.contains("open"))
        .unwrap_or(false)
}

fn set_open(root: &leptos::web_sys::Element, open: bool) {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys;
    if open {
        remove_state(root, "closed");
        add_state(root, "open");
    } else {
        remove_state(root, "open");
        add_state(root, "closed");
    }
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-dropdown-menu-trigger]") {
        if let Ok(el) = trigger.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.set_attribute("aria-expanded", if open { "true" } else { "false" });
        }
    }
}

fn get_items(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    let mut result = Vec::new();
    let mut i = 0u32;
    loop {
        let selector = format!("[data-rs-dropdown-menu-item]:nth-of-type({})", i + 1);
        match root.query_selector(&selector) {
            Ok(Some(el)) => { result.push(el); i += 1; }
            _ => break,
        }
    }
    result
}

fn clear_focused(root: &leptos::web_sys::Element) {
    for item in get_items(root) {
        remove_state(&item, "focus");
    }
}

fn focused_index(items: &[leptos::web_sys::Element]) -> Option<usize> {
    items.iter().position(|el| {
        el.get_attribute("data-rs-state")
            .map(|s: String| s.contains("focus"))
            .unwrap_or(false)
    })
}

fn navigable_items(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    get_items(root).into_iter()
        .filter(|el| {
            !el.get_attribute("data-rs-state")
                .map(|s: String| s.contains("disabled"))
                .unwrap_or(false)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Struct de item
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropdownMenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

// ---------------------------------------------------------------------------
// Island
// ---------------------------------------------------------------------------

static DROPDOWN_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

#[island]
pub fn DropdownMenuIsland(
    items: Vec<DropdownMenuIslandItem>,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Options".to_string());
    let island_id     = DROPDOWN_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed).to_string();

    let node_ref = NodeRef::<leptos::html::Div>::new();

    let id_for_effect = island_id.clone();
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        let _ = root.set_attribute("data-rs-dropdown-id", &id_for_effect);

        // --- click trigger ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if target.closest("[data-rs-dropdown-menu-trigger]").ok().flatten().is_some() {
                    e.stop_propagation();
                    if !is_disabled(&root_cb) {
                        let open = is_open(&root_cb);
                        set_open(&root_cb, !open);
                        if !open { clear_focused(&root_cb); }
                    }
                    return;
                }
                // click em item
                if let Ok(Some(item)) = target.closest("[data-rs-dropdown-menu-item]") {
                    let state = item.get_attribute("data-rs-state").unwrap_or_default();
                    if state.contains("disabled") { return; }
                    e.stop_propagation();
                    set_open(&root_cb, false);
                    clear_focused(&root_cb);
                }
            }));
            let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // --- mouseover items (focus) ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if let Ok(Some(item)) = target.closest("[data-rs-dropdown-menu-item]") {
                    let state = item.get_attribute("data-rs-state").unwrap_or_default();
                    if state.contains("disabled") { return; }
                    clear_focused(&root_cb);
                    add_state(&item, "focus");
                }
            }));
            let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // --- keydown ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if !is_open(&root_cb) { return; }
                let key = e.key();
                match key.as_str() {
                    "Escape" | "Tab" => {
                        set_open(&root_cb, false);
                        clear_focused(&root_cb);
                    }
                    "Enter" | " " => {
                        e.prevent_default();
                        let items = navigable_items(&root_cb);
                        if let Some(idx) = focused_index(&items) {
                            if let Some(_el) = items.get(idx) {
                                set_open(&root_cb, false);
                                clear_focused(&root_cb);
                            }
                        }
                    }
                    "ArrowDown" => {
                        e.prevent_default();
                        let items = navigable_items(&root_cb);
                        let len = items.len();
                        if len == 0 { return; }
                        let cur  = focused_index(&items);
                        let next = match cur {
                            None    => 0,
                            Some(i) => (i + 1).min(len - 1),
                        };
                        clear_focused(&root_cb);
                        if let Some(el) = items.get(next) { add_state(el, "focus"); }
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        let items = navigable_items(&root_cb);
                        let len = items.len();
                        if len == 0 { return; }
                        let cur  = focused_index(&items);
                        let next = match cur {
                            None    => len - 1,
                            Some(i) => if i == 0 { 0 } else { i - 1 },
                        };
                        clear_focused(&root_cb);
                        if let Some(el) = items.get(next) { add_state(el, "focus"); }
                    }
                    _ => {}
                }
            }));
            let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // --- click fora fecha ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if root_cb.contains(Some(&target)) { return; }
                set_open(&root_cb, false);
                clear_focused(&root_cb);
            }));
            if let Some(doc) = leptos::web_sys::window().and_then(|w| w.document()) {
                let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            }
            cb.forget();
        }
    });

    // SSR — estrutura pura
    let items_view = items.into_iter().map(|item| {
        let disabled = if item.disabled {
            DisabledState::Disabled
        } else {
            DisabledState::Enabled
        };
        view! {
            <DropdownMenuItem disabled=disabled>
                {item.label}
            </DropdownMenuItem>
        }
    }).collect::<Vec<_>>();

    view! {
        <DropdownMenu class=class node_ref=node_ref>
            <DropdownMenuTrigger>{trigger_label}</DropdownMenuTrigger>
            <DropdownMenuContent>
                {items_view}
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
