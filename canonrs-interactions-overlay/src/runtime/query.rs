//! Query — selecao segura de elementos e targets
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};

pub fn safe_target(e: &MouseEvent) -> Option<Element> {
    let el = e.target()?.dyn_ref::<Element>()?.clone();
    if !el.is_connected() { return None; }
    Some(el)
}

pub fn safe_current(e: &MouseEvent) -> Option<Element> {
    let el = e.current_target()?.dyn_into::<Element>().ok()?;
    if !el.is_connected() { return None; }
    Some(el)
}

pub fn closest(el: &Element, selector: &str) -> bool {
    el.closest(selector).ok().flatten().is_some()
}

/// Busca o root de um overlay pelo atributo e uid — resiste a re-render.
/// Uso: query::root_of("data-rs-dialog", &uid)
pub fn root_of(attr: &str, uid: &str) -> Option<Element> {
    let doc = web_sys::window().and_then(|w| w.document())?;
    doc.query_selector(&format!("[{}][data-rs-uid='{}']", attr, uid)).ok().flatten()
}

pub fn each<F: Fn(Element)>(selector: &str, f: F) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(nodes) = doc.query_selector_all(selector) else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(node)  = raw.dyn_into::<Element>() else { continue };
        if !node.is_connected() { continue };
        f(node);
    }
}
