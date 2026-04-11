//! Keyboard — helpers para navegação por teclado em listas

use wasm_bindgen::JsCast;
use web_sys::Element;

/// Foca o elemento em `index` dentro de `items` via seletor de trigger
pub fn focus_at(items: &[Element], index: usize, trigger_selector: &str) {
    if let Some(item) = items.get(index) {
        if let Ok(Some(trigger)) = item.query_selector(trigger_selector) {
            if let Ok(el) = trigger.dyn_into::<web_sys::HtmlElement>() {
                let _ = el.focus();
            }
        }
    }
}

/// Navega para o próximo item (ArrowDown)
pub fn focus_next(items: &[Element], current_pos: usize, trigger_selector: &str) {
    let next = (current_pos + 1).min(items.len().saturating_sub(1));
    focus_at(items, next, trigger_selector);
}

/// Navega para o item anterior (ArrowUp)
pub fn focus_prev(items: &[Element], current_pos: usize, trigger_selector: &str) {
    let prev = if current_pos == 0 { 0 } else { current_pos - 1 };
    focus_at(items, prev, trigger_selector);
}

/// Navega para o primeiro item (Home)
pub fn focus_first(items: &[Element], trigger_selector: &str) {
    focus_at(items, 0, trigger_selector);
}

/// Navega para o último item (End)
pub fn focus_last(items: &[Element], trigger_selector: &str) {
    if !items.is_empty() {
        focus_at(items, items.len() - 1, trigger_selector);
    }
}

/// Retorna a posição do item que contém o target
pub fn find_pos(items: &[Element], target: &Element) -> Option<usize> {
    items.iter().position(|el| el.contains(Some(target.as_ref())))
}
