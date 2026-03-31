#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, Element};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-combobox", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-combobox-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-combobox-attached", "1").ok();

        // abrir via focus/input no input interno
        let Ok(Some(input_el)) = root.query_selector("[data-rs-combobox-input]") else { return Ok(()); };
        let Ok(input) = input_el.dyn_into::<HtmlElement>() else { return Ok(()); };

        // open on focus
        let root_focus = root.clone();
        let cb_focus = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
root_focus.set_attribute("data-rs-state", "open").ok();
            root_focus.set_attribute("aria-expanded", "true").ok();
        }) as Box<dyn FnMut(_)>);
        root.add_event_listener_with_callback("focusin", cb_focus.as_ref().unchecked_ref()).ok();
        cb_focus.forget();

        // filter on input
        let root_input = root.clone();
        let cb_input = Closure::wrap(Box::new(move |e: web_sys::InputEvent| {
            let target = match e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                Some(el) => el,
                None => return,
            };
            let query = target.value().to_lowercase();
            if let Ok(items) = root_input.query_selector_all("[data-rs-combobox-item]") {
                for i in 0..items.length() {
                    if let Some(node) = items.item(i) {
                        if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                            let text = el.text_content().unwrap_or_default().to_lowercase();
                            if query.is_empty() || text.contains(&query) {
                                el.remove_attribute("data-rs-visible").ok();
                            } else {
                                el.set_attribute("data-rs-visible", "false").ok();
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        input.add_event_listener_with_callback("input", cb_input.as_ref().unchecked_ref()).ok();
        cb_input.forget();

        // close on blur
        let root_blur = root.clone();
        let cb_blur = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
            let root_clone = root_blur.clone();
            let closure = Closure::once(Box::new(move || {
                let active = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.active_element());
                let inside = active.and_then(|el| el.closest("[data-rs-combobox]").ok().flatten()).is_some();
                if !inside {
                    root_clone.set_attribute("data-rs-state", "closed").ok();
                    root_clone.set_attribute("aria-expanded", "false").ok();
                }
            }) as Box<dyn FnOnce()>);
            web_sys::window().unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 150).ok();
            closure.forget();
        }) as Box<dyn FnMut(_)>);
        root.add_event_listener_with_callback("focusout", cb_blur.as_ref().unchecked_ref()).ok();
        cb_blur.forget();

        // fechar ao clicar fora
        let root_outside = root.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if let Some(target) = e.target() {
                if let Some(el) = target.dyn_ref::<Element>() {
                    let inside_list = el.closest("[data-rs-combobox-list]").ok().flatten().is_some();
                    let inside_root = el.closest("[data-rs-combobox]").ok().flatten().is_some();
                    if !inside_list && !inside_root {
                        root_outside.set_attribute("data-rs-state", "closed").ok();
                        root_outside.set_attribute("aria-expanded", "false").ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap()
            .add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).ok();
        cb_outside.forget();

        // navegação ↑ ↓ + Enter + ESC
        let root_key = root.clone();
        let cb_key = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            let is_open = root_key.get_attribute("data-rs-state").as_deref() == Some("open");

            match key.as_str() {
                "Escape" => {
                    root_key.set_attribute("data-rs-state", "closed").ok();
                    root_key.set_attribute("aria-expanded", "false").ok();
                }
                "ArrowDown" | "ArrowUp" => {
                    e.prevent_default();
                    if !is_open {
                        root_key.set_attribute("data-rs-state", "open").ok();
                        root_key.set_attribute("aria-expanded", "true").ok();
                    }
                    let Ok(all_nodes) = root_key.query_selector_all("[data-rs-combobox-item]") else { return };
                    let mut items: Vec<web_sys::HtmlElement> = vec![];
                    let mut focused_idx: Option<usize> = None;

                    for i in 0..all_nodes.length() {
                        if let Some(node) = all_nodes.item(i) {
                            if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                                if el.get_attribute("data-rs-visible").as_deref() == Some("false") {
                                    continue;
                                }
                                if el.get_attribute("data-rs-focused").as_deref() == Some("true") {
                                    focused_idx = Some(items.len());
                                }
                                items.push(el);
                            }
                        }
                    }

                    let len = items.len();
                    if len == 0 { return }

                    // limpar focused
                    for el in &items {
                        el.remove_attribute("data-rs-focused").ok();
                    }

                    let next_idx = match (key.as_str(), focused_idx) {
                        ("ArrowDown", None) => 0,
                        ("ArrowDown", Some(i)) => (i + 1).min(len - 1),
                        ("ArrowUp", None) => len - 1,
                        ("ArrowUp", Some(i)) => if i == 0 { 0 } else { i - 1 },
                        _ => 0,
                    };

                    items[next_idx].set_attribute("data-rs-focused", "true").ok();
                    items[next_idx].scroll_into_view();
                }
                "Enter" => {
                    if !is_open { return }
                    e.prevent_default();
                    let Ok(items) = root_key.query_selector_all("[data-rs-combobox-item]") else { return };
                    for i in 0..items.length() {
                        if let Some(node) = items.item(i) {
                            if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                                if el.get_attribute("data-rs-focused").as_deref() == Some("true") {
                                    let value = el.get_attribute("data-rs-value").unwrap_or_default();
                                    let text = el.text_content().unwrap_or_default();
                                    // update input
                                    if let Ok(Some(input_el)) = root_key.query_selector("[data-rs-combobox-input]") {
                                        if let Ok(input) = input_el.dyn_into::<web_sys::HtmlInputElement>() {
                                            input.set_value(&text);
                                        }
                                    }
                                    root_key.set_attribute("data-rs-value", &value).ok();
                                    root_key.set_attribute("data-rs-state", "closed").ok();
                                    root_key.set_attribute("aria-expanded", "false").ok();
                                    el.remove_attribute("data-rs-focused").ok();
                                    if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
                                        root_key.dispatch_event(&event).ok();
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        input.add_event_listener_with_callback("keydown", cb_key.as_ref().unchecked_ref()).ok();
        cb_key.forget();

        // item selection + rs-change
        if let Ok(items) = root.query_selector_all("[data-rs-combobox-item]") {
            for i in 0..items.length() {
                let Some(node) = items.item(i) else { continue };
                let Ok(item) = node.dyn_into::<web_sys::Element>() else { continue };
                let root_item = root.clone();
                let cb_item = Closure::wrap(Box::new(move |e: MouseEvent| {
                    e.stop_propagation();
                    let target = match e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                        Some(el) => el,
                        None => return,
                    };
                    let value = target.get_attribute("data-rs-value").unwrap_or_default();
                    let text  = target.text_content().unwrap_or_default();

                    // clear all
                    if let Ok(all) = root_item.query_selector_all("[data-rs-combobox-item]") {
                        for j in 0..all.length() {
                            if let Some(n) = all.item(j) {
                                if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                                    el.set_attribute("data-rs-state", "unselected").ok();
                                    el.set_attribute("aria-selected", "false").ok();
                                }
                            }
                        }
                    }
                    target.set_attribute("data-rs-state", "selected").ok();
                    target.set_attribute("aria-selected", "true").ok();

                    // update input value após seleção
                    if let Ok(Some(input_el)) = root_item.query_selector("[data-rs-combobox-input]") {
                        if let Ok(input) = input_el.dyn_into::<web_sys::HtmlInputElement>() {
                            input.set_value(&text);
                        }
                    }

                    root_item.set_attribute("data-rs-value", &value).ok();
                    root_item.set_attribute("data-rs-state", "closed").ok();
                    root_item.set_attribute("aria-expanded", "false").ok();

                    // dispatch rs-change
                    if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
                        root_item.dispatch_event(&event).ok();
                    }
                }) as Box<dyn FnMut(_)>);
                item.add_event_listener_with_callback("click", cb_item.as_ref().unchecked_ref()).ok();
                cb_item.forget();
            }
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
