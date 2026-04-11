//! Query — helpers para navegação e busca no DOM

use wasm_bindgen::JsCast;
use web_sys::Element;

/// Retorna todos os filhos diretos que casam com o seletor
pub fn all(root: &Element, selector: &str) -> Vec<Element> {
    let Ok(list) = root.query_selector_all(selector) else { return vec![] };
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

/// Primeiro filho que casa com o seletor
pub fn first(root: &Element, selector: &str) -> Option<Element> {
    root.query_selector(selector).ok().flatten()
}

/// Sobe o DOM até encontrar elemento com o atributo
pub fn closest_attr(el: &Element, attr: &str) -> Option<Element> {
    let mut current = Some(el.clone());
    while let Some(e) = current {
        if e.has_attribute(attr) { return Some(e); }
        current = e.parent_element();
    }
    None
}

/// Sobe o DOM até encontrar elemento com data-rs-action=value
pub fn closest_action(el: &Element, action: &str) -> Option<Element> {
    let mut current = Some(el.clone());
    while let Some(e) = current {
        if e.get_attribute("data-rs-action").as_deref() == Some(action) { return Some(e); }
        current = e.parent_element();
    }
    None
}

/// Verifica se o elemento ou ancestral tem o atributo
pub fn has_ancestor_attr(el: &Element, attr: &str) -> bool {
    closest_attr(el, attr).is_some()
}
