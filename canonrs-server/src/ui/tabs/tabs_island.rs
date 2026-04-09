//! @canon-level: strict
//! Tabs Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;

use super::tabs_ui::{Tabs, TabsTrigger, TabsContent};
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

fn get_triggers(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    let mut result = Vec::new();
    let mut i = 0u32;
    loop {
        let selector = format!("[data-rs-tabs-trigger]:nth-of-type({})", i + 1);
        match root.query_selector(&selector) {
            Ok(Some(el)) => { result.push(el); i += 1; }
            _ => break,
        }
    }
    result
}

fn get_contents(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    // busca todos os values dos triggers e localiza contents pelo value
    let mut result = Vec::new();
    let triggers = get_triggers(root);
    for trigger in &triggers {
        let v = trigger.get_attribute("data-rs-value").unwrap_or_default();
        if v.is_empty() { continue; }
        let selector = format!("[data-rs-tabs-content][data-rs-value='{}']", v);
        if let Ok(Some(el)) = root.query_selector(&selector) {
            result.push(el);
        }
    }
    result
}

fn activate_tab(root: &leptos::web_sys::Element, value: &str) {

    for trigger in get_triggers(root) {
        let v = trigger.get_attribute("data-rs-value").unwrap_or_default();
        let is_active = v == value;
        remove_state(&trigger, "active");
        remove_state(&trigger, "inactive");
        if is_active { add_state(&trigger, "active"); } else { add_state(&trigger, "inactive"); }
        let _ = trigger.set_attribute("aria-selected", if is_active { "true" } else { "false" });
    }

    for content in get_contents(root) {
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;
        let v = content.get_attribute("data-rs-value").unwrap_or_default();
        let is_active = v == value;
        remove_state(&content, "active");
        remove_state(&content, "inactive");
        if is_active {
            add_state(&content, "active");
            if let Ok(el) = content.clone().dyn_into::<web_sys::HtmlElement>() {
                let _ = el.remove_attribute("hidden");
            }
        } else {
            add_state(&content, "inactive");
            if let Ok(el) = content.clone().dyn_into::<web_sys::HtmlElement>() {
                let _ = el.set_attribute("hidden", "");
            }
        }
    }
}

fn navigable_triggers(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    get_triggers(root).into_iter()
        .filter(|el| {
            !el.get_attribute("data-rs-state")
                .map(|s: String| s.contains("disabled"))
                .unwrap_or(false)
        })
        .collect()
}

fn focused_trigger_index(items: &[leptos::web_sys::Element]) -> Option<usize> {
    items.iter().position(|el| {
        el.get_attribute("data-rs-state")
            .map(|s: String| s.contains("focus"))
            .unwrap_or(false)
    })
}

fn active_trigger_index(items: &[leptos::web_sys::Element]) -> Option<usize> {
    items.iter().position(|el| {
        el.get_attribute("data-rs-state")
            .map(|s: String| s.contains("active"))
            .unwrap_or(false)
    })
}

fn clear_focused(root: &leptos::web_sys::Element) {
    for trigger in get_triggers(root) {
        remove_state(&trigger, "focus");
    }
}

// ---------------------------------------------------------------------------
// Islands composáveis
// ---------------------------------------------------------------------------

static TABS_ROOT_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

#[island]
pub fn TabsRootIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class     = class.unwrap_or_default();
    let island_id = TABS_ROOT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed).to_string();

    let node_ref = NodeRef::<leptos::html::Div>::new();

    let id_for_effect  = island_id.clone();
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        let _ = root.set_attribute("data-rs-tabs-id", &id_for_effect);

        // --- click trigger ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if let Ok(Some(trigger)) = target.closest("[data-rs-tabs-trigger]") {
                    let state = trigger.get_attribute("data-rs-state").unwrap_or_default();
                    if state.contains("disabled") { return; }
                    let value = trigger.get_attribute("data-rs-value").unwrap_or_default();
                    activate_tab(&root_cb, &value);
                    clear_focused(&root_cb);
                }
            }));
            let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // --- keydown ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if target.closest("[data-rs-tabs-trigger]").ok().flatten().is_none() { return; }
                let key = e.key();
                match key.as_str() {
                    "Enter" | " " => {
                        e.prevent_default();
                        if let Ok(Some(trigger)) = target.closest("[data-rs-tabs-trigger]") {
                            let state = trigger.get_attribute("data-rs-state").unwrap_or_default();
                            if !state.contains("disabled") {
                                let value = trigger.get_attribute("data-rs-value").unwrap_or_default();
                                activate_tab(&root_cb, &value);
                                clear_focused(&root_cb);
                            }
                        }
                    }
                    "ArrowRight" | "ArrowDown" | "ArrowLeft" | "ArrowUp" => {
                        e.prevent_default();
                        let items = navigable_triggers(&root_cb);
                        let len = items.len();
                        if len == 0 { return; }
                        let cur = focused_trigger_index(&items)
                            .or_else(|| active_trigger_index(&items))
                            .unwrap_or(0);
                        let next = match key.as_str() {
                            "ArrowRight" | "ArrowDown" => (cur + 1) % len,
                            "ArrowLeft"  | "ArrowUp"   => if cur == 0 { len - 1 } else { cur - 1 },
                            _ => cur,
                        };
                        clear_focused(&root_cb);
                        if let Some(el) = items.get(next) {
                            add_state(el, "focus");
                            if let Ok(btn) = el.clone().dyn_into::<web_sys::HtmlElement>() {
                                let _ = btn.focus();
                            }
                        }
                    }
                    _ => {}
                }
            }));
            let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    view! {
        <Tabs class=class node_ref=node_ref>
            {children()}
        </Tabs>
    }
}

#[island]
pub fn TabsTriggerIsland(
    children: Children,
    #[prop(into)] value: String,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let active   = canonrs_core::meta::ActivityState::Inactive;
    let dis      = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <TabsTrigger value=value active=active disabled=dis>
            {children()}
        </TabsTrigger>
    }
}

#[island]
pub fn TabsContentIsland(
    children: Children,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <TabsContent value=value active=canonrs_core::meta::ActivityState::Inactive>
            {children()}
        </TabsContent>
    }
}
