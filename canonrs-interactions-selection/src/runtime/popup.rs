//! Popup -- click outside handler tier 1
//! Um único listener global é registrado por selector (não por interaction)

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{state, context};
use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    static REGISTERED_SELECTORS: RefCell<HashSet<&'static str>> = RefCell::new(HashSet::new());
}

pub fn register_click_outside(root_selector: &'static str, close_fn: fn(&Element)) {
    let already = REGISTERED_SELECTORS.with(|r| {
        let mut set = r.borrow_mut();
        if set.contains(root_selector) { true } else { set.insert(root_selector); false }
    });
    if already { return; }

    let selector = root_selector.to_string();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if context::find_root(&target, &selector).is_some() { return; }
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
        let Ok(nodes) = doc.query_selector_all(&selector) else { return };
        for i in 0..nodes.length() {
            if let Some(root) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                if state::has(&root, "open") { close_fn(&root); }
            }
        }
    }));
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
    }
    cb.forget();
}
