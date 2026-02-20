#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, Document};
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    use leptos::leptos_dom::helpers::window;

    register_behavior("data-accordion", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        let doc: Document = window().document()
            .ok_or_else(|| canonrs_shared::BehaviorError::JsError { message: "no document".into() })?;

        let accordion = doc.get_element_by_id(id)
            .ok_or_else(|| canonrs_shared::BehaviorError::ElementNotFound { selector: id.into() })?;

        if accordion.get_attribute("data-accordion-attached").as_deref() == Some("1") {
            return Ok(());
        }
        accordion.set_attribute("data-accordion-attached", "1").ok();

        let is_single = accordion.get_attribute("data-selection").as_deref() != Some("multiple");
        let accordion_id = id.to_string();

        let triggers = accordion.query_selector_all("[data-accordion-trigger]")
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query triggers".into() })?;

        for i in 0..triggers.length() {
            let node = match triggers.item(i) { Some(n) => n, None => continue };
            let trigger = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            if trigger.get_attribute("data-accordion-initialized").is_some() { continue; }
            trigger.set_attribute("data-accordion-initialized", "1").ok();

            let doc_clone = doc.clone();
            let accordion_id_clone = accordion_id.clone();

            let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let target = match e.current_target()
                    .and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                    Some(el) => el,
                    None => return,
                };

                let controls_id = match target.get_attribute("aria-controls") {
                    Some(id) if !id.is_empty() => id,
                    _ => return,
                };

                let content = match doc_clone.get_element_by_id(&controls_id)
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
                    Some(el) => el,
                    None => return,
                };

                let is_open = !content.has_attribute("hidden");

                // Fechar outros se single
                if is_single && !is_open {
                    if let Ok(accordion_el) = doc_clone.get_element_by_id(&accordion_id_clone)
                        .ok_or(())
                    {
                        if let Ok(all_contents) = accordion_el.query_selector_all("[data-accordion-content]") {
                            for j in 0..all_contents.length() {
                                if let Some(node) = all_contents.item(j) {
                                    if let Ok(el) = node.dyn_into::<HtmlElement>() {
                                        el.set_attribute("hidden", "").ok();
                                        el.set_attribute("aria-hidden", "true").ok();
                                    }
                                }
                            }
                        }
                        if let Ok(all_triggers) = accordion_el.query_selector_all("[data-accordion-trigger]") {
                            for j in 0..all_triggers.length() {
                                if let Some(node) = all_triggers.item(j) {
                                    if let Ok(el) = node.dyn_into::<HtmlElement>() {
                                        el.set_attribute("aria-expanded", "false").ok();
                                        if let Some(item) = el.closest("[data-accordion-item]").ok().flatten() {
                                            item.set_attribute("data-state", "closed").ok();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Toggle atual
                if is_open {
                    content.set_attribute("hidden", "").ok();
                    content.set_attribute("aria-hidden", "true").ok();
                    target.set_attribute("aria-expanded", "false").ok();
                    if let Some(item) = target.closest("[data-accordion-item]").ok().flatten() {
                        item.set_attribute("data-state", "closed").ok();
                    }
                } else {
                    content.remove_attribute("hidden").ok();
                    content.set_attribute("aria-hidden", "false").ok();
                    target.set_attribute("aria-expanded", "true").ok();
                    if let Some(item) = target.closest("[data-accordion-item]").ok().flatten() {
                        item.set_attribute("data-state", "open").ok();
                    }
                }
            }) as Box<dyn FnMut(_)>);

            trigger.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
