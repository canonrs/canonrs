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

pub fn closest_el(el: &Element, selector: &str) -> Option<Element> {
    el.closest(selector).ok().flatten()
}

/// Busca root de um componente pelo atributo + uid — resiste a re-render.
pub fn root_of(attr: &str, uid: &str) -> Option<Element> {
    let doc = web_sys::window().and_then(|w| w.document())?;
    doc.query_selector(&format!("[{}][data-rs-uid='{}']", attr, uid)).ok().flatten()
}

/// Todos os elementos que batem o selector dentro do root.
pub fn all(root: &Element, selector: &str) -> Vec<Element> {
    let Ok(list) = root.query_selector_all(selector) else { return vec![] };
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

/// Primeiro elemento que bate o selector dentro do root.
pub fn first(root: &Element, selector: &str) -> Option<Element> {
    root.query_selector(selector).ok().flatten()
}

/// Itera globalmente sobre todos os elementos que batem o selector.
/// Guard de is_connected em cada elemento.
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

/// Sobe a árvore procurando um elemento com o atributo dado.
pub fn closest_attr(el: &Element, attr: &str) -> Option<Element> {
    let mut current = Some(el.clone());
    while let Some(e) = current {
        if e.has_attribute(attr) { return Some(e); }
        current = e.parent_element();
    }
    None
}

/// Sobe a arvore verificando se algum ancestral tem o atributo dado.
pub fn has_ancestor_attr(el: &Element, attr: &str) -> bool {
    let mut current = el.parent_element();
    while let Some(e) = current {
        if e.has_attribute(attr) { return true; }
        current = e.parent_element();
    }
    false
}
