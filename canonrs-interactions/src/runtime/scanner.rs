use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn query(selector: &str) -> Vec<Element> {
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return vec![],
    };
    let list = match doc.query_selector_all(selector) {
        Ok(l) => l,
        Err(_) => return vec![],
    };
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}
