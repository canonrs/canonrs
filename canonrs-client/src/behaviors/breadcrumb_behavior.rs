use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event, HtmlElement, NodeList};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn register() {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    
    if let Ok(details_list) = document.query_selector_all("[data-breadcrumb-collapse]") {
        for i in 0..details_list.length() {
            if let Some(node) = details_list.get(i) {
                let details_el = node.dyn_into::<HtmlElement>().unwrap();
                let details_clone = details_el.clone();
                
                let closure = Closure::wrap(Box::new(move |e: Event| {
                    if let Some(target) = e.target() {
                        if let Ok(target_el) = target.dyn_into::<Element>() {
                            if !details_clone.contains(Some(&target_el)) {
                                let _ = details_clone.remove_attribute("open");
                            }
                        }
                    }
                }) as Box<dyn FnMut(Event)>);
                
                let _ = document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }
    }
}
