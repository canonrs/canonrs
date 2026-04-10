//! Nav — keyboard navigation helpers for data interactions
use web_sys::Element;
use wasm_bindgen::JsCast;

pub fn get_items(root: &Element, selector: &str) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all(selector) else { return vec![] };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

pub fn focused_index(items: &[Element]) -> Option<usize> {
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element())?;
    items.iter().position(|el| *el == active)
}

pub fn focus_next(items: &[Element], current: usize) {
    let next = (current + 1).min(items.len() - 1);
    if let Ok(el) = items[next].clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el.focus();
    }
}

pub fn focus_prev(items: &[Element], current: usize) {
    let prev = if current == 0 { 0 } else { current - 1 };
    if let Ok(el) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el.focus();
    }
}

pub fn focus_first(items: &[Element]) {
    if let Some(el) = items.first() {
        if let Ok(h) = el.clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
    }
}

pub fn focus_last(items: &[Element]) {
    if let Some(el) = items.last() {
        if let Ok(h) = el.clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
    }
}
