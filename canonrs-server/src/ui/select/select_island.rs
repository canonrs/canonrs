//! @canon-level: strict
//! Select Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;

use super::select_ui::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem};
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

fn get_items(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    let mut result = Vec::new();
    let mut i = 0u32;
    loop {
        let selector = format!("[data-rs-select-item]:nth-of-type({})", i + 1);
        match root.query_selector(&selector) {
            Ok(Some(el)) => { result.push(el); i += 1; }
            _ => break,
        }
    }
    result
}

fn is_disabled(root: &leptos::web_sys::Element) -> bool {
    root.get_attribute("data-rs-state")
        .map(|s: String| s.contains("disabled"))
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
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-select-trigger]") {
        if let Ok(el) = trigger.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.set_attribute("aria-expanded", if open { "true" } else { "false" });
        }
    }
}

fn is_open(root: &leptos::web_sys::Element) -> bool {
    root.get_attribute("data-rs-state")
        .map(|s: String| s.contains("open"))
        .unwrap_or(false)
}

fn set_selected(root: &leptos::web_sys::Element, value: &str) {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys;
    // Atualiza estado de cada item baseado no data-rs-value do próprio DOM
    for item in get_items(root) {
        let matches = item.get_attribute("data-rs-value")
            .map(|v: String| v == value)
            .unwrap_or(false);
        remove_state(&item, "selected");
        remove_state(&item, "unselected");
        if matches {
            add_state(&item, "selected");
            let _ = item.set_attribute("aria-selected", "true");
        } else {
            add_state(&item, "unselected");
            let _ = item.set_attribute("aria-selected", "false");
        }
    }
    // Atualiza label: lê text_content do item selecionado no DOM
    if let Ok(Some(span)) = root.query_selector("[data-rs-select-value]") {
        if let Ok(span_el) = span.dyn_into::<web_sys::HtmlElement>() {
            let selector = format!("[data-rs-select-item][data-rs-value='{}']", value);
            let label = root.query_selector(&selector)
                .ok().flatten()
                .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                .and_then(|n| n.text_content())
                .unwrap_or_else(|| span_el.get_attribute("data-rs-placeholder").unwrap_or_default());
            span_el.set_text_content(Some(&label));
        }
    }
}

fn clear_focused(root: &leptos::web_sys::Element) {
    for item in get_items(root) {
        remove_state(&item, "focus");
    }
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

fn focused_index(items: &[leptos::web_sys::Element]) -> Option<usize> {
    items.iter().position(|el| {
        el.get_attribute("data-rs-state")
            .map(|s: String| s.contains("focus"))
            .unwrap_or(false)
    })
}

fn set_focused_item(root: &leptos::web_sys::Element, items: &[leptos::web_sys::Element], idx: usize) {
    clear_focused(root);
    if let Some(el) = items.get(idx) {
        add_state(el, "focus");
    }
}

// ---------------------------------------------------------------------------
// Struct de opção
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

// ---------------------------------------------------------------------------
// Island
// ---------------------------------------------------------------------------

static SELECT_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

#[island]
pub fn SelectIsland(
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    options: Vec<SelectOption>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "Select...".to_string());
    let class       = class.unwrap_or_default();
    let island_id   = SELECT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed).to_string();

    let node_ref = NodeRef::<leptos::html::Div>::new();

    let id_for_effect = island_id.clone();
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        let _ = root.set_attribute("data-rs-select-id", &id_for_effect);

        // --- click (trigger + item) ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };

                if let Ok(Some(item)) = target.closest("[data-rs-select-item]") {
                    let state = item.get_attribute("data-rs-state").unwrap_or_default();
                    if state.contains("disabled") { return; }
                    e.stop_propagation();
                    let value = item.get_attribute("data-rs-value").unwrap_or_default();
                    set_selected(&root_cb, &value);
                    set_open(&root_cb, false);
                    clear_focused(&root_cb);
                    return;
                }

                if target.closest("[data-rs-select-trigger]").ok().flatten().is_some() {
                    e.stop_propagation();
                    if !is_disabled(&root_cb) {
                        let open = is_open(&root_cb);
                        set_open(&root_cb, !open);
                    }
                }
            }));
            let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // --- mouseover (focused) ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if let Ok(Some(item)) = target.closest("[data-rs-select-item]") {
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
                if is_disabled(&root_cb) { return; }
                let key = e.key();
                match key.as_str() {
                    "Escape" | "Tab" => {
                        set_open(&root_cb, false);
                        clear_focused(&root_cb);
                    }
                    " " | "Enter" => {
                        e.prevent_default();
                        if !is_open(&root_cb) {
                            set_open(&root_cb, true);
                        } else {
                            let items = navigable_items(&root_cb);
                            if let Some(idx) = focused_index(&items) {
                                if let Some(el) = items.get(idx) {
                                    let value = el.get_attribute("data-rs-value").unwrap_or_default();
                                    set_selected(&root_cb, &value);
                                    set_open(&root_cb, false);
                                    clear_focused(&root_cb);
                                }
                            }
                        }
                    }
                    "ArrowDown" | "ArrowUp" => {
                        e.prevent_default();
                        if !is_open(&root_cb) { set_open(&root_cb, true); }
                        let items = navigable_items(&root_cb);
                        let len = items.len();
                        if len == 0 { return; }
                        let cur  = focused_index(&items);
                        let next = match (key.as_str(), cur) {
                            ("ArrowDown", None)    => 0,
                            ("ArrowDown", Some(i)) => (i + 1).min(len - 1),
                            ("ArrowUp",   None)    => len - 1,
                            ("ArrowUp",   Some(i)) => if i == 0 { 0 } else { i - 1 },
                            _                      => 0,
                        };
                        set_focused_item(&root_cb, &items, next);
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
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            }
            cb.forget();
        }
    });

    // SSR — estrutura pura, estado vem do DOM
    let options_view = options.into_iter().map(|opt| {
        let item_disabled = if opt.disabled {
            DisabledState::Disabled
        } else {
            DisabledState::Enabled
        };
        view! {
            <SelectItem value=opt.value disabled=item_disabled>
                {opt.label}
            </SelectItem>
        }
    }).collect::<Vec<_>>();

    view! {
        <Select class=class node_ref=node_ref>
            <SelectTrigger>
                <SelectValue placeholder=placeholder>{""}</SelectValue>
            </SelectTrigger>
            <SelectContent>
                {options_view}
            </SelectContent>
        </Select>
    }
}
