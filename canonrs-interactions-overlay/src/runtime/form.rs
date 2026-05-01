//! Form helpers — native form integration for overlay components
use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn sync_hidden_input(root: &Element, value: &str) {
    let name = match root.get_attribute("data-rs-name") {
        Some(n) if !n.is_empty() => n,
        _ => return,
    };
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };
    if let Ok(Some(el)) = root.query_selector("input[data-rs-hidden]") {
        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
            input.set_value(value);
            return;
        }
    }
    if let Ok(el) = doc.create_element("input") {
        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
            let _ = input.set_attribute("type", "hidden");
            let _ = input.set_attribute("data-rs-hidden", "");
            let _ = input.set_attribute("name", &name);
            input.set_value(value);
            let _ = root.append_child(input.unchecked_ref());
        }
    }
}
