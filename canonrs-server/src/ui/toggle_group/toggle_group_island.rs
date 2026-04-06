//! @canon-level: strict
//! ToggleGroup Island

use leptos::prelude::*;
use super::toggle_group_ui::ToggleGroup;

fn add_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let next = format!("{} {}", current, token).trim().to_string();
    let _ = el.set_attribute("data-rs-state", &next);
}

fn remove_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

fn get_toggles(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    let mut result = Vec::new();
    let mut i = 0u32;
    loop {
        let selector = format!("[data-rs-toggle]:nth-of-type({})", i + 1);
        match root.query_selector(&selector) {
            Ok(Some(el)) => { result.push(el); i += 1; }
            _ => break,
        }
    }
    result
}

fn is_on(el: &leptos::web_sys::Element) -> bool {
    el.get_attribute("data-rs-state").map(|s: String| s.contains("on")).unwrap_or(false)
}

fn is_disabled_el(el: &leptos::web_sys::Element) -> bool {
    el.get_attribute("data-rs-state").map(|s: String| s.contains("disabled")).unwrap_or(false)
}

fn is_group_disabled(root: &leptos::web_sys::Element) -> bool {
    root.get_attribute("data-rs-state").map(|s: String| s.contains("disabled")).unwrap_or(false)
}

fn is_multiple(root: &leptos::web_sys::Element) -> bool {
    root.get_attribute("data-rs-multiple").map(|s: String| s == "true").unwrap_or(false)
}

fn toggle_item(root: &leptos::web_sys::Element, item: &leptos::web_sys::Element) {
    let multiple = is_multiple(root);
    let currently_on = is_on(item);
    if !multiple {
        for toggle in get_toggles(root) {
            remove_state(&toggle, "on");
            add_state(&toggle, "off");
            let _ = toggle.set_attribute("aria-pressed", "false");
        }
        if !currently_on {
            remove_state(item, "off");
            add_state(item, "on");
            let _ = item.set_attribute("aria-pressed", "true");
        }
    } else {
        if currently_on {
            remove_state(item, "on");
            add_state(item, "off");
            let _ = item.set_attribute("aria-pressed", "false");
        } else {
            remove_state(item, "off");
            add_state(item, "on");
            let _ = item.set_attribute("aria-pressed", "true");
        }
    }
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToggleGroupItem {
    pub value:    String,
    pub label:    String,
    pub on:       bool,
    pub disabled: bool,
}

static TOGGLE_GROUP_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

#[island]
pub fn ToggleGroupIsland(
    items: Vec<ToggleGroupItem>,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let multiple  = multiple.unwrap_or(false);
    let disabled  = disabled.unwrap_or(false);
    let class     = class.unwrap_or_default();
    let island_id = TOGGLE_GROUP_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed).to_string();
    let node_ref  = NodeRef::<leptos::html::Div>::new();
    let id_for_effect = island_id.clone();
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;
        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        let _ = root.set_attribute("data-rs-toggle-group-id", &id_for_effect);
        if disabled {
            add_state(&root, "disabled");
            for toggle in get_toggles(&root) { add_state(&toggle, "disabled"); }
        }
        // click
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if let Ok(Some(item)) = target.closest("[data-rs-toggle]") {
                    if is_group_disabled(&root_cb) { return; }
                    if is_disabled_el(&item) { return; }
                    e.stop_propagation();
                    toggle_item(&root_cb, &item);
                }
            }));
            let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // keydown
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if target.closest("[data-rs-toggle]").ok().flatten().is_none() { return; }
                if is_group_disabled(&root_cb) { return; }
                let key = e.key();
                match key.as_str() {
                    " " | "Enter" => {
                        e.prevent_default();
                        if let Ok(Some(item)) = target.closest("[data-rs-toggle]") {
                            if !is_disabled_el(&item) { toggle_item(&root_cb, &item); }
                        }
                    }
                    "ArrowRight" | "ArrowDown" => {
                        e.prevent_default();
                        let items = get_toggles(&root_cb);
                        let len = items.len();
                        if let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) {
                            let next = (pos + 1) % len;
                            if let Ok(btn) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = btn.focus(); }
                        }
                    }
                    "ArrowLeft" | "ArrowUp" => {
                        e.prevent_default();
                        let items = get_toggles(&root_cb);
                        let len = items.len();
                        if let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) {
                            let prev = if pos == 0 { len - 1 } else { pos - 1 };
                            if let Ok(btn) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = btn.focus(); }
                        }
                    }
                    _ => {}
                }
            }));
            let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });
    let items_view = items.into_iter().map(|item| {
        let base = if item.on { "on" } else { "off" };
        let state = if item.disabled { format!("{} disabled", base) } else { base.to_string() };
        let aria_pressed = if item.on { "true" } else { "false" };
        view! {
            <button
                type="button"
                data-rs-toggle=""
                data-rs-component="Toggle"
                data-rs-value=item.value
                data-rs-state=state
                aria-pressed=aria_pressed
                role="button"
            >
                {item.label}
            </button>
        }
    }).collect::<Vec<_>>();
    view! {
        <ToggleGroup multiple=multiple class=class node_ref=node_ref>
            {items_view}
        </ToggleGroup>
    }
}
