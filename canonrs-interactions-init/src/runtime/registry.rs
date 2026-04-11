//! Registry — mapa selector → init_fn
//! Tier S: zero if-chains, extensível, isolado

use web_sys::Element;

pub type InitFn = fn(Element);

static REGISTRY: &[(&str, InitFn)] = &[
    ("[data-rs-table]",              crate::table::init),
    ("[data-rs-tooltip]",            crate::tooltip::init),
    ("[data-rs-table-context]",      crate::table_row_sheet_preview::init),
];

pub fn dispatch(el: &Element) {
    for (selector, init_fn) in REGISTRY {
        if el.matches(selector).unwrap_or(false) {
            init_fn(el.clone());
            return;
        }
    }
}

pub fn scan_all() {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let selectors: Vec<&str> = REGISTRY.iter().map(|(s, _)| *s).collect();
    let combined = selectors.join(", ");
    let Ok(list) = doc.query_selector_all(&combined) else { return };
    for i in 0..list.length() {
        if let Some(el) = list.item(i).and_then(|n| {
            use wasm_bindgen::JsCast;
            n.dyn_into::<Element>().ok()
        }) {
            dispatch(&el);
        }
    }
}
