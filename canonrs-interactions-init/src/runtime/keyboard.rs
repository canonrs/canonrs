//! Keyboard — navegação por teclado padronizada
//!
//! Lições dos componentes nav_item, menu, navigation_menu:
//! - <a> (Link): keydown em cada item, sem .focus() — browser não move foco com seta
//! - <button> (Button): keydown no root, com .focus() — evita disparo duplo

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use std::rc::Rc;
use std::cell::Cell;
use crate::runtime::{state, query};

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ElementType {
    /// <a> — keydown em cada item, sem .focus()
    Link,
    /// <button> — keydown no root, com .focus()
    Button,
}

pub struct NavConfig {
    pub orientation: Orientation,
    pub element_type: ElementType,
    pub focus_state: &'static str,
    pub wrap: bool,
}

impl Default for NavConfig {
    fn default() -> Self {
        Self {
            orientation: Orientation::Vertical,
            element_type: ElementType::Button,
            focus_state: "focused",
            wrap: false,
        }
    }
}

fn move_focus(items: &[Element], idx: usize, element_type: ElementType) {
    if element_type == ElementType::Button {
        if let Some(el) = items.get(idx) {
            if let Ok(h) = el.clone().dyn_into::<web_sys::HtmlElement>() {
                let _ = h.focus();
            }
        }
    }
}

fn navigate(
    items: &[Element],
    _current: Option<usize>,
    next_idx: usize,
    focus_state: &str,
    element_type: ElementType,
) {
    for el in items { state::remove_state(el, focus_state); }
    if let Some(el) = items.get(next_idx) {
        state::add_state(el, focus_state);
        move_focus(items, next_idx, element_type);
    }
}

/// Inicializa navegação por teclado.
/// Retorna Rc<Cell<Option<usize>>> para sincronizar com click handler.
pub fn init_nav(
    root: &Element,
    item_selector: &'static str,
    config: NavConfig,
    on_enter: Option<Box<dyn Fn(usize, &[Element]) + 'static>>,
    on_escape: Option<Box<dyn Fn() + 'static>>,
) -> Rc<Cell<Option<usize>>> {
    let current_idx: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));

    let root_kb = root.clone();
    let idx_kb = current_idx.clone();
    let focus_state = config.focus_state;
    let wrap = config.wrap;
    let orientation = config.orientation;
    let element_type = config.element_type;

    let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
        let items: Vec<Element> = query::all(&root_kb, item_selector)
            .into_iter()
            .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
            .collect();
        if items.is_empty() { return; }

        let prev_key = if orientation == Orientation::Horizontal { "ArrowLeft" } else { "ArrowUp" };
        let next_key = if orientation == Orientation::Horizontal { "ArrowRight" } else { "ArrowDown" };
        let current = idx_kb.get();
        let len = items.len();

        match e.key().as_str() {
            k if k == next_key => {
                e.prevent_default();
                let next = match (current, wrap) {
                    (Some(i), false) => (i + 1).min(len - 1),
                    (Some(i), true)  => (i + 1) % len,
                    (None, _)        => 0,
                };
                navigate(&items, current, next, focus_state, element_type);
                idx_kb.set(Some(next));
            }
            "ArrowDown" if orientation == Orientation::Horizontal => {
                e.prevent_default();
                if let Some(idx) = current {
                    if let Some(ref cb) = on_enter { cb(idx, &items); }
                }
            }
            k if k == prev_key => {
                e.prevent_default();
                let prev = match (current, wrap) {
                    (Some(0), false) => 0,
                    (Some(0), true)  => len - 1,
                    (Some(i), _)     => i - 1,
                    (None, _)        => 0,
                };
                navigate(&items, current, prev, focus_state, element_type);
                idx_kb.set(Some(prev));
            }
            "Home" => {
                e.prevent_default();
                navigate(&items, current, 0, focus_state, element_type);
                idx_kb.set(Some(0));
            }
            "End" => {
                e.prevent_default();
                navigate(&items, current, len - 1, focus_state, element_type);
                idx_kb.set(Some(len - 1));
            }
            "Enter" | " " => {
                e.prevent_default();
                if let Some(idx) = current {
                    if let Some(ref cb) = on_enter { cb(idx, &items); }
                }
            }
            "Escape" => {
                if let Some(ref cb) = on_escape {
                    e.prevent_default();
                    if let Some(prev) = current { state::remove_state(&items[prev], focus_state); }
                    idx_kb.set(None);
                    cb();
                }
            }
            _ => {}
        }
    });

    match element_type {
        ElementType::Button => {
            let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        ElementType::Link => {
            for item in query::all(root, item_selector) {
                let _ = item.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            }
        }
    }
    cb.forget();

    current_idx
}

/// Helper — acha idx de um elemento na lista por uid
pub fn find_idx_by_uid(items: &[Element], target: &Element) -> Option<usize> {
    let uid = target.get_attribute("data-rs-uid")?;
    items.iter().position(|el| el.get_attribute("data-rs-uid").as_deref() == Some(&uid))
}
