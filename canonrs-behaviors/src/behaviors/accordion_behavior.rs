#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, HtmlElement, Node, NodeList, Element, Document, MutationObserver, MutationObserverInit};

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

        // Função para inicializar triggers de um accordion
        fn init_accordion_triggers(doc: &Document, accordion_id: &str) {
            let selector = format!("#{} [data-accordion-trigger]", accordion_id);
            
            let triggers: NodeList = match doc.query_selector_all(&selector) {
                Ok(list) => list,
                Err(_) => return,
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

                // Skip se já tem listener
                if trigger.get_attribute("data-accordion-initialized").is_some() {
                    continue;
                }
                
                let _ = trigger.set_attribute("data-accordion-initialized", "true");

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
                            
                            // Re-inicializar accordions aninhados que ficaram visíveis
                            if let Ok(nested_accordions) = content_el.query_selector_all("[data-accordion]") {
                                for j in 0..nested_accordions.length() {
                                    if let Some(nested_node) = nested_accordions.item(j) {
                                        if let Some(nested_el) = nested_node.dyn_ref::<Element>() {
                                            if let Some(nested_id) = nested_el.get_attribute("id") {
                                                init_accordion_triggers(&doc_clone, &nested_id);
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            let _ = content_el.set_attribute("hidden", "");
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let target: &EventTarget = trigger.as_ref();
                let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }

        // Inicializar accordion principal
        init_accordion_triggers(&doc, &accordion_id);

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
