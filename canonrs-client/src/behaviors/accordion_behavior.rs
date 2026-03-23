#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;

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
                    if let Ok(Some(trig)) = item.query_selector("[data-rs-trigger]") {
                        if let Ok(trig_el) = trig.dyn_into::<HtmlElement>() {
                            if let Some(cid) = trig_el.get_attribute("aria-controls") {
                                if let Some(content_el) = root
                                    .query_selector(&format!("#{}", cid))
                                    .ok().flatten()
                                    .and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
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

        let triggers = root.query_selector_all("[data-rs-trigger]")
            .map_err(|_| canonrs_core::BehaviorError::JsError { message: "query triggers".into() })?;

        for i in 0..triggers.length() {
            let node = match triggers.item(i) { Some(n) => n, None => continue };
            let trigger = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            if trigger.get_attribute("data-rs-trigger-initialized").is_some() { continue; }
            trigger.set_attribute("data-rs-trigger-initialized", "1").ok();

            let is_open_init = trigger.get_attribute("aria-expanded").as_deref() == Some("true");
            if let Some(item) = trigger.closest("[data-rs-accordion-item]").ok().flatten() {
                item.set_attribute("data-rs-state", if is_open_init { "open" } else { "closed" }).ok();
            }

            let root_click = root.clone();
            let trigger_click = trigger.clone();

            let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
                let controls_id = match trigger_click.get_attribute("aria-controls") {
                    Some(id) if !id.is_empty() => id,
                    _ => return,
                };
                let content = match root_click
                    .query_selector(&format!("#{}", controls_id))
                    .ok().flatten()
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
                    Some(el) => el,
                    None => return,
                };
                let item = match trigger_click.closest("[data-rs-accordion-item]").ok().flatten() {
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
                    "ArrowDown" | "ArrowUp" => {
                        e.prevent_default();
                        let win = match web_sys::window() { Some(w) => w, None => return };
                        let doc = match win.document() { Some(d) => d, None => return };
                        if let Ok(all) = root_key.query_selector_all("[data-rs-trigger]") {
                            let len = all.length();
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
                            if let Some(idx) = cur {
                                let next = if e.key() == "ArrowDown" { (idx + 1) % len } else { (idx + len - 1) % len };
                                if let Some(n) = all.item(next) {
                                    if let Ok(el) = n.dyn_into::<HtmlElement>() { el.focus().ok(); }
                                }
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
