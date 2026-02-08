#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Node, NodeList, Element, EventTarget};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = clipboard)]
    #[wasm_bindgen(thread_local_v2)]
    static CLIPBOARD: Clipboard;

    type Clipboard;

    #[wasm_bindgen(method, js_name = writeText)]
    fn write_text(this: &Clipboard, text: &str) -> js_sys::Promise;
}

#[cfg(feature = "hydrate")]
pub fn init_copy_button_behavior() {
    use leptos::leptos_dom::helpers::window;
    use wasm_bindgen::JsCast;

    let document: Document = window().document().expect("document");

    let buttons: NodeList = document.query_selector_all("[data-copy-button]").expect("query copy buttons");
    let length: u32 = buttons.length();

    for i in 0..length {
        let node: Node = match buttons.item(i) {
            Some(n) => n,
            None => continue,
        };

        let button: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
            Some(el) => el,
            None => continue,
        };

        let target_id: String = match button.get_attribute("data-copy-target") {
            Some(id) => id,
            None => continue,
        };

        let doc: Document = document.clone();
        let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
            let target_element: Element = match doc.get_element_by_id(&target_id) {
                Some(el) => el,
                None => return,
            };

            let html_el: &HtmlElement = match target_element.dyn_ref::<HtmlElement>() {
                Some(h) => h,
                None => return,
            };

            let text: String = html_el.inner_text();

            CLIPBOARD.with(|clip| {
                let _ = clip.write_text(&text);
            });
        }) as Box<dyn FnMut(_)>);

        let target: &EventTarget = button.as_ref();
        let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn init_copy_button_behavior() {}
