//! Focus — trap, restore, focusable elements

use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn focusable_elements(content: &Element) -> Vec<web_sys::HtmlElement> {
    let sel = "a[href]:not([disabled]), button:not([disabled]), input:not([disabled]), \
               select:not([disabled]), textarea:not([disabled]), \
               [tabindex]:not([tabindex='-1'])";
    let mut els = Vec::new();
    if let Ok(nodes) = content.query_selector_all(sel) {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok()) {
                els.push(n);
            }
        }
    }
    els
}

pub fn focus_first(content: &Element) {
    let els = focusable_elements(content);
    if let Some(el) = els.first() {
        let _ = el.focus();
    } else if let Ok(html) = content.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = html.focus();
    }
}

pub fn active_element() -> Option<Element> {
    web_sys::window().and_then(|w| w.document())?.active_element()
}

/// Retorna true se o foco saiu do content (para detectar escape de focus trap)
pub fn focus_escaped(content: &Element) -> bool {
    let Some(active) = active_element() else { return true };
    !content.contains(Some(&active))
}

/// Trap: Tab / Shift+Tab dentro do content
/// Retorna true se o evento foi consumido
pub fn trap_tab(e: &web_sys::KeyboardEvent, content: &Element) -> bool {
    if e.key() != "Tab" { return false; }
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return false };
    let els = focusable_elements(content);
    if els.is_empty() { e.prevent_default(); return true; }

    let first   = els.first().unwrap();
    let last    = els.last().unwrap();
    let active  = doc.active_element();
    let first_el = first.clone().dyn_into::<Element>().ok();
    let last_el  = last.clone().dyn_into::<Element>().ok();

    if e.shift_key() {
        if active == first_el {
            e.prevent_default();
            let _ = last.focus();
            return true;
        }
    } else if active == last_el {
        e.prevent_default();
        let _ = first.focus();
        return true;
    }
    false
}
