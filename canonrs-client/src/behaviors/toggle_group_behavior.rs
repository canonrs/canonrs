//! ToggleGroup Behavior - coordena seleção e emite rs-change com valor do grupo
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
fn collect_values(root: &Element) -> String {
    let mut values = vec![];
    if let Ok(items) = root.query_selector_all("[data-rs-toggle][data-rs-state='on']") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(el) = node.dyn_into::<Element>() {
                    let v = el.get_attribute("data-rs-value")
                        .or_else(|| el.text_content())
                        .unwrap_or_default();
                    if !v.is_empty() { values.push(v); }
                }
            }
        }
    }
    values.join(",")
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-toggle-group", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-toggle-group-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-toggle-group-attached", "1").ok();

        let multiple = root.get_attribute("data-rs-multiple").as_deref() == Some("true");

        let root_clone = root.clone();
        let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
            let target = match e.target().and_then(|t| t.dyn_into::<Element>().ok()) {
                Some(t) => t,
                None => return,
            };
            // só processar se veio de um toggle dentro do grupo
            let toggle = if target.has_attribute("data-rs-toggle") {
                Some(target)
            } else {
                target.closest("[data-rs-toggle]").ok().flatten()
            };

            if let Some(item) = toggle {
                if !multiple {
                    // desativar todos exceto o clicado
                    if let Ok(items) = root_clone.query_selector_all("[data-rs-toggle]") {
                        for i in 0..items.length() {
                            if let Some(node) = items.item(i) {
                                if let Ok(el) = node.dyn_into::<Element>() {
                                    if el != item {
                                        el.set_attribute("data-rs-state", "off").ok();
                                        el.set_attribute("data-rs-value", "off").ok();
                                    }
                                }
                            }
                        }
                    }
                }

                // coletar valor do grupo e emitir rs-change
                let value = collect_values(&root_clone);
                root_clone.set_attribute("data-rs-value", &value).ok();

                {
                    let init = web_sys::CustomEventInit::new();
                    init.set_bubbles(true);
                    init.set_detail(&wasm_bindgen::JsValue::from_str(&value));
                    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-change", &init) {
                        root_clone.dispatch_event(&event).ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
