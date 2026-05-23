//! Subtree — safe subtree operations
//!
//! Helpers para operar em subtrees de forma segura:
//! - query com connected guard
//! - init de subtree dinamica (ex: portal, island, lazy)
//! - destroy de subtree

use wasm_bindgen::JsCast;
use web_sys::Element;

/// Retorna todos os elementos [data-rs-interaction] conectados dentro do root
pub fn interaction_roots(root: &Element) -> Vec<(Element, String)> {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction]") else { return vec![] };
    let mut result = vec![];
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(el)    = raw.dyn_into::<Element>() else { continue };
        if !el.is_connected() { continue }
        let Some(group) = el.get_attribute("data-rs-interaction") else { continue };
        result.push((el, group));
    }
    result
}

/// Verifica se subtree tem elementos nao inicializados
pub fn has_uninitialized(root: &Element) -> bool {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction]") else { return false };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(el)    = raw.dyn_into::<Element>() else { continue };
        if !el.has_attribute("data-rs-initialized") { return true; }
    }
    false
}

/// Move elemento para o body — operacao de portal SSR-safe
pub fn move_to_body(el: &Element) {
    let Some(doc)  = web_sys::window().and_then(|w| w.document()) else { return };
    let Some(body) = doc.body() else { return };
    if !body.contains(Some(el)) {
        let _ = body.append_child(el);
    }
}

/// Verifica se elemento esta dentro de um portal movido para o body
pub fn is_in_portal(el: &Element) -> bool {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            return body.contains(Some(el));
        }
    }
    false
}

/// Clona atributo data-rs-owner para todos os filhos de um portal
pub fn propagate_owner(portal: &Element, uid: &str) {
    let _ = portal.set_attribute("data-rs-owner", uid);
    let Ok(nodes) = portal.query_selector_all("*") else { return };
    for i in 0..nodes.length() {
        if let Some(raw) = nodes.item(i) {
            if let Ok(el) = raw.dyn_into::<Element>() {
                let _ = el.set_attribute("data-rs-owner", uid);
            }
        }
    }
}
