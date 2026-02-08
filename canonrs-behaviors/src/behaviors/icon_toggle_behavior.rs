#[cfg(feature = "hydrate")]
use leptos::web_sys::{EventTarget, Element, Window};
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
pub fn init_icon_toggle() {
    use leptos::prelude::*;
    use leptos::leptos_dom::helpers::window;
    use web_sys::{Document, HtmlElement, Node, NodeList};
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let document: Document = window().document().expect("document");

        let toggles: NodeList = match document.query_selector_all("[data-icon-toggle]") {
            Ok(list) => list,
            Err(_) => return,
        };
        let length: u32 = toggles.length();

        for i in 0..length {
            let node: Node = match toggles.item(i) {
                Some(n) => n,
                None => continue,
            };

            let toggle_el: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
                Some(el) => el,
                None => continue,
            };

            let toggle_clone: HtmlElement = toggle_el.clone();
            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let is_active: bool = toggle_clone.has_attribute("data-active");
                if is_active {
                    let _ = toggle_clone.remove_attribute("data-active");
                } else {
                    let _ = toggle_clone.set_attribute("data-active", "true");
                }
                let _ = window().alert_with_message("Icon toggle clicked!");
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = toggle_el.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn init_icon_toggle() {}
