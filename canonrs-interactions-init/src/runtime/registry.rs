//! Registry — mapa selector → init_fn
//! Tier S: zero if-chains, extensível, isolado

use web_sys::Element;

pub type InitFn = fn(Element);

pub fn dispatch(el: &Element) {
    if el.has_attribute("data-rs-table")      { crate::table::init(el.clone()); return; }
    if el.has_attribute("data-rs-tooltip")    { crate::tooltip::init(el.clone()); return; }
    if el.has_attribute("data-rs-collapsible"){ crate::collapsible::init(el.clone()); return; }
    if el.has_attribute("data-rs-switch")     { crate::switch::init(el.clone()); return; }
    if el.has_attribute("data-rs-toggle")     { crate::toggle::init(el.clone()); return; }
    if el.has_attribute("data-rs-checkbox")   { crate::checkbox::init(el.clone()); return; }
}

pub fn scan_all() {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let selector = "[data-rs-table],[data-rs-tooltip],[data-rs-collapsible],[data-rs-switch],[data-rs-toggle],[data-rs-checkbox]";
    let Ok(list) = doc.query_selector_all(selector) else { return };
    for i in 0..list.length() {
        if let Some(el) = list.item(i).and_then(|n| {
            use wasm_bindgen::JsCast;
            n.dyn_into::<Element>().ok()
        }) {
            dispatch(&el);
        }
    }
}
