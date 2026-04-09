//! Query — selecao segura de elementos e targets
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};

pub fn safe_target(e: &MouseEvent) -> Option<Element> {
    e.target()?.dyn_ref::<Element>().cloned()
}

pub fn safe_current(e: &MouseEvent) -> Option<Element> {
    e.current_target()?.dyn_into::<Element>().ok()
}

pub fn closest(el: &Element, selector: &str) -> bool {
    el.closest(selector).ok().flatten().is_some()
}

pub fn each<F: Fn(Element)>(selector: &str, f: F) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(nodes) = doc.query_selector_all(selector) else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(node)  = raw.dyn_into::<Element>() else { continue };
        f(node);
    }
}
