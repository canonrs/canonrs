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


/// Handler completo de navegação por teclado
/// item_selector: seletor dos itens navegáveis
/// on_escape: callback opcional chamado no Esc
pub fn init_navigation(
    root: &web_sys::Element,
    item_selector: &'static str,
    on_escape: Option<Box<dyn Fn() + 'static>>,
) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use crate::runtime::{state, query};

    let root_kb = root.clone();
    let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
        let items: Vec<web_sys::Element> = query::all(&root_kb, item_selector)
            .into_iter()
            .filter(|el| !el.get_attribute("data-rs-state").map(|s| s.contains("hidden")).unwrap_or(false))
            .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
            .collect();

        if items.is_empty() { return; }

        let current = items.iter().position(|el| {
            el.get_attribute("data-rs-state").map(|s| s.contains("active")).unwrap_or(false)
        });

        match e.key().as_str() {
            "ArrowDown" => {
                e.prevent_default();
                let next = current.map(|i| (i + 1).min(items.len() - 1)).unwrap_or(0);
                for item in &items {
                    state::remove_state(item, "active");
                    state::remove_state(item, "focus");
                    state::remove_state(item, "hover");
                    let _ = item.set_attribute("aria-selected", "false");
                }
                state::add_state(&items[next], "active");
                let _ = items[next].set_attribute("aria-selected", "true");
            }
            "ArrowUp" => {
                e.prevent_default();
                let prev_idx = current.map(|i| if i == 0 { 0 } else { i - 1 }).unwrap_or(0);
                for item in &items {
                    state::remove_state(item, "active");
                    state::remove_state(item, "focus");
                    state::remove_state(item, "hover");
                    let _ = item.set_attribute("aria-selected", "false");
                }
                state::add_state(&items[prev_idx], "active");
                let _ = items[prev_idx].set_attribute("aria-selected", "true");
            }
            "Home" => {
                e.prevent_default();
                if let Some(prev) = current { state::remove_state(&items[prev], "active"); }
                state::add_state(&items[0], "active");
            }
            "End" => {
                e.prevent_default();
                if let Some(prev) = current { state::remove_state(&items[prev], "active"); }
                state::add_state(&items[items.len() - 1], "active");
            }
            "Enter" => {
                e.prevent_default();
                if let Some(idx) = current {
                    // remover active de todos antes de click
                    for item in &items {
                        state::remove_state(item, "active");
                    }
                    let _ = items[idx].clone().dyn_into::<web_sys::HtmlElement>().map(|el| el.click());
                }
            }
            "Escape" => {
                e.prevent_default();
                if let Some(prev) = current { state::remove_state(&items[prev], "active"); }
                if let Some(ref cb) = on_escape { cb(); }
            }
            _ => {}
        }
    });
    let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
    cb.forget();
}
