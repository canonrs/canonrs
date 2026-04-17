//! Toolbar Interaction Engine — roving tabindex + action dispatch

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, query};

fn get_items(root: &Element) -> Vec<Element> {
    query::all(root, "[data-rs-toolbar-item]:not([disabled])")
}

fn dispatch_action(root: &Element, value: &str, pressed: bool) {
    use web_sys::CustomEventInit;
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &"value".into(), &value.into()).ok();
    js_sys::Reflect::set(&detail, &"pressed".into(), &pressed.into()).ok();
    let init = CustomEventInit::new();
    init.set_bubbles(true);
    init.set_detail(&detail);
    if let Ok(evt) = web_sys::CustomEvent::new_with_event_init_dict("canon:toolbar:action", &init) {
        let _ = root.dispatch_event(&evt);
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // data-rs-variant é o atributo correto no primitive
    let is_vertical = root.get_attribute("data-rs-variant").as_deref() == Some("vertical");

    // roving tabindex init — primeiro item recebe 0
    let items = get_items(&root);
    for (i, item) in items.iter().enumerate() {
        let _ = item.set_attribute("tabindex", if i == 0 { "0" } else { "-1" });
    }

    // keydown — roving tabindex
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-toolbar-item]").ok().flatten().is_none() { return; }

            let items = get_items(&root_cb);
            let len = items.len();
            if len == 0 { return; }

            let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));
            let Some(pos) = pos else { return };

            let next = match e.key().as_str() {
                "ArrowRight" if !is_vertical => { e.prevent_default(); Some((pos + 1) % len) }
                "ArrowLeft"  if !is_vertical => { e.prevent_default(); Some((pos + len - 1) % len) }
                "ArrowDown"  if  is_vertical => { e.prevent_default(); Some((pos + 1) % len) }
                "ArrowUp"    if  is_vertical => { e.prevent_default(); Some((pos + len - 1) % len) }
                "Home" => { e.prevent_default(); Some(0) }
                "End"  => { e.prevent_default(); Some(len - 1) }
                _ => None,
            };

            if let Some(next_idx) = next {
                for (i, item) in items.iter().enumerate() {
                    let _ = item.set_attribute("tabindex", if i == next_idx { "0" } else { "-1" });
                }
                if let Ok(el) = items[next_idx].clone().dyn_into::<web_sys::HtmlElement>() {
                    let _ = el.focus();
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click — toggle aria-pressed + dispatch CustomEvent via web_sys
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-toolbar-item]").ok().flatten() else { return };
            if item.has_attribute("disabled") { return; }

            let pressed = item.get_attribute("aria-pressed").as_deref() == Some("true");
            let next_pressed = !pressed;
            let _ = item.set_attribute("aria-pressed", if next_pressed { "true" } else { "false" });

            let value = item.get_attribute("data-rs-value").unwrap_or_default();
            dispatch_action(&root_cb, &value, next_pressed);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
