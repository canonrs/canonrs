#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
pub fn init_sidebar() {
    use leptos::prelude::*;
    use leptos::leptos_dom::helpers::window;
    use web_sys::{Document, HtmlElement, Node, NodeList, Element, EventTarget};
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let document: Document = window().document().expect("document");

        let toggles: NodeList = match document.query_selector_all("[data-sidebar-toggle]") {
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

            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let doc: Document = window().document().expect("document");

                let sidebar: Element = match doc.query_selector("[data-sidebar]") {
                    Ok(Some(s)) => s,
                    _ => return,
                };

                let sidebar_el: &HtmlElement = match sidebar.dyn_ref::<HtmlElement>() {
                    Some(el) => el,
                    None => return,
                };

                if sidebar_el.has_attribute("data-collapsed") {
                    let _ = sidebar_el.remove_attribute("data-collapsed");
                } else {
                    let _ = sidebar_el.set_attribute("data-collapsed", "true");
                }
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = toggle_el.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn init_sidebar() {}
