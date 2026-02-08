#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, HtmlElement, Node, NodeList, Element, Document};

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    use leptos::leptos_dom::helpers::window;
    use wasm_bindgen::JsCast;

    register_behavior("data-accordion", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        let accordion_id = id.to_string();
        let doc: Document = window().document().expect("document");

        let selector = format!("#{} [data-accordion-trigger]", accordion_id);

        let triggers: NodeList = match doc.query_selector_all(&selector) {
            Ok(list) => list,
            Err(_) => return Ok(()),
        };
        let length: u32 = triggers.length();

        for i in 0..length {
            let node: Node = match triggers.item(i) {
                Some(n) => n,
                None => continue,
            };

            let trigger: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
                Some(el) => el,
                None => continue,
            };

            let target_id: String = trigger.get_attribute("data-accordion-trigger").unwrap_or_default();
            let doc_clone: Document = doc.clone();

            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                if let Ok(Some(content)) = doc_clone.query_selector(&format!("[data-accordion-content='{}']", target_id)) {
                    let content_el: &HtmlElement = match content.dyn_ref::<HtmlElement>() {
                        Some(el) => el,
                        None => return,
                    };

                    if content_el.has_attribute("hidden") {
                        let _ = content_el.remove_attribute("hidden");
                    } else {
                        let _ = content_el.set_attribute("hidden", "");
                    }
                }
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = trigger.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
