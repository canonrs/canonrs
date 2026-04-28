//! Context — propaga UID do root para filhos diretos com data-rs-*
//! Fast path: closest() — O(depth)
//! Fallback: data-rs-owner -> data-rs-uid

use web_sys::Element;
use wasm_bindgen::JsCast;

/// Propaga data-rs-owner apenas para filhos com atributos data-rs-*
/// Evita O(n) global com query_selector_all("*")
pub fn propagate_owner(root: &Element) {
    let Some(uid) = root.get_attribute("data-rs-uid") else { return };
    let Ok(nodes) = root.query_selector_all("[data-rs-uid], [data-rs-interaction], [data-rs-state]") else { return };
    for i in 0..nodes.length() {
        if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
            // nao sobrescrever owner de subcomponentes que ja tem seu proprio uid
            if el.get_attribute("data-rs-uid").is_none() {
                let _ = el.set_attribute("data-rs-owner", &uid);
            }
        }
    }
}

pub fn find_root(target: &Element, root_selector: &str) -> Option<Element> {
    // Fast path: sobe via closest() — O(depth)
    if let Ok(Some(el)) = target.closest(root_selector) {
        return Some(el);
    }
    // Fallback: data-rs-owner -> data-rs-uid
    let uid = target.get_attribute("data-rs-owner")?;
    let doc = web_sys::window().and_then(|w| w.document())?;
    doc.query_selector(&format!("[data-rs-uid=\'{}\']", uid)).ok().flatten()
}
