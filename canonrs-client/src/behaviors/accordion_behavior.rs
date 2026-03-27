#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;

#[cfg(feature = "hydrate")]
thread_local! {
    static COUNTER: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

#[cfg(feature = "hydrate")]
fn next_id() -> String {
    COUNTER.with(|c| {
        let n = c.get() + 1;
        c.set(n);
        format!("rs-accordion-content-{}", n)
    })
}

#[cfg(feature = "hydrate")]
fn set_item_open(item: &web_sys::Element, trigger: &HtmlElement, content: &HtmlElement, open: bool) {
    if open {
        content.remove_attribute("hidden").ok();
        content.set_attribute("aria-hidden", "false").ok();
        trigger.set_attribute("aria-expanded", "true").ok();
        item.set_attribute("data-rs-state", "open").ok();
    } else {
        content.set_attribute("hidden", "").ok();
        content.set_attribute("aria-hidden", "true").ok();
        trigger.set_attribute("aria-expanded", "false").ok();
        item.set_attribute("data-rs-state", "closed").ok();
    }
}

#[cfg(feature = "hydrate")]
fn close_all(root: &web_sys::Element) {
    if let Ok(items) = root.query_selector_all("[data-rs-accordion-item]") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(item) = node.clone().dyn_into::<web_sys::Element>() {
                    if let Ok(Some(trig)) = item.query_selector("[data-rs-accordion-trigger]") {
                        if let Ok(trig_el) = trig.dyn_into::<HtmlElement>() {
                            if let Ok(Some(content)) = item.query_selector("[data-rs-accordion-content]") {
                                if let Ok(content_el) = content.dyn_into::<HtmlElement>() {
                                    set_item_open(&item, &trig_el, &content_el, false);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-accordion", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-accordion-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-accordion-attached", "1").ok();

        let is_single = root.get_attribute("data-rs-selection").as_deref() != Some("multiple");
        let collapsible = root.get_attribute("data-rs-collapsible").as_deref() != Some("false");

        // Fase 1: garantir id no content + aria-controls no trigger (via DOM)
        if let Ok(items) = root.query_selector_all("[data-rs-accordion-item]") {
            for i in 0..items.length() {
                if let Some(node) = items.item(i) {
                    if let Ok(item) = node.dyn_into::<web_sys::Element>() {
                        if let (Ok(Some(trig)), Ok(Some(content))) = (
                            item.query_selector("[data-rs-accordion-trigger]"),
                            item.query_selector("[data-rs-accordion-content]"),
                        ) {
                            if let Ok(content_el) = content.dyn_into::<HtmlElement>() {
                                let content_id = if content_el.id().is_empty() {
                                    let id = next_id();
                                    content_el.set_id(&id);
                                    id
                                } else {
                                    content_el.id()
                                };
                                trig.set_attribute("aria-controls", &content_id).ok();
                            }
                        }
                    }
                }
            }
        }

        // Fase 2: registrar eventos nos triggers
        let triggers = root.query_selector_all("[data-rs-accordion-trigger]")
            .map_err(|_| crate::BehaviorError::JsError { message: "query triggers".into() })?;

        for i in 0..triggers.length() {
            let node = match triggers.item(i) { Some(n) => n, None => continue };
            let trigger = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            if trigger.get_attribute("data-rs-accordion-trigger-initialized").is_some() { continue; }
            trigger.set_attribute("data-rs-accordion-trigger-initialized", "1").ok();

            let root_click = root.clone();
            let trigger_click = trigger.clone();

            let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
                let item = match trigger_click.closest("[data-rs-accordion-item]").ok().flatten() {
                    Some(el) => el,
                    None => return,
                };
                let content = match item
                    .query_selector("[data-rs-accordion-content]")
                    .ok().flatten()
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
                    Some(el) => el,
                    None => return,
                };
                let is_open = !content.has_attribute("hidden");
                if is_open {
                    if collapsible { set_item_open(&item, &trigger_click, &content, false); }
                } else {
                    if is_single { close_all(&root_click); }
                    set_item_open(&item, &trigger_click, &content, true);
                }
            }) as Box<dyn FnMut(_)>);

            trigger.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();

            let root_key = root.clone();
            let trigger_key = trigger.clone();

            let key_closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" | " " => { e.prevent_default(); trigger_key.click(); }
                    "ArrowDown" | "ArrowUp" | "Home" | "End" => {
                        e.prevent_default();
                        let win = match web_sys::window() { Some(w) => w, None => return };
                        let doc = match win.document() { Some(d) => d, None => return };
                        if let Ok(all) = root_key.query_selector_all("[data-rs-accordion-trigger]") {
                            let len = all.length();
                            if len == 0 { return; }
                            let mut cur: Option<u32> = None;
                            for j in 0..len {
                                if let Some(n) = all.item(j) {
                                    if let Ok(el) = n.dyn_into::<HtmlElement>() {
                                        if doc.active_element().map(|a| a == *el.as_ref()) == Some(true) {
                                            cur = Some(j); break;
                                        }
                                    }
                                }
                            }
                            let next = match e.key().as_str() {
                                "ArrowDown" => cur.map(|i| (i + 1) % len).unwrap_or(0),
                                "ArrowUp"   => cur.map(|i| (i + len - 1) % len).unwrap_or(len - 1),
                                "Home"      => 0,
                                "End"       => len - 1,
                                _           => return,
                            };
                            if let Some(n) = all.item(next) {
                                if let Ok(el) = n.dyn_into::<HtmlElement>() { el.focus().ok(); }
                            }
                        }
                    }
                    _ => {}
                }
            }) as Box<dyn FnMut(_)>);

            trigger.add_event_listener_with_callback("keydown", key_closure.as_ref().unchecked_ref()).ok();
            key_closure.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
