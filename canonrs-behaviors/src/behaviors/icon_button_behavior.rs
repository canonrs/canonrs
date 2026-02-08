#[cfg(feature = "hydrate")]
use leptos::web_sys::{EventTarget, Window, Element};
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
pub fn init_icon_button() {
    use leptos::prelude::*;
    use leptos::leptos_dom::helpers::window;
    use web_sys::{Document, HtmlElement, Node, NodeList};
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let document: Document = window().document().expect("document");

        let buttons: NodeList = match document.query_selector_all("[data-icon-button]") {
            Ok(list) => list,
            Err(_) => return,
        };
        let length: u32 = buttons.length();

        for i in 0..length {
            let node: Node = match buttons.item(i) {
                Some(n) => n,
                None => continue,
            };

            let btn_el: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
                Some(el) => el,
                None => continue,
            };

            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let _ = window().alert_with_message("Icon button clicked!");
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = btn_el.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn init_icon_button() {}
