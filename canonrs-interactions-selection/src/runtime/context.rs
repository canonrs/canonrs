//! Context -- propaga UID do root para todos os filhos
//! Padrao tier 1: cada filho conhece seu root via data-rs-owner

use web_sys::Element;
use wasm_bindgen::JsCast;

pub fn propagate_owner(root: &Element) {
    let Some(uid) = root.get_attribute("data-rs-uid") else { return };
    let Ok(nodes) = root.query_selector_all("*") else { return };
    for i in 0..nodes.length() {
        if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
            let _ = el.set_attribute("data-rs-owner", &uid);
        }
    }
}

pub fn find_root(target: &Element, root_selector: &str) -> Option<Element> {
    // Fast path: sobe via closest() — O(depth) em vez de O(n) global
    if let Ok(Some(el)) = target.closest(root_selector) {
        return Some(el);
    }
    // Fallback: busca por data-rs-owner → data-rs-uid
    let uid = target.get_attribute("data-rs-owner")?;
    let doc = web_sys::window().and_then(|w| w.document())?;
    let Ok(nodes) = doc.query_selector_all("[data-rs-uid]") else { return None };
    for i in 0..nodes.length() {
        if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
            if el.get_attribute("data-rs-uid").as_deref() == Some(uid.as_str()) {
                return Some(el);
            }
        }
    }
    None
}
