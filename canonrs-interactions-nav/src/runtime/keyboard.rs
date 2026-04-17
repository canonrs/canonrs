//! Keyboard — navegação por teclado
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use std::rc::Rc;
use std::cell::Cell;
use crate::runtime::{state, query};

// ── Low-level focus helpers (compatibilidade) ────────────────

pub fn focus_at(items: &[Element], index: usize, trigger_selector: &str) {
    if let Some(item) = items.get(index) {
        if let Ok(Some(trigger)) = item.query_selector(trigger_selector) {
            if let Ok(el) = trigger.dyn_into::<web_sys::HtmlElement>() {
                let _ = el.focus();
            }
        }
    }
}

pub fn focus_next(items: &[Element], current_pos: usize, trigger_selector: &str) {
    let next = (current_pos + 1).min(items.len().saturating_sub(1));
    focus_at(items, next, trigger_selector);
}

pub fn focus_prev(items: &[Element], current_pos: usize, trigger_selector: &str) {
    let prev = if current_pos == 0 { 0 } else { current_pos - 1 };
    focus_at(items, prev, trigger_selector);
}

pub fn focus_first(items: &[Element], trigger_selector: &str) {
    focus_at(items, 0, trigger_selector);
}

pub fn focus_last(items: &[Element], trigger_selector: &str) {
    if !items.is_empty() {
        focus_at(items, items.len() - 1, trigger_selector);
    }
}

pub fn find_pos(items: &[Element], target: &Element) -> Option<usize> {
    items.iter().position(|el| el.contains(Some(target.as_ref())))
}

// ── High-level nav orchestration ─────────────────────────────

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
    next_idx: usize,
    focus_state: &str,
    element_type: ElementType,
) {
    for el in items { state::remove(el, focus_state); }
    if let Some(el) = items.get(next_idx) {
        state::add(el, focus_state);
        move_focus(items, next_idx, element_type);
    }
}

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
                navigate(&items, next, focus_state, element_type);
                idx_kb.set(Some(next));
            }
            k if k == prev_key => {
                e.prevent_default();
                let prev = match (current, wrap) {
                    (Some(0), false) => 0,
                    (Some(0), true)  => len - 1,
                    (Some(i), _)     => i - 1,
                    (None, _)        => 0,
                };
                navigate(&items, prev, focus_state, element_type);
                idx_kb.set(Some(prev));
            }
            "Home" => {
                e.prevent_default();
                navigate(&items, 0, focus_state, element_type);
                idx_kb.set(Some(0));
            }
            "End" => {
                e.prevent_default();
                navigate(&items, len - 1, focus_state, element_type);
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
                    if let Some(prev) = current { state::remove(&items[prev], focus_state); }
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

pub fn find_idx_by_uid(items: &[Element], target: &Element) -> Option<usize> {
    let uid = target.get_attribute("data-rs-uid")?;
    items.iter().position(|el| el.get_attribute("data-rs-uid").as_deref() == Some(&uid))
}
