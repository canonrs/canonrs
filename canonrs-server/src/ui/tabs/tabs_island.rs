//! Tabs Island — Canon Rule #340
//! TabsRootIsland é o único island real. Trigger e Content são passthrough.

use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, prelude::Closure};
type Element = leptos::web_sys::Element;
type HtmlElement = leptos::web_sys::HtmlElement;
type MouseEvent = leptos::web_sys::MouseEvent;
type KeyboardEvent = leptos::web_sys::KeyboardEvent;
use super::tabs_ui::{Tabs, TabsTrigger, TabsContent};
use canonrs_core::meta::{ActivityState, DisabledState};

fn add_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let next = format!("{} {}", current, token).trim().to_string();
    let _ = el.set_attribute("data-rs-state", &next);
}

fn remove_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

fn query_all(_root: &Element, _selector: &str) -> Vec<Element> {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::Element as WsElement;
        let root_ws: &WsElement = _root.unchecked_ref();
        let Ok(nodes) = root_ws.query_selector_all(_selector) else { return vec![] };
        return (0..nodes.length())
            .filter_map(|i| nodes.item(i))
            .filter_map(|n| n.dyn_into::<Element>().ok())
            .collect();
    }
    #[cfg(not(feature = "hydrate"))]
    vec![]
}

fn dispatch_tab_activated() {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::CustomEventInit;
        if let Some(win) = leptos::web_sys::window() {
            if let Some(doc) = win.document() {
                let init = CustomEventInit::new();
                init.set_bubbles(true);
                if let Ok(evt) = web_sys::CustomEvent::new_with_event_init_dict("canon:tab-activated", &init) {
                    let _ = doc.dispatch_event(&evt);
                }
            }
        }
    }
}

fn get_triggers(root: &Element) -> Vec<Element> { query_all(root, "[data-rs-tabs-trigger]") }
fn get_contents(root: &Element) -> Vec<Element> { query_all(root, "[data-rs-tabs-content]") }

fn activate_tab(root: &Element, value: &str) {
    for trigger in get_triggers(root) {
        let v = trigger.get_attribute("data-rs-value").unwrap_or_default();
        let is_active = v == value;
        remove_state(&trigger, "active");
        remove_state(&trigger, "inactive");
        if is_active { add_state(&trigger, "active"); } else { add_state(&trigger, "inactive"); }
        let _ = trigger.set_attribute("aria-selected", if is_active { "true" } else { "false" });
    }

    for content in get_contents(root) {
        let v = content.get_attribute("data-rs-value").unwrap_or_default();
        let is_active = v == value;
        remove_state(&content, "active");
        remove_state(&content, "inactive");
        if let Ok(el) = content.clone().dyn_into::<HtmlElement>() {
            if is_active {
                add_state(&content, "active");
                let _ = el.remove_attribute("hidden");
            } else {
                add_state(&content, "inactive");
                let _ = el.set_attribute("hidden", "");
            }
        }
    }


    dispatch_tab_activated();
}

#[island]
pub fn TabsRootIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let Some(root_html) = node_ref.get() else { return };
        let root: Element = (*root_html).clone().unchecked_into();

        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |e: MouseEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                if let Ok(Some(trigger)) = target.closest("[data-rs-tabs-trigger]") {
                    let state = trigger.get_attribute("data-rs-state").unwrap_or_default();
                    if state.contains("disabled") { return; }
                    let value = trigger.get_attribute("data-rs-value").unwrap_or_default();
                    activate_tab(&root_cb, &value);
                }
            }));
            let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |e: KeyboardEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
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
                            }
                        }
                    }
                    "ArrowRight" | "ArrowDown" | "ArrowLeft" | "ArrowUp" => {
                        e.prevent_default();
                        let items: Vec<_> = get_triggers(&root_cb).into_iter()
                            .filter(|el| !el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false))
                            .collect();
                        let len = items.len();
                        if len == 0 { return; }
                        let cur = items.iter().position(|el| el.get_attribute("data-rs-state").map(|s| s.contains("active")).unwrap_or(false)).unwrap_or(0);
                        let next = match key.as_str() {
                            "ArrowRight" | "ArrowDown" => (cur + 1) % len,
                            _ => if cur == 0 { len - 1 } else { cur - 1 },
                        };
                        if let Some(el) = items.get(next) {
                            if let Ok(btn) = el.clone().dyn_into::<HtmlElement>() { let _ = btn.focus(); }
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

#[component]
pub fn TabsTriggerIsland(
    children: Children,
    #[prop(into)] value: String,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let dis = if disabled.unwrap_or(false) { DisabledState::Disabled } else { DisabledState::Enabled };
    view! { <TabsTrigger value=value active=ActivityState::Inactive disabled=dis>{children()}</TabsTrigger> }
}

#[component]
pub fn TabsContentIsland(
    children: Children,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! { <TabsContent value=value active=ActivityState::Inactive>{children()}</TabsContent> }
}
