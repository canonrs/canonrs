//! Query — helpers para navegação e busca no DOM
use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn all(root: &Element, selector: &str) -> Vec<Element> {
    let Ok(list) = root.query_selector_all(selector) else { return vec![] };
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

pub fn first(root: &Element, selector: &str) -> Option<Element> {
    root.query_selector(selector).ok().flatten()
}

pub fn closest_attr(el: &Element, attr: &str) -> Option<Element> {
    let mut current = Some(el.clone());
    while let Some(e) = current {
        if e.has_attribute(attr) { return Some(e); }
        current = e.parent_element();
    }
    None
}
