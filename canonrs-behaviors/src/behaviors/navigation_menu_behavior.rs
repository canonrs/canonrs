#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
pub fn init_navigation_menu() {
    use leptos::prelude::*;
    use leptos::leptos_dom::helpers::window;
    use web_sys::{Document, HtmlElement, Node, NodeList, Element, EventTarget};
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let document: Document = window().document().expect("document");

        let triggers: NodeList = match document.query_selector_all("[data-nav-trigger]") {
            Ok(list) => list,
            Err(_) => return,
        };
        let length: u32 = triggers.length();

        for i in 0..length {
            let node: Node = match triggers.item(i) {
                Some(n) => n,
                None => continue,
            };

            let trigger_el: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
                Some(el) => el,
                None => continue,
            };

            let trigger_id: String = trigger_el.get_attribute("data-nav-trigger").unwrap_or_default();

            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let doc: Document = window().document().expect("document");

                let content: Element = match doc.query_selector(&format!("[data-nav-content='{}']", trigger_id)) {
                    Ok(Some(c)) => c,
                    _ => return,
                };

                let content_el: &HtmlElement = match content.dyn_ref::<HtmlElement>() {
                    Some(el) => el,
                    None => return,
                };

                if content_el.has_attribute("hidden") {
                    let _ = content_el.remove_attribute("hidden");
                } else {
                    let _ = content_el.set_attribute("hidden", "");
                }
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = trigger_el.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn init_navigation_menu() {}
